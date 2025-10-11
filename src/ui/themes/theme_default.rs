use crate::config::{
    AnsiColor, ColorConfig, IconConfig, SegmentConfig, SegmentId, TextStyleConfig,
};
use std::collections::HashMap;

pub fn model_segment() -> SegmentConfig {
    SegmentConfig {
        id: SegmentId::Model,
        enabled: true,
        icon: IconConfig {
            plain: "🤖".to_string(),
            nerd_font: "\u{e26d}".to_string(),
        },
        colors: ColorConfig {
            icon: Some(AnsiColor::Color16 { c16: 14 }), // Cyan
            text: Some(AnsiColor::Color16 { c16: 14 }),
            background: None,
        },
        styles: TextStyleConfig::default(),
        options: HashMap::new(),
    }
}

pub fn directory_segment() -> SegmentConfig {
    SegmentConfig {
        id: SegmentId::Directory,
        enabled: true,
        icon: IconConfig {
            plain: "📁".to_string(),
            nerd_font: "\u{f024b}".to_string(),
        },
        colors: ColorConfig {
            icon: Some(AnsiColor::Color16 { c16: 11 }), // Yellow
            text: Some(AnsiColor::Color16 { c16: 10 }), // Green
            background: None,
        },
        styles: TextStyleConfig::default(),
        options: HashMap::new(),
    }
}

pub fn git_segment() -> SegmentConfig {
    SegmentConfig {
        id: SegmentId::Git,
        enabled: true,
        icon: IconConfig {
            plain: "🌿".to_string(),
            nerd_font: "\u{f02a2}".to_string(),
        },
        colors: ColorConfig {
            icon: Some(AnsiColor::Color16 { c16: 12 }), // Blue
            text: Some(AnsiColor::Color16 { c16: 12 }),
            background: None,
        },
        styles: TextStyleConfig::default(),
        options: {
            let mut opts = HashMap::new();
            opts.insert("show_sha".to_string(), serde_json::Value::Bool(false));
            opts.insert("show_dirty_count".to_string(), serde_json::Value::Bool(true));
            opts
        },
    }
}

pub fn context_window_segment() -> SegmentConfig {
    SegmentConfig {
        id: SegmentId::ContextWindow,
        enabled: true,
        icon: IconConfig {
            plain: "⚡️".to_string(),
            nerd_font: "\u{f49b}".to_string(),
        },
        colors: ColorConfig {
            icon: Some(AnsiColor::Color16 { c16: 13 }), // Magenta
            text: Some(AnsiColor::Color16 { c16: 13 }),
            background: None,
        },
        styles: TextStyleConfig::default(),
        options: {
            let mut opts = HashMap::new();
            opts.insert(
                "warning_threshold".to_string(),
                serde_json::Value::Number(60.into()),
            );
            opts.insert(
                "critical_threshold".to_string(),
                serde_json::Value::Number(80.into()),
            );
            opts.insert(
                "warning_color".to_string(),
                serde_json::json!({"c16": 11}),
            );
            opts.insert(
                "critical_color".to_string(),
                serde_json::json!({"c16": 9}),
            );
            opts.insert(
                "warning_bold".to_string(),
                serde_json::Value::Bool(false),
            );
            opts.insert(
                "critical_bold".to_string(),
                serde_json::Value::Bool(true),
            );
            opts
        },
    }
}

pub fn usage_segment() -> SegmentConfig {
    SegmentConfig {
        id: SegmentId::Usage,
        enabled: false,
        icon: IconConfig {
            plain: "📊".to_string(),
            nerd_font: "\u{f0a9e}".to_string(), // circle_slice_1
        },
        colors: ColorConfig {
            icon: Some(AnsiColor::Color16 { c16: 14 }), // Cyan
            text: Some(AnsiColor::Color16 { c16: 14 }),
            background: None,
        },
        styles: TextStyleConfig::default(),
        options: {
            let mut opts = HashMap::new();
            opts.insert(
                "api_base_url".to_string(),
                serde_json::Value::String("https://api.anthropic.com".to_string()),
            );
            opts.insert(
                "cache_duration".to_string(),
                serde_json::Value::Number(180.into()),
            );
            opts.insert("timeout".to_string(), serde_json::Value::Number(2.into()));
            opts
        },
    }
}

