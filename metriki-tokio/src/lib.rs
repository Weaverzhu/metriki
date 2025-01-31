//! # Metriki Tokio Instrument
//!
//! This library provides
//! [tokio-metrics](https://github.com/tokio/tokio-metrics)
//! integration for metriki.
//!
//! # Setup
//!
//! This library provides `RuntimeMetrics` by default. Accroding to
//! tokio-metrics, `tokio_unstable` is required for this feature. See
//! the
//! [doc](https://github.com/tokio-rs/tokio-metrics#getting-started-with-runtime-metrics)
//! for steps to setup the cargo configuration.
//!
//! To disable unstable features, you can include this library with
//! `default-features = false`.
//!
//! # Usage
//!
//! [An
//! Example](https://github.com/sunng87/metriki/blob/master/metriki-tokio/examples/server.rs)
//! is provided in the codebase.
//!
//! Check [docs of tokio-metrics](https://docs.rs/tokio-metrics/) for
//! meaning of the metrics.
//!
use std::collections::HashMap;
use std::fmt::{self};
use std::sync::{Arc, Mutex};

use derive_builder::Builder;
use metriki_core::metrics::{Metric, StaticGauge};
use metriki_core::MetricsSet;

use tokio_metrics::{RuntimeMetrics, RuntimeMonitor, TaskMetrics, TaskMonitor};

/// A MetricsSet works with tokio_metrics `TaskMonitor`.
///
#[derive(Builder)]
pub struct TokioTaskMetricsSet {
    #[builder(setter(into))]
    name: String,
    #[builder(setter(custom))]
    monitor: Arc<Mutex<dyn Iterator<Item = TaskMetrics> + Send>>,
}

impl fmt::Debug for TokioTaskMetricsSet {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> Result<(), fmt::Error> {
        f.debug_struct("TokioTaskMetricsSet")
            .field("name", &self.name)
            .finish()
    }
}

impl TokioTaskMetricsSet {
    pub fn name(&self) -> &String {
        &self.name
    }
}

impl TokioTaskMetricsSetBuilder {
    pub fn monitor(&mut self, monitor: &TaskMonitor) -> &Self {
        self.monitor = Some(Arc::new(Mutex::new(monitor.intervals())));
        self
    }
}

impl MetricsSet for TokioTaskMetricsSet {
    fn get_all(&self) -> HashMap<String, Metric> {
        let metrics: TaskMetrics = self.monitor.lock().unwrap().next().unwrap();

        let mut result = HashMap::new();
        result.insert(
            format!("{}.first_poll_count", self.name),
            Metric::gauge(Box::new(StaticGauge(metrics.first_poll_count as f64))).into(),
        );
        result.insert(
            format!("{}.instrumented_count", self.name),
            Metric::gauge(Box::new(StaticGauge(metrics.instrumented_count as f64))).into(),
        );
        result.insert(
            format!("{}.dropped_count", self.name),
            Metric::gauge(Box::new(StaticGauge(metrics.dropped_count as f64))).into(),
        );
        result.insert(
            format!("{}.total_poll_count", self.name),
            Metric::gauge(Box::new(StaticGauge(metrics.total_poll_count as f64))).into(),
        );
        result.insert(
            format!("{}.total_idled_count", self.name),
            Metric::gauge(Box::new(StaticGauge(metrics.total_idled_count as f64))).into(),
        );
        result.insert(
            format!("{}.total_scheduled_count", self.name),
            Metric::gauge(Box::new(StaticGauge(metrics.total_scheduled_count as f64))).into(),
        );
        result.insert(
            format!("{}.total_slow_poll_count", self.name),
            Metric::gauge(Box::new(StaticGauge(metrics.total_slow_poll_count as f64))).into(),
        );
        result.insert(
            format!("{}.total_fast_poll_count", self.name),
            Metric::gauge(Box::new(StaticGauge(metrics.total_fast_poll_count as f64))).into(),
        );

        result.insert(
            format!("{}.mean_poll_duration", self.name),
            Metric::gauge(Box::new(StaticGauge(
                metrics.mean_poll_duration().as_millis() as f64,
            )))
            .into(),
        );

        result.insert(
            format!("{}.mean_first_poll_delay", self.name),
            Metric::gauge(Box::new(StaticGauge(
                metrics.mean_first_poll_delay().as_millis() as f64,
            )))
            .into(),
        );

        result.insert(
            format!("{}.mean_scheduled_duration", self.name),
            Metric::gauge(Box::new(StaticGauge(
                metrics.mean_scheduled_duration().as_millis() as f64,
            )))
            .into(),
        );

        result
    }
}

/// A MetricsSet works with tokio_metrics `TaskMonitor`.
///
#[cfg(feature = "rt")]
#[derive(Builder)]
pub struct TokioRuntimeMetricsSet {
    #[builder(setter(into))]
    name: String,
    #[builder(setter(custom))]
    monitor: Arc<Mutex<dyn Iterator<Item = RuntimeMetrics> + Send>>,
}

#[cfg(feature = "rt")]
impl TokioRuntimeMetricsSetBuilder {
    pub fn monitor(&mut self, monitor: &RuntimeMonitor) -> &Self {
        self.monitor = Some(Arc::new(Mutex::new(monitor.intervals())));
        self
    }
}

#[cfg(feature = "rt")]
impl fmt::Debug for TokioRuntimeMetricsSet {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> Result<(), fmt::Error> {
        f.debug_struct("TokioRuntimeMetricsSet")
            .field("name", &self.name)
            .finish()
    }
}

#[cfg(feature = "rt")]
impl TokioRuntimeMetricsSet {
    pub fn name(&self) -> &String {
        &self.name
    }
}

#[cfg(feature = "rt")]
impl MetricsSet for TokioRuntimeMetricsSet {
    fn get_all(&self) -> HashMap<String, Metric> {
        let metrics: RuntimeMetrics = self.monitor.lock().unwrap().next().unwrap();

        let mut result = HashMap::new();
        result.insert(
            format!("{}.total_polls_count", self.name),
            Metric::gauge(Box::new(StaticGauge(metrics.total_polls_count as f64))).into(),
        );
        result.insert(
            format!("{}.total_steal_count", self.name),
            Metric::gauge(Box::new(StaticGauge(metrics.total_steal_count as f64))).into(),
        );
        result.insert(
            format!("{}.total_park_count", self.name),
            Metric::gauge(Box::new(StaticGauge(metrics.total_park_count as f64))).into(),
        );
        result.insert(
            format!("{}.num_remote_schedules", self.name),
            Metric::gauge(Box::new(StaticGauge(metrics.num_remote_schedules as f64))).into(),
        );
        result.insert(
            format!("{}.total_local_schedule_count", self.name),
            Metric::gauge(Box::new(StaticGauge(
                metrics.total_local_schedule_count as f64,
            )))
            .into(),
        );
        result.insert(
            format!("{}.total_overflow_count", self.name),
            Metric::gauge(Box::new(StaticGauge(metrics.total_overflow_count as f64))).into(),
        );
        result.insert(
            format!("{}.total_noop_count", self.name),
            Metric::gauge(Box::new(StaticGauge(metrics.total_noop_count as f64))).into(),
        );

        result.insert(
            format!("{}.total_busy_duration", self.name),
            Metric::gauge(Box::new(StaticGauge(
                metrics.total_busy_duration.as_millis() as f64,
            )))
            .into(),
        );

        result.insert(
            format!("{}.busy_ratio", self.name),
            Metric::gauge(Box::new(StaticGauge(metrics.busy_ratio()))).into(),
        );

        result
    }
}
