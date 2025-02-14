//! API responses for the [`stats`](crate::servers::apis::v1::context::stats)
//! API context.
use axum::response::Json;

use super::resources::Stats;
use crate::tracker::services::statistics::TrackerMetrics;

/// `200` response that contains the [`Stats`](crate::servers::apis::v1::context::stats::resources::Stats) resource as json.
pub fn stats_response(tracker_metrics: TrackerMetrics) -> Json<Stats> {
    Json(Stats::from(tracker_metrics))
}
