// =================================================================
// MODULE: Dashboard UI (Modern Sleek Sapphire Theme)
// ARCHITECTURE: Rounded Cards & Modern Blue-Slate Dashboard
// PATENT NOTICE: All Rights Reserved © 2026 Jana Mohammed
// =================================================================

use eframe::egui;

pub struct BlackKingApp {
    pub selected_tab: String,
    pub is_system_safe: bool,
}

impl Default for BlackKingApp {
    fn default() -> Self {
        Self {
            selected_tab: "Dashboard".to_string(),
            is_system_safe: true,
        }
    }
}

impl eframe::App for BlackKingApp {
    fn update(&mut self, ctx: &egui::Context, _frame: &mut eframe::Frame) {
        // --- 1. تطبيق الثيم المظلم الهادئ والدرجات النيلية ---
        let mut visuals = egui::Visuals::dark();
        visuals.override_text_color = Some(egui::Color32::from_rgb(235, 240, 255));
        visuals.panel_fill = egui::Color32::from_rgb(18, 22, 33); // خلفية كحلي داكنة مريحة
        visuals.widgets.noninteractive.bg_fill = egui::Color32::from_rgb(28, 35, 51); // لون الكروت
        visuals.widgets.noninteractive.rounding = egui::Rounding::same(12.0); // حواف دائرية ناعمة
        ctx.set_visuals(visuals);

        // --- 2. القائمة الجانبية (Left Sidebar) ---
        egui::SidePanel::left("sidebar_panel")
            .resizable(false)
            .default_width(200.0)
            .show(ctx, |ui| {
                ui.add_space(20.0);
                ui.heading("👑 BLACK KING");
                ui.label("Security Suite v1.0");
                ui.add_space(30.0);

                if ui.button("📊 Dashboard").clicked() {
                    self.selected_tab = "Dashboard".to_string();
                }
                ui.add_space(10.0);
                if ui.button("🛡️ Threat Center").clicked() {
                    self.selected_tab = "Threats".to_string();
                }
                ui.add_space(10.0);
                if ui.button("⚙️ Settings").clicked() {
                    self.selected_tab = "Settings".to_string();
                }
            });

        // --- 3. الشاشة الرئيسية والتقسيم للكروت (Main Content Area) ---
        egui::CentralPanel::default().show(ctx, |ui| {
            ui.add_space(15.0);
            ui.heading("System Security Status");
            ui.add_space(15.0);

            // كارت حالة النظام الرئيسي (Main Status Card)
            egui::Frame::none()
                .fill(egui::Color32::from_rgb(28, 35, 51))
                .rounding(egui::Rounding::same(14.0))
                .inner_margin(egui::Margin::same(20.0))
                .show(ui, |ui| {
                    ui.horizontal(|ui| {
                        ui.label("🛡️ Status:");
                        ui.colored_label(egui::Color32::from_rgb(76, 209, 149), "100% SAFE & PROTECTED");
                    });
                    ui.add_space(5.0);
                    ui.label("All 3 Engine Stations are active with zero latency.");
                });

            ui.add_space(20.0);

            // كروت تفاصيل المحطات والـ Telemetry
            ui.columns(2, |cols| {
                cols[0].group(|ui| {
                    ui.heading("Engine Stations");
                    ui.add_space(8.0);
                    ui.label("✔ Station 1: Deception Engine");
                    ui.label("✔ Station 2: Dark Zone Air-Gap");
                    ui.label("✔ Station 3: AES-256 Memory Purge");
                });

                cols[1].group(|ui| {
                    ui.heading("Performance Telemetry");
                    ui.add_space(8.0);
                    ui.label("⏱ Execution Speed: 0.015 μs");
                    ui.label("💻 RAM Usage: 0.00%");
                    ui.label("⚡ CPU Load: 0.00%");
                });
            });
        });
    }
}
