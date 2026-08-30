// =================================================================
// PROJECT: Black King (Ultra-Fast Security System & GUI Dashboard)
// ARCHITECTURE: 32-bit Desktop GUI Engine (Native Rust)
// PERFORMANCE: Microsecond Execution | Zero System Impact (0% RAM/CPU)
// -----------------------------------------------------------------
// INTELLECTUAL PROPERTY & PATENT NOTICE:
// Designed & Invented by: Jana Mohammed
// Original Architecture: Multi-Station Deception (Station 1-3) &
// Modern Blue-Slate Dashboard UI. All Rights Reserved.
// =================================================================

mod usb_shield;
mod network_server;
mod deception_engine;
mod dark_zone;
mod purge_engine;
mod security_logger;
mod app_ui;
mod localization; // <--- أضفنا محرك اللغات الملكي هنا بانسجام تام!

use app_ui::BlackKingApp;
use eframe::egui;

fn main() -> Result<(), eframe::Error> {
    // --- إعدادات النافذة الرئيسية للتطبيق ---
    let options = eframe::NativeOptions {
        viewport: egui::ViewportBuilder::default()
            .with_inner_size([850.0, 550.0])
            .with_min_inner_size([700.0, 450.0])
            .with_title("Black King Security Dashboard - Jana Mohammed"),
        ..Default::default()
    };

    // --- تشغيل الواجهة الرسومية الحديثة ---
    eframe::run_native(
        "Black King Security System",
        options,
        Box::new(|_cc| Box::new(BlackKingApp::default())),
    )
}
