use std::process::Command;
use std::sync::atomic::{AtomicU32, Ordering};
use std::sync::{Arc, Mutex};
use std::thread;
use std::time::Duration;
use xkbcommon::xkb;

/// Cada cuánto se consulta `niri msg -j keyboard-layouts` para detectar
/// un cambio de distribución. niri no empuja este evento, así que se
/// hace polling; 200ms es imperceptible para el usuario.
const POLL_INTERVAL: Duration = Duration::from_millis(200);

/// xkb::MOD_INVALID: valor que devuelve `mod_get_index` cuando el
/// modificador no existe en el keymap activo (ej. AltGr en un layout
/// sin nivel 3). Usarlo como cantidad de desplazamiento de bits
/// (`1 << índice`) provocaría un shift-overflow, así que cualquier
/// máscara calculada con este índice se descarta.
const MOD_INVALID: u32 = 0xffffffff;

/// Traduce keycodes evdev crudos al símbolo real de la distribución
/// activa (incluyendo cirílico), usando xkbcommon en vez de los nombres
/// KEY_* fijos que evdev asigna según la posición física US-QWERTY.
pub struct XkbTranslator {
    state: Mutex<xkb::State>,
    current_group: Arc<AtomicU32>,
    // Índices de modificador resueltos una sola vez al crear el keymap,
    // en vez de repetir el lookup por nombre en cada pulsación.
    idx_shift: u32,
    idx_altgr: u32,
    idx_caps: u32,
}

// xkb::State envuelve un puntero C (*mut xkb_state) que no implementa Send
// por defecto. Es seguro moverlo entre hilos porque todo acceso pasa por
// el Mutex de arriba: nunca hay dos hilos tocando el puntero a la vez, y
// libxkbcommon no guarda estado thread-local que dependa del hilo de origen.
unsafe impl Send for XkbTranslator {}
unsafe impl Sync for XkbTranslator {}

impl XkbTranslator {
    /// Crea el traductor a partir del layout X11 configurado en el sistema
    /// (ej. "us,ru" con `grp:caps_toggle`, como reporta `localectl`).
    pub fn new() -> Option<Arc<Self>> {
        let (layout, variant, options) = leer_layout_sistema()?;

        let context = xkb::Context::new(xkb::CONTEXT_NO_FLAGS);
        let keymap = xkb::Keymap::new_from_names(
            &context,
            "",
            "",
            &layout,
            &variant,
            Some(options),
            xkb::KEYMAP_COMPILE_NO_FLAGS,
        )?;
        let idx_shift = keymap.mod_get_index(xkb::MOD_NAME_SHIFT);
        let idx_altgr = keymap.mod_get_index("Mod5");
        let idx_caps = keymap.mod_get_index(xkb::MOD_NAME_CAPS);

        let state = xkb::State::new(&keymap);

        let translator = Arc::new(Self {
            state: Mutex::new(state),
            current_group: Arc::new(AtomicU32::new(0)),
            idx_shift,
            idx_altgr,
            idx_caps,
        });

        translator.iniciar_polling_layout();
        Some(translator)
    }

    /// Máscara de un modificador dado su índice ya resuelto, o 0 si el
    /// modificador no existe en este keymap (evita el shift-overflow de
    /// `1 << MOD_INVALID`).
    fn mascara(indice: u32) -> u32 {
        if indice == MOD_INVALID {
            0
        } else {
            1 << indice
        }
    }

    /// Convierte un keycode evdev (ej. 30 para KEY_A) al símbolo actual,
    /// respetando la distribución y el estado de Shift/CapsLock/AltGr.
    /// Devuelve `None` si la tecla no produce texto (Backspace, F1,
    /// flechas...), evitando así una lista aparte de "teclas no imprimibles"
    /// que habría que mantener sincronizada a mano.
    pub fn traducir(&self, evdev_code: u32, shift: bool, altgr: bool, caps: bool) -> Option<String> {
        let mut state = self.state.lock().unwrap();

        // evdev usa keycode físico; xkb espera keycode = evdev + 8 (offset X11 histórico).
        let keycode = xkb::Keycode::new(evdev_code + 8);

        let mut mods_depressed = 0;
        if shift {
            mods_depressed |= Self::mascara(self.idx_shift);
        }
        if altgr {
            mods_depressed |= Self::mascara(self.idx_altgr);
        }
        let mods_locked = if caps { Self::mascara(self.idx_caps) } else { 0 };

        state.update_mask(
            mods_depressed,
            0,
            mods_locked,
            0,
            0,
            self.current_group.load(Ordering::Relaxed),
        );

        let utf8 = state.key_get_utf8(keycode);

        // Teclas como Enter o Tab producen un carácter de control real
        // (ej. "\r", "\t"), no una cadena vacía — key_get_utf8() no las
        // filtra por sí solo. Cualquier resultado compuesto solo por
        // caracteres de control (< 0x20, o DEL 0x7F) se descarta para
        // que esas teclas caigan al nombre físico (KEY_ENTER, KEY_TAB...).
        let es_imprimible = !utf8.is_empty() && utf8.chars().any(|c| c != '\u{7f}' && c as u32 >= 0x20);

        if es_imprimible {
            Some(utf8)
        } else {
            None
        }
    }

    /// Consulta periódicamente `niri msg -j keyboard-layouts` para
    /// detectar cambios de distribución. Si el primer intento falla
    /// (compositor distinto a niri, o `niri` no está en PATH), no hay
    /// forma de saber el grupo activo por este medio: se desiste del
    /// todo en vez de reintentar 5 veces por segundo para siempre.
    fn iniciar_polling_layout(self: &Arc<Self>) {
        let current_group = Arc::clone(&self.current_group);
        thread::spawn(move || {
            let Some(idx) = leer_grupo_activo_niri() else {
                return;
            };
            current_group.store(idx, Ordering::Relaxed);

            loop {
                thread::sleep(POLL_INTERVAL);
                if let Some(idx) = leer_grupo_activo_niri() {
                    current_group.store(idx, Ordering::Relaxed);
                }
            }
        });
    }
}

/// Lee el layout X11 configurado (ej. "us,ru") y sus opciones (ej.
/// "grp:caps_toggle") desde `localectl status`.
fn leer_layout_sistema() -> Option<(String, String, String)> {
    let salida = Command::new("localectl").arg("status").output().ok()?;
    let texto = String::from_utf8_lossy(&salida.stdout);

    let mut layout = String::new();
    let mut variant = String::new();
    let mut options = String::new();

    for linea in texto.lines() {
        let linea = linea.trim();
        if let Some(v) = linea.strip_prefix("X11 Layout:") {
            layout = v.trim().to_string();
        } else if let Some(v) = linea.strip_prefix("X11 Variant:") {
            variant = v.trim().to_string();
        } else if let Some(v) = linea.strip_prefix("X11 Options:") {
            options = v.trim().to_string();
        }
    }

    if layout.is_empty() {
        return None;
    }
    Some((layout, variant, options))
}

/// Consulta el índice de distribución activa vía `niri msg -j keyboard-layouts`.
fn leer_grupo_activo_niri() -> Option<u32> {
    let salida = Command::new("niri")
        .args(["msg", "-j", "keyboard-layouts"])
        .output()
        .ok()?;
    let json: serde_json::Value = serde_json::from_slice(&salida.stdout).ok()?;
    json.get("current_idx")?.as_u64().map(|v| v as u32)
}
