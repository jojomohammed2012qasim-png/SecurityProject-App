// =================================================================
// MODULE: Enterprise Localization Engine (5 Languages JSON Powered)
// ARCHITECTURE: Dynamic High-Performance JSON Translation
// PATENT NOTICE: All Rights Reserved © 2026 Jana Mohammed
// =================================================================

use eframe::egui;
use serde::Deserialize;
use std::collections::HashMap;

#[derive(Deserialize, Clone, Default)]
pub struct AlertMessages {
    pub usb_blocked: String,
}

#[derive(Deserialize, Clone, Default)]
pub struct LocaleData {
    pub title: String,
    pub subtitle: String,
    pub dashboard: String,
    pub threats: String,
    pub settings: String,
    pub status_title: String,
    pub status_ok: String,
    pub status_desc: String,
    pub stations_title: String,
    pub st1: String,
    pub st2: String,
    pub st3: String,
    pub telemetry_title: String,
    pub speed: String,
    pub ram: String,
    pub cpu: String,
    pub alerts: AlertMessages,
}

#[derive(PartialEq, Clone, Copy)]
pub enum Language {
    Arabic,
    English,
    Spanish,
    Russian,
    French,
}

pub struct BlackKingApp {
    pub current_lang: Language,
    pub locales: HashMap<&'static str, LocaleData>,
    pub selected_tab: String,
    pub active_alert: Option<String>,
}

impl Default for BlackKingApp {
    fn default() -> Self {
        let mut locales = HashMap::new();

        // تحميل ملفات الـ 5 لغات بالكامل في الذاكرة
        if let Ok(data) = serde_json::from_str::<LocaleData>(include_str!("../locales/ar.json")) {
            locales.insert("ar", data);
        }
        if let Ok(data) = serde_json::from_str::<LocaleData>(include_str!("../locales/en.json")) {
            locales.insert("en", data);
        }
        if let Ok(data) = serde_json::from_str::<LocaleData>(include_str!("../locales/es.json")) {
            locales.insert("es", data);
        }
        if let Ok(data) = serde_json::from_str::<LocaleData>(include_str!("../locales/ru.json")) {
            locales.insert("ru", data);
        }
        if let Ok(data) = serde_json::from_str::<LocaleData>(include_str!("../locales/fr.json")) {
            locales.insert("fr", data);
        }

        Self {
            current_lang: Language::Arabic,
            locales,
            selected_tab: "Dashboard".to_string(),
            active_alert: None,
        }
    }
}

impl BlackKingApp {
    pub fn get_text(&self) -> &LocaleData {
        let key = match self.current_lang {
            Language::Arabic => "ar",
            Language::English => "en",
            Language::Spanish => "es",
            Language::Russian => "ru",
            Language::French => "fr",
        };
        self.locales.get(key).unwrap()
    }
}

impl eframe::App for BlackKingApp {
    fn update(&mut self, ctx: &egui::Context, _frame: &mut eframe::Frame) {
        let mut visuals = egui::Visuals::dark();
        visuals.override_text_color = Some(egui::Color32::from_rgb(235, 240, 255));
        visuals.panel_fill = egui::Color32::from_rgb(18, 22, 33);
        visuals.widgets.noninteractive.bg_fill = egui::Color32::from_rgb(28, 35, 51);
        visuals.widgets.noninteractive.rounding = egui::Rounding::same(12.0);
        ctx.set_visuals(visuals);

        let t = self.get_text().clone();

        // --- القائمة الجانبية (Left Sidebar) ---
        egui::SidePanel::left("sidebar_panel")
            .resizable(false)
            .default_width(220.0)
            .show(ctx, |ui| {
                ui.add_space(15.0);
                ui.heading(&t.title);
                ui.label(&t.subtitle);
                ui.add_space(20.0);

                // اختيارات اللغات الـ 5 القوية
                ui.label("🌐 Select Language / اختر اللغة:");
                egui::ComboBox::from_label("")
                    .selected_text(match self.current_lang {
                        Language::Arabic => "🇮🇶 العربية",
                        Language::English => "🇬🇧 English",
                        Language::Spanish => "🇪🇸 Español",
                        Language::Russian => "🇷🇺 Русский",
                        Language::French => "🇫🇷 Français",
                    })
                    .show_ui(ui, |ui| {
                        ui.selectable_value(&mut self.current_lang, Language::Arabic, "🇮🇶 العربية");
                        ui.selectable_value(&mut self.current_lang, Language::English, "🇬🇧 English");
                        ui.selectable_value(&mut self.current_lang, Language::Spanish, "🇪🇸 Español");
                        ui.selectable_value(&mut self.current_lang, Language::Russian, "🇷🇺 Русский");
                        ui.selectable_value(&mut self.current_lang, Language::French, "🇫🇷 Français");
                    });

                ui.add_space(25.0);

                if ui.button(&t.dashboard).clicked() {
                    self.selected_tab = "Dashboard".to_string();
                }
                ui.add_space(10.0);
                if ui.button(&t.threats).clicked() {
                    self.selected_tab = "Threats".to_string();
                }
                ui.add_space(10.0);
                if ui.button(&t.settings).clicked() {
                    self.selected_tab = "Settings".to_string();
                }
            });

        // --- الشاشة الرئيسية تفاعلية بحسب الترجمة ---
        egui::CentralPanel::default().show(ctx, |ui| {
            ui.add_space(15.0);
            ui.heading(&t.status_title);
            ui.add_space(15.0);

            // كارت الحالة الرئيسي
            egui::Frame::none()
                .fill(egui::Color32::from_rgb(28, 35, 51))
                .rounding(egui::Rounding::same(14.0))
                .inner_margin(egui::Margin::same(20.0))
                .show(ui, |ui| {
                    ui.horizontal(|ui| {
                        ui.label("🛡️ Status:");
                        ui.colored_label(egui::Color32::from_rgb(76, 209, 149), &t.status_ok);
                    });
                    ui.add_space(5.0);
                    ui.label(&t.status_desc);
                });

            ui.add_space(20.0);

            // كروت تفاصيل المحطات والـ Telemetry
            ui.columns(2, |cols| {
                cols[0].group(|ui| {
                    ui.heading(&t.stations_title);
                    ui.add_space(8.0);
                    ui.label(&t.st1);
                    ui.label(&t.st2);
                    ui.label(&t.st3);
                });

                cols[1].group(|ui| {
                    ui.heading(&t.telemetry_title);
                    ui.add_space(8.0);
                    ui.label(&t.speed);
                    ui.label(&t.ram);
                    ui.label(&t.cpu);
                });
            });

            // تجربة إنذار تفاعلي
            ui.add_space(30.0);
            ui.separator();
            ui.add_space(10.0);
            ui.label("⚡ Live Alerts Simulation:");
            if ui.button("🚨 Simulate USB Threat").clicked() {
                self.active_alert = Some(t.alerts.usb_blocked.clone());
            }

            if let Some(ref alert_msg) = self.active_alert {
                ui.add_space(10.0);
                egui::Frame::none()
                    .fill(egui::Color32::from_rgb(120, 30, 30))
                    .rounding(egui::Rounding::same(8.0))
                    .inner_margin(egui::Margin::same(12.0))
                    .show(ui, |ui| {
                        ui.colored_label(egui::Color32::WHITE, alert_msg);
                    });
            }
        });
    }
}
