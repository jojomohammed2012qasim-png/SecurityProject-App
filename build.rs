// =================================================================
// BUILD SCRIPT: Windows Resource & Application Icon Embedder
// PATENT NOTICE: All Rights Reserved © 2026 Jana Mohammed
// =================================================================

#[cfg(target_os = "windows")]
fn main() {
    let mut res = winres::WindowsResource::new();
    res.set_icon("app_icon.ico");
    res.compile().unwrap();
}

#[cfg(not(target_os = "windows"))]
fn main() {}
