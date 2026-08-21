use gtk_layer_shell::{Edge, KeyboardMode, Layer, LayerShell};
use tauri::{WebviewWindow, Wry};

/// Convierte la ventana principal en una layer-surface de wlr-layer-shell:
/// sin decoraciones, sin foco de teclado, siempre por encima, sin aparecer
/// en el listado de ventanas del compositor (alt-tab / overview de niri).
///
/// Debe llamarse ANTES de que la ventana se muestre por primera vez, ya que
/// gtk-layer-shell solo puede inicializar una ventana GTK que aún no está
/// mapeada. Por eso la ventana se crea con `visible: false` en
/// `tauri.conf.json` y se muestra manualmente al final de esta función.
pub fn iniciar_layer_shell(window: &WebviewWindow<Wry>) -> tauri::Result<()> {
    let gtk_window = window.gtk_window()?;

    gtk_window.init_layer_shell();
    gtk_window.set_layer(Layer::Overlay);
    gtk_window.set_namespace("keyviz-overlay");

    // Nunca recibe foco de teclado: el overlay es solo informativo.
    gtk_window.set_keyboard_mode(KeyboardMode::None);

    // Anclado a las tres orillas horizontales para que el compositor
    // estire la superficie a todo el ancho de la pantalla; el centrado
    // real se hace con flexbox dentro del contenido (App.css / KeyOverlay).
    gtk_window.set_anchor(Edge::Bottom, true);
    gtk_window.set_anchor(Edge::Left, true);
    gtk_window.set_anchor(Edge::Right, true);
    gtk_window.set_anchor(Edge::Top, false);
    gtk_window.set_layer_shell_margin(Edge::Bottom, 40);

    // No reserva espacio exclusivo: otras ventanas pueden ocupar esa zona.
    gtk_window.set_exclusive_zone(-1);

    window.show()?;

    Ok(())
}
