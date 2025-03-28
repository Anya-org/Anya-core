#![feature(edition2021)]
//! Chaos Visualization Performance Metrics
use prometheus::{register, HistogramOpts, HistogramVec, IntCounterOpts, IntCounterVec};

lazy_static! {
    pub static ref VISUALIZATION_METRICS: VisualizationMetrics = { VisualizationMetrics::new() };
}

pub struct VisualizationMetrics {
    pub frame_rate: HistogramVec,
    pub data_latency: HistogramVec,
    pub node_count: IntCounterVec,
}

impl VisualizationMetrics {
    pub fn new() -> Self {
        Self {
            frame_rate: register_histogram_vec!(
                "chaos_visualization_fps",
                "Frames per second",
                &["scene"]
            )
            .unwrap(),
            data_latency: register_histogram_vec!(
                "chaos_network_data_latency",
                "Data update latency",
                &["source"]
            )
            .unwrap(),
            node_count: register_int_counter_vec!(
                "chaos_visualization_nodes",
                "Node count metrics",
                &["type"]
            )
            .unwrap(),
        }
    }
}
