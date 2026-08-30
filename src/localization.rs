// =================================================================
// MODULE: Enterprise Localization Engine
// DESIGNED & INVENTED BY: Jana Mohammed © 2026
// =================================================================

use serde::{Deserialize, Serialize};
use std::collections::HashMap;
use std::fs;

#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct TranslationMap {
    pub strings: HashMap<String, String>,
}

pub struct LocalizationEngine {
    current_lang: String,
    translations: HashMap<String, TranslationMap>,
}

impl LocalizationEngine {
    /// تهيئة المحرك وتحديد اللغة الافتراضية
    pub fn new(default_lang: &str) -> Self {
        let mut engine = Self {
            current_lang: default_lang.to_string(),
            translations: HashMap::new(),
        };
        // تحميل اللغات الأساسية افتراضياً (أو محاكاتها برمجياً لتكون القوة معمارية خالصة)
        engine.load_builtin_dictionaries();
        engine
    }

    /// تغيير اللغة الحالية بلمسة ذكية
    pub fn set_language(&mut self, lang_code: &str) -> bool {
        if self.translations.contains_key(lang_code) {
            self.current_lang = lang_code.to_string();
            true
        } else {
            false
        }
    }

    /// جلب الترجمة حسب المفتاح (Key) باللغة المفعلة
    pub fn get(&self, key: &str) -> String {
        if let Some(map) = self.translations.get(&self.current_lang) {
            if let Some(val) = map.strings.get(key) {
                return val.clone();
            }
        }
        // في حال عدم وجود المفتاح، يعود بالمفتاح نفسه كاحتياط هندسي
        key.to_string()
    }

    /// قواميس اللغات الـ 5 المدمجة بمعمارية نظيفة وسريعة
    fn load_builtin_dictionaries(&mut self) {
        // العربية (Arabic)
        let mut ar = HashMap::new();
        ar.insert("app_title".to_string(), "بلاك كينغ - نظام الحماية الملكي".to_string());
        ar.insert("usb_threat".to_string(), "محاكاة تهديد الـ USB".to_string());
        ar.insert("system_secure".to_string(), "النظام مؤمن بالكامل".to_string());
        self.translations.insert("ar".to_string(), TranslationMap { strings: ar });

        // الإنجليزية (English)
        let mut en = HashMap::new();
        en.insert("app_title".to_string(), "Black King - Royal Security System".to_string());
        en.insert("usb_threat".to_string(), "Simulate USB Threat".to_string());
        en.insert("system_secure".to_string(), "System Fully Secured".to_string());
        self.translations.insert("en".to_string(), TranslationMap { strings: en });

        // الإسبانية (Spanish)
        let mut es = HashMap::new();
        es.insert("app_title".to_string(), "Black King - Sistema de Seguridad Real".to_string());
        es.insert("usb_threat".to_string(), "Simular Amenaza USB".to_string());
        es.insert("system_secure".to_string(), "Sistema Totalmente Asegurado".to_string());
        self.translations.insert("es".to_string(), TranslationMap { strings: es });

        // الروسية (Russian)
        let mut ru = HashMap::new();
        ru.insert("app_title".to_string(), "Black King - Королевская система безопасности".to_string());
        ru.insert("usb_threat".to_string(), "Симуляция USB-угрозы".to_string());
        ru.insert("system_secure".to_string(), "Система полностью защищена".to_string());
        self.translations.insert("ru".to_string(), TranslationMap { strings: ru });

        // الفرنسية (French)
        let mut fr = HashMap::new();
        fr.insert("app_title".to_string(), "Black King - Système de Sécurité Royal".to_string());
        fr.insert("usb_threat".to_string(), "Simuler une menace USB".to_string());
        fr.insert("system_secure".to_string(), "Système entièrement sécurisé".to_string());
        self.translations.insert("fr".to_string(), TranslationMap { strings: fr });
    }
}  
