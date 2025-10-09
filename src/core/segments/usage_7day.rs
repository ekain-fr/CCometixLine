use super::{Segment, SegmentData};
use crate::config::{InputData, SegmentId};
use crate::core::segments::usage::UsageSegment;
use std::collections::HashMap;

#[derive(Default)]
pub struct Usage7DaySegment;

impl Usage7DaySegment {
    pub fn new() -> Self {
        Self
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
