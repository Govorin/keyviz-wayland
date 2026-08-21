// input.rs (evdev) y xkb_translator.rs (xkbcommon) son exclusivos de
// Linux: ver el comentario en Cargo.toml sobre el alcance de este fork.
#[cfg(target_os = "linux")]
pub mod input;
#[cfg(target_os = "linux")]
pub mod layer_shell;
#[cfg(target_os = "linux")]
pub mod xkb_translator;