pub fn usage_5hour_segment() -> SegmentConfig {
    SegmentConfig {
        id: SegmentId::Usage5Hour,
        enabled: false,
        icon: IconConfig {
            plain: "📊".to_string(),
            nerd_font: "\u{f0a9e}".to_string(), // circle_slice_1
        },
        colors: ColorConfig {
            icon: Some(AnsiColor::Color16 { c16: 14 }), // Cyan
            text: Some(AnsiColor::Color16 { c16: 14 }),
            background: None,
        },
        styles: TextStyleConfig::default(),
        options: {
            let mut opts = HashMap::new();
            opts.insert(
                "warning_threshold".to_string(),
                serde_json::Value::Number(60.into()),
            );
            opts.insert(
                "critical_threshold".to_string(),
                serde_json::Value::Number(80.into()),
            );
            opts.insert(
                "warning_color".to_string(),
                serde_json::json!({"c16": 11}),
            );
            opts.insert(
                "critical_color".to_string(),
                serde_json::json!({"c16": 9}),
            );
            opts.insert(
                "warning_bold".to_string(),
                serde_json::Value::Bool(false),
            );
            opts.insert(
                "critical_bold".to_string(),
                serde_json::Value::Bool(true),
            );
            opts
        },
    }
}

pub fn usage_7day_segment() -> SegmentConfig {
    SegmentConfig {
        id: SegmentId::Usage7Day,
        enabled: false,
        icon: IconConfig {
            plain: "📊".to_string(),
            nerd_font: "\u{f0a9e}".to_string(), // circle_slice_1
        },
        colors: ColorConfig {
            icon: Some(AnsiColor::Color16 { c16: 12 }), // Light Blue (to differentiate from 5hour)
            text: Some(AnsiColor::Color16 { c16: 12 }),
            background: None,
        },
        styles: TextStyleConfig::default(),
        options: {
            let mut opts = HashMap::new();
            opts.insert(
                "warning_threshold".to_string(),
                serde_json::Value::Number(60.into()),
            );
            opts.insert(
                "critical_threshold".to_string(),
                serde_json::Value::Number(80.into()),
            );
            opts.insert(
                "warning_color".to_string(),
                serde_json::json!({"c16": 11}),
            );
            opts.insert(
                "critical_color".to_string(),
                serde_json::json!({"c16": 9}),
            );
            opts.insert(
                "warning_bold".to_string(),
                serde_json::Value::Bool(false),
            );
            opts.insert(
                "critical_bold".to_string(),
                serde_json::Value::Bool(true),
            );
            opts
        },
    }
}

pub fn cost_segment() -> SegmentConfig {
    SegmentConfig {
        id: SegmentId::Cost,
        enabled: false,
        icon: IconConfig {
            plain: "💰".to_string(),
            nerd_font: "\u{eec1}".to_string(),
        },
        colors: ColorConfig {
            icon: Some(AnsiColor::Color16 { c16: 3 }), // Yellow
            text: Some(AnsiColor::Color16 { c16: 3 }),
            background: None,
        },
        styles: TextStyleConfig::default(),
        options: HashMap::new(),
    }
}

pub fn session_segment() -> SegmentConfig {
    SegmentConfig {
        id: SegmentId::Session,
        enabled: false,
        icon: IconConfig {
            plain: "⏱️".to_string(),
            nerd_font: "\u{f19bb}".to_string(),
        },
        colors: ColorConfig {
            icon: Some(AnsiColor::Color16 { c16: 2 }), // Green
            text: Some(AnsiColor::Color16 { c16: 2 }),
            background: None,
        },
        styles: TextStyleConfig::default(),
        options: HashMap::new(),
    }
}

pub fn output_style_segment() -> SegmentConfig {
    SegmentConfig {
        id: SegmentId::OutputStyle,
        enabled: false,
        icon: IconConfig {
            plain: "🎯".to_string(),
            nerd_font: "\u{f12f5}".to_string(),
        },
        colors: ColorConfig {
            icon: Some(AnsiColor::Color16 { c16: 6 }), // Cyan
            text: Some(AnsiColor::Color16 { c16: 6 }),
            background: None,
        },
        styles: TextStyleConfig::default(),
        options: HashMap::new(),
    }
}
