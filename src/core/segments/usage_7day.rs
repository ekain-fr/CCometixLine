use super::{Segment, SegmentData};
use crate::config::{AnsiColor, Config, InputData, SegmentId};
use crate::core::segments::usage::UsageSegment;
use std::collections::HashMap;

#[derive(Default)]
pub struct Usage7DaySegment;

impl Usage7DaySegment {
    pub fn new() -> Self {
        Self
    }

    fn get_color_for_utilization(&self, utilization: f64) -> Option<AnsiColor> {
        // Load config to get threshold settings
        let config = Config::load().ok()?;
        let segment_config = config.segments.iter().find(|s| s.id == SegmentId::Usage7Day)?;

        // Get threshold values from options
        let warning_threshold = segment_config
            .options
            .get("warning_threshold")
            .and_then(|v| v.as_u64())
            .unwrap_or(60) as f64;

        let critical_threshold = segment_config
            .options
            .get("critical_threshold")
            .and_then(|v| v.as_u64())
            .unwrap_or(80) as f64;

        // Determine which color to use based on utilization
        if utilization >= critical_threshold {
            // Critical threshold exceeded - use critical color
            segment_config
                .options
                .get("critical_color")
                .and_then(|v| {
                    if let Some(c256) = v.get("c256").and_then(|c| c.as_u64()) {
                        Some(AnsiColor::Color256 { c256: c256 as u8 })
                    } else if let Some(c16) = v.get("c16").and_then(|c| c.as_u64()) {
                        Some(AnsiColor::Color16 { c16: c16 as u8 })
                    } else {
                        None
                    }
                })
        } else if utilization >= warning_threshold {
            // Warning threshold exceeded - use warning color
            segment_config
                .options
                .get("warning_color")
                .and_then(|v| {
                    if let Some(c256) = v.get("c256").and_then(|c| c.as_u64()) {
                        Some(AnsiColor::Color256 { c256: c256 as u8 })
                    } else if let Some(c16) = v.get("c16").and_then(|c| c.as_u64()) {
                        Some(AnsiColor::Color16 { c16: c16 as u8 })
                    } else {
                        None
                    }
                })
        } else {
            // Below warning threshold - use default color
            None
        }
    }
}

impl Segment for Usage7DaySegment {
    fn collect(&self, _input: &InputData) -> Option<SegmentData> {
        // Load the shared cache created by UsageSegment
        let cache = UsageSegment::load_usage_cache()?;

        let seven_day_util = cache.seven_day_utilization;
        let reset_time = UsageSegment::format_7day_reset_time(cache.seven_day_resets_at.as_deref());

        // Use the same circle icon logic based on utilization
        let dynamic_icon = UsageSegment::get_circle_icon(seven_day_util / 100.0);

        let seven_day_percent = seven_day_util.round() as u8;
        let primary = format!("{}%", seven_day_percent);
        let secondary = format!("→ {}", reset_time);

        let mut metadata = HashMap::new();
        metadata.insert("dynamic_icon".to_string(), dynamic_icon);
        metadata.insert("seven_day_utilization".to_string(), seven_day_util.to_string());

        // Check if we need to apply threshold-based color override
        if let Some(color) = self.get_color_for_utilization(seven_day_util) {
            // Serialize the color to JSON for metadata
            let color_json = match color {
                AnsiColor::Color256 { c256 } => {
                    serde_json::json!({"c256": c256}).to_string()
                }
                AnsiColor::Color16 { c16 } => {
                    serde_json::json!({"c16": c16}).to_string()
                }
                AnsiColor::Rgb { r, g, b } => {
                    serde_json::json!({"r": r, "g": g, "b": b}).to_string()
                }
            };
            metadata.insert("text_color_override".to_string(), color_json);
        }

        Some(SegmentData {
            primary,
            secondary,
            metadata,
        })
    }

    fn id(&self) -> SegmentId {
        SegmentId::Usage7Day
    }
}
