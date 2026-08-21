mod app;
#[cfg(target_os = "linux")]
use app::input::iniciar_captura;
use tauri::Manager;

#[cfg_attr(mobile, tauri::mobile_entry_point)]
pub fn run() {
    tauri::Builder::default()
        .plugin(tauri_plugin_opener::init())
        .setup(|app| {
            #[cfg(target_os = "linux")]
            iniciar_captura(app.handle().clone());

            #[cfg(target_os = "linux")]
            {
                let window = app.get_webview_window("main").unwrap();
                // Si el compositor no soporta wlr-layer-shell, degradamos a
                // ventana normal en vez de tumbar toda la app: mejor un
                // overlay que se ve como ventana que ningún overlay.
                if let Err(e) = app::layer_shell::iniciar_layer_shell(&window) {
                    eprintln!("keyviz-wayland: layer-shell no disponible ({e}), usando ventana normal");
                    let _ = window.show();
                }
            }

            Ok(())
        })
        .run(tauri::generate_context!())
        .expect("error while running tauri application");
}
