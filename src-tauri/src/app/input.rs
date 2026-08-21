use crate::app::xkb_translator::XkbTranslator;
use evdev::{Device, EventSummary, KeyCode, LedCode};
use std::collections::HashSet;
use std::sync::{Arc, Mutex};
use std::thread;
use tauri::{AppHandle, Emitter};

/// Separador interno entre partes de una combinación (ej. Super, Ctrl,
/// tecla). Un carácter de control (Unit Separator, U+001F) que ningún
/// símbolo real de teclado puede producir, a diferencia de "+" que sí
/// es una tecla legítima (Shift+=) y corrompería el parseo en el frontend.
const SEPARADOR_COMBO: char = '\u{1F}';

fn es_teclado(dev: &Device) -> bool {
    dev.supported_keys()
        .map(|keys| keys.contains(KeyCode::KEY_A))
        .unwrap_or(false)
}

/// Lee el estado real del LED de Bloq Mayús del dispositivo al arrancar,
/// en vez de asumir que siempre está apagado.
fn caps_lock_inicial(dev: &Device) -> bool {
    dev.get_led_state()
        .map(|leds| leds.contains(LedCode::LED_CAPSL))
        .unwrap_or(false)
}

/// Devuelve el nombre de modificador normalizado (izquierda/derecha unificados),
/// o None si la tecla no es un modificador.
fn nombre_modificador(key: KeyCode) -> Option<&'static str> {
    match key {
        KeyCode::KEY_LEFTCTRL | KeyCode::KEY_RIGHTCTRL => Some("Ctrl"),
        KeyCode::KEY_LEFTSHIFT | KeyCode::KEY_RIGHTSHIFT => Some("Shift"),
        KeyCode::KEY_LEFTALT => Some("Alt"),
        KeyCode::KEY_RIGHTALT => Some("AltGr"),
        KeyCode::KEY_LEFTMETA | KeyCode::KEY_RIGHTMETA => Some("Super"),
        _ => None,
    }
}

/// Orden fijo de modificadores en la combinación mostrada.
const ORDEN_MODIFICADORES: [&str; 5] = ["Super", "Ctrl", "Alt", "AltGr", "Shift"];

pub fn iniciar_captura(app: AppHandle) {
    // Estado de modificadores compartido entre todos los teclados detectados.
    let modificadores_activos: Arc<Mutex<HashSet<&'static str>>> =
        Arc::new(Mutex::new(HashSet::new()));
    let xkb = XkbTranslator::new();

    thread::spawn(move || {
        let dispositivos: Vec<Device> = evdev::enumerate()
            .map(|(_, d)| d)
            .filter(es_teclado)
            .collect();

        for mut dev in dispositivos {
            let app = app.clone();
            let modificadores_activos = Arc::clone(&modificadores_activos);
            let caps_lock_activo = Arc::new(Mutex::new(caps_lock_inicial(&dev)));
            let xkb = xkb.clone();
            thread::spawn(move || loop {
                match dev.fetch_events() {
                    Ok(events) => {
                        for ev in events {
                            // value 1 = press, 0 = release, 2 = autorepeat (ignorado)
                            if let EventSummary::Key(_, key, estado @ (0 | 1)) = ev.destructure() {
                                // El emit de Tauri se hace DESPUÉS de soltar los Mutex: son
                                // solo para proteger el estado compartido entre hilos, no
                                // deben quedar retenidos durante la llamada IPC síncrona.
                                let combo_a_emitir = {
                                    let mut mods = modificadores_activos.lock().unwrap();

                                    if key == KeyCode::KEY_CAPSLOCK && estado == 1 {
                                        let mut caps = caps_lock_activo.lock().unwrap();
                                        *caps = !*caps;
                                    }

                                    if let Some(nombre_mod) = nombre_modificador(key) {
                                        if estado == 1 {
                                            mods.insert(nombre_mod);
                                        } else {
                                            mods.remove(nombre_mod);
                                        }
                                        // Emitimos el modificador solo mientras se mantiene
                                        // presionado sin ninguna otra tecla aún.
                                        if estado == 1 {
                                            Some(construir_combo(&mods, None))
                                        } else {
                                            None
                                        }
                                    } else if estado == 1 {
                                        let caps = *caps_lock_activo.lock().unwrap();
                                        let nombre = etiqueta_tecla(key, &mods, &xkb, caps);
                                        Some(construir_combo(&mods, Some(&nombre)))
                                    } else {
                                        None
                                    }
                                };

                                if let Some(combo) = combo_a_emitir {
                                    // Si el frontend ya no existe (ventana cerrada durante
                                    // shutdown), no hay razón para tumbar este hilo por eso.
                                    let _ = app.emit("key-event", combo);
                                }
                            }
                        }
                    }
                    Err(_) => {
                        thread::sleep(std::time::Duration::from_millis(10));
                    }
                }
            });
        }
    });
}

/// Determina la etiqueta de la tecla principal: si no hay modificadores
/// "pesados" (Ctrl/Alt/Super) presionados, se intenta traducir el símbolo
/// real de la distribución activa vía xkbcommon (incluye cirílico). xkb
/// mismo decide si la tecla produce texto o no (`traducir` devuelve None
/// para Backspace, F1, flechas...), así que no hace falta duplicar esa
/// lista aquí. Si hay un atajo (ej. Ctrl+C) o la tecla no es imprimible,
/// se usa el código físico de siempre.
fn etiqueta_tecla(
    key: KeyCode,
    mods: &HashSet<&'static str>,
    xkb: &Option<Arc<XkbTranslator>>,
    caps: bool,
) -> String {
    let es_atajo = mods.contains("Ctrl") || mods.contains("Alt") || mods.contains("Super");

    if !es_atajo {
        if let Some(xkb) = xkb {
            let shift = mods.contains("Shift");
            let altgr = mods.contains("AltGr");
            if let Some(simbolo) = xkb.traducir(key.code() as u32, shift, altgr, caps) {
                return simbolo;
            }
        }
    }

    format!("{:?}", key)
}

/// Construye la etiqueta de combinación, ej. "Ctrl<SEP>Shift<SEP>KEY_K",
/// respetando el orden fijo de modificadores. Usa `SEPARADOR_COMBO` en
/// vez de "+" porque "+" es también un símbolo legítimo que xkb puede
/// devolver (ej. Shift+=), y el frontend debe poder distinguir el
/// separador de un símbolo de tecla real al hacer split.
fn construir_combo(mods: &HashSet<&'static str>, tecla_principal: Option<&str>) -> String {
    let mut partes: Vec<&str> = ORDEN_MODIFICADORES
        .iter()
        .filter(|m| mods.contains(*m))
        .copied()
        .collect();

    if let Some(tecla) = tecla_principal {
        partes.push(tecla);
    }

    partes.join(&SEPARADOR_COMBO.to_string())
}
