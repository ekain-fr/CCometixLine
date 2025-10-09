use super::{Segment, SegmentData};
use crate::config::{InputData, SegmentId};
use crate::core::segments::usage::UsageSegment;
use std::collections::HashMap;

#[derive(Default)]
pub struct Usage5HourSegment;

impl Usage5HourSegment {
    pub fn new() -> Self {
        Self
    }
}

impl Segment for Usage5HourSegment {
    fn collect(&self, _input: &InputData) -> Option<SegmentData> {
        // Load the shared cache created by UsageSegment
        let cache = UsageSegment::load_usage_cache()?;

        let five_hour_util = cache.five_hour_utilization;
        let reset_time = UsageSegment::format_5hour_reset_time(cache.five_hour_resets_at.as_deref());

        // Use the same circle icon logic based on utilization
        let dynamic_icon = UsageSegment::get_circle_icon(five_hour_util / 100.0);

        let five_hour_percent = five_hour_util.round() as u8;
        let primary = format!("{}%", five_hour_percent);
        let secondary = format!("→ {}", reset_time);

        let mut metadata = HashMap::new();
        metadata.insert("dynamic_icon".to_string(), dynamic_icon);
        metadata.insert("five_hour_utilization".to_string(), five_hour_util.to_string());

        Some(SegmentData {
            primary,
            secondary,
            metadata,
        })
    }

    fn id(&self) -> SegmentId {
        SegmentId::Usage5Hour
    }
}
