//! Android entry point for Craft Anything.
//!
//! Android's `NativeActivity` loads this library and calls [`android_main`] on a dedicated
//! thread. It opens the shared app ([`craft_anything::run_android`], the same code the desktop
//! binary runs) and returns when the window closes, ending the activity's main-loop thread.
//!
//! This crate exists because that export must be unmangled, and on edition 2024 the only spelling
//! (`#[unsafe(no_mangle)]`) trips the workspace-forbidden `unsafe_code` lint — see `Cargo.toml`
//! for why the crate carries its own lints instead.
//!
//! Android limitations (v1): no command line, so there is no path to open at launch. Settings,
//! logs and new projects live in the app's private storage (the activity resolves it), since
//! `HOME`, `Documents` and the XDG config folder don't exist on the target. The native file
//! dialogs (rfd) and the clipboard-picture reader (arboard) have no Android backend, so choosing
//! a file, exporting a report and pasting a picture answer "not available yet"; shared storage
//! can still be reached by a typed path when the person grants "All files access".
//!
//! On every other target this crate is empty: only Android's loader ever looks for `android_main`.

/// NativeActivity's main-loop thread entry point (required; see the `android-activity` docs).
/// winit pumps the activity's lifecycle and input events inside [`craft_anything::run_android`],
/// which needs the activity's handle to create its event loop; when the activity is destroyed that
/// returns and this returns with it.
#[cfg(target_os = "android")]
#[unsafe(no_mangle)]
pub fn android_main(app: android_activity::AndroidApp) {
    if let Err(e) = craft_anything::run_android(app) {
        log::error!("craft-anything exited with an error: {e}");
    }
}
