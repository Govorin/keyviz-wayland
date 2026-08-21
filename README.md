# keyviz-wayland (fork)

Fork of [notlimdev/keyviz-wayland](https://github.com/notlimdev/keyviz-wayland), which itself is a Tauri/Rust port of [keyviz](https://github.com/keyviz/keyviz) for Wayland.

This fork was built for [niri](https://github.com/YaLTeR/niri) and other `wlr-layer-shell` compositors, to fix issues the upstream binary had there: a large window showing a scrolling key history, no way to reposition it without dragging, and Latin-only labels regardless of the active keyboard layout.

## What's different from upstream

- **True layer-shell overlay, not a regular window.** The window is turned into a `wlr-layer-shell` surface (via [`gtk-layer-shell`](https://github.com/wmww/gtk-layer-shell)) instead of a normal Tauri/GTK window. It no longer appears in alt-tab / workspace overview, never steals keyboard focus, and stays anchored to the bottom-center of the screen. If layer-shell isn't supported by the compositor, it falls back to a normal window instead of crashing.
- **One combo chip, not a scrolling history.** Upstream kept the last 6 keypresses in a row, each as a separate chip, which grew into a wide window. This fork tracks currently-held modifiers and shows a single chip per keystroke — e.g. `Ctrl + Shift + K` — that clears itself after a short idle timeout instead of accumulating.
- **Real layout symbols via xkbcommon, including Cyrillic.** Upstream mapped raw evdev keycodes to their *physical* US-QWERTY name (`KEY_K`), so switching to a Cyrillic layout still showed Latin letters. This fork resolves the actual character produced by the system's configured X11 layout (via `libxkbcommon`), polls the active layout group from `niri msg -j keyboard-layouts`, and correctly tracks Shift/CapsLock/AltGr. Keyboard shortcuts (Ctrl/Alt/Super combos) and non-printable keys (Backspace, F-keys, arrows...) still show their physical key name, since translating those through xkb doesn't make sense.
- **Compact by default.** No visible empty box between keystrokes, smaller default window size, and the vertical/horizontal offset from the screen edge is tunable in `layer_shell.rs`.

## Requirements

Linux only, on a Wayland compositor that implements `wlr-layer-shell-unstable-v1` (niri, sway, Hyprland, ...). Needs `libxkbcommon` and `gtk-layer-shell` (both are common desktop dependencies, e.g. already pulled in by `eww`/`waybar`/`swaync` on most setups).

The active-layout polling currently shells out to `niri msg -j keyboard-layouts` specifically; on other compositors the overlay still works but keeps translating with whatever layout group was active at startup.

## Building

```sh
bun install
cargo tauri build
```

The resulting binary is self-contained; install it wherever you like (e.g. `/usr/local/bin`) and add it to your compositor's autostart.

---

Original upstream README:

> Project inspired by [keyviz](https://github.com/keyviz/keyviz)
