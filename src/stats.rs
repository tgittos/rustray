use hdrhistogram::Histogram;
use std::collections::HashMap;
use std::sync;
use std::time;

const HIST_LOW: u64 = 1;
const HIST_HIGH: u64 = 60_000_000_000; // 60s in nanos
const HIST_SIGFIGS: u8 = 3;

fn make_histogram() -> Histogram<u64> {
    Histogram::new_with_bounds(HIST_LOW, HIST_HIGH, HIST_SIGFIGS)
        .expect("failed to initialize stats histogram")
}

pub struct Stat {
    name: &'static str,
    value: time::Duration,
}

impl Stat {
    pub fn new(name: &'static str, value: time::Duration) -> Self {
        Stat { name, value }
    }
}

#[derive(Clone)]
pub struct Metric {
    hist: Histogram<u64>,
    total: time::Duration,
    count: u64,
}

impl Metric {
    pub fn new() -> Self {
        Metric {
            hist: make_histogram(),
            total: time::Duration::default(),
            count: 0,
        }
    }

    fn record(&mut self, duration: time::Duration) {
        let nanos = duration
            .as_nanos()
            .max(HIST_LOW as u128)
            .min(HIST_HIGH as u128) as u64;

        // Ignore recording errors; clipping keeps bounds small and predictable.
        let _ = self.hist.record(nanos);
        self.total += duration;
        self.count += 1;
    }

    fn percentile(&self, quantile: f64) -> time::Duration {
        if self.count == 0 {
            time::Duration::default()
        } else {
            time::Duration::from_nanos(self.hist.value_at_quantile(quantile))
        }
    }
}

pub struct Stats {
    hit_metrics: HashMap<&'static str, Metric>,
    sample_metrics: HashMap<&'static str, Metric>,
    all_hit: Metric,
    all_sample: Metric,
}

impl Stats {
    pub fn new() -> Self {
        Stats {
            hit_metrics: HashMap::new(),
            sample_metrics: HashMap::new(),
            all_hit: Metric::new(),
            all_sample: Metric::new(),
        }
    }

    pub fn add_hit_stat(&mut self, stat: Stat) {
        self.all_hit.record(stat.value);

        let entry = self
            .hit_metrics
            .entry(stat.name)
            .or_insert_with(Metric::new);
        entry.record(stat.value);
    }

    pub fn add_sample_stat(&mut self, stat: Stat) {
        self.all_sample.record(stat.value);

        let entry = self
            .sample_metrics
            .entry(stat.name)
            .or_insert_with(Metric::new);
        entry.record(stat.value);
    }

    fn percentile_for(
        map: &HashMap<&'static str, Metric>,
        name: &str,
        quantile: f64,
    ) -> time::Duration {
        map.get(name)
            .map(|metric| metric.percentile(quantile))
            .unwrap_or_default()
    }

    pub fn p50_by_name(&self, name: &str) -> (time::Duration, time::Duration) {
        (
            Self::percentile_for(&self.hit_metrics, name, 0.50),
            Self::percentile_for(&self.sample_metrics, name, 0.50),
        )
    }

    pub fn p50(&self) -> (time::Duration, time::Duration) {
        (
            self.all_hit.percentile(0.50),
            self.all_sample.percentile(0.50),
        )
    }

    pub fn p90(&self) -> (time::Duration, time::Duration) {
        (
            self.all_hit.percentile(0.90),
            self.all_sample.percentile(0.90),
        )
    }

    pub fn p90_by_name(&self, name: &str) -> (time::Duration, time::Duration) {
        (
            Self::percentile_for(&self.hit_metrics, name, 0.90),
            Self::percentile_for(&self.sample_metrics, name, 0.90),
        )
    }

    pub fn p99(&self) -> (time::Duration, time::Duration) {
        (
            self.all_hit.percentile(0.99),
            self.all_sample.percentile(0.99),
        )
    }

    pub fn p99_by_name(&self, name: &str) -> (time::Duration, time::Duration) {
        (
            Self::percentile_for(&self.hit_metrics, name, 0.99),
            Self::percentile_for(&self.sample_metrics, name, 0.99),
        )
    }

    pub fn total_hit_time(&self) -> time::Duration {
        self.all_hit.total
    }

    pub fn total_hit_time_by_name(&self, name: &str) -> time::Duration {
        self.hit_metrics
            .get(name)
            .map(|metric| metric.total)
            .unwrap_or_default()
    }

    pub fn total_sample_time(&self) -> time::Duration {
        self.all_sample.total
    }

    pub fn total_sample_time_by_name(&self, name: &str) -> time::Duration {
        self.sample_metrics
            .get(name)
            .map(|metric| metric.total)
            .unwrap_or_default()
    }

    pub fn total_time(&self) -> time::Duration {
        self.total_hit_time() + self.total_sample_time()
    }

    pub fn total_time_by_name(&self, name: &str) -> time::Duration {
        self.total_hit_time_by_name(name) + self.total_sample_time_by_name(name)
    }

    pub fn total_hits(&self) -> usize {
        self.all_hit.count as usize
    }

    pub fn total_samples(&self) -> usize {
        self.all_sample.count as usize
    }
}

pub static DIELECTRIC_HIT: &str = "dielectric_hit";
pub static DIELECTRIC_SAMPLE: &str = "dielectric_sample";
pub static LAMBERTIAN_HIT: &str = "lambertian_hit";
pub static LAMBERTIAN_SAMPLE: &str = "lambertian_sample";
pub static METALLIC_HIT: &str = "metallic_hit";
pub static METALLIC_SAMPLE: &str = "metallic_sample";
pub static DIFFUSE_LIGHT_HIT: &str = "diffuse_light_hit";
pub static DIFFUSE_LIGHT_SAMPLE: &str = "diffuse_light_sample";
pub static SCENE_HIT: &str = "scene_hit";
pub static SCENE_SAMPLE: &str = "scene_sample";

static STATS: sync::LazyLock<sync::Mutex<Stats>> =
    sync::LazyLock::new(|| sync::Mutex::new(Stats::new()));

pub fn reset() {
    let mut stats = STATS.lock().unwrap();
    *stats = Stats::new();
}

pub fn add_hit_stat(stat: Stat) {
    let mut stats = STATS.lock().unwrap();
    stats.add_hit_stat(stat);
}

pub fn add_sample_stat(stat: Stat) {
    let mut stats = STATS.lock().unwrap();
    stats.add_sample_stat(stat);
}

pub fn get_stats() -> sync::MutexGuard<'static, Stats> {
    STATS.lock().unwrap()
}
