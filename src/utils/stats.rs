use std::sync;
use std::time;

pub struct Stat {
    name: String,
    value: time::Duration
}

impl Stat {
    pub fn new(name: &str, value: time::Duration) -> Self {
        Stat {
            name: name.to_string(),
            value,
        }
    }
}

pub struct Stats {
    hit_stats: Vec<Stat>,
    sample_stats: Vec<Stat>,
}

impl Stats {
    pub fn new() -> Self {
        Stats {
            hit_stats: vec![],
            sample_stats: vec![],
        }
    }

    pub fn add_hit_stat(&mut self, stat: Stat) {
        self.hit_stats.push(stat);
    }

    pub fn add_sample_stat(&mut self, stat: Stat) {
        self.sample_stats.push(stat);
    }

    pub fn p50_by_name(&self, name: &str) -> (time::Duration, time::Duration) {
        let mut hit_times: Vec<time::Duration> = self
            .hit_stats
            .iter()
            .filter(|s| s.name == name)
            .map(|s| s.value)
            .collect();
        let mut sample_times: Vec<time::Duration> = self
            .sample_stats
            .iter()
            .filter(|s| s.name == name)
            .map(|s| s.value)
            .collect();

        hit_times.sort();
        sample_times.sort();

        let mid = hit_times.len() / 2;
        let p50_hit = hit_times[mid];
        let p50_sample = sample_times[mid];

        (p50_hit, p50_sample)
    }

    pub fn p50(&self) -> (time::Duration, time::Duration) {
        let mut hit_times: Vec<time::Duration> = self.hit_stats.iter().map(|s| s.value).collect();
        let mut sample_times: Vec<time::Duration> =
            self.sample_stats.iter().map(|s| s.value).collect();

        hit_times.sort();
        sample_times.sort();

        let mid = hit_times.len() / 2;
        let p50_hit = hit_times[mid];
        let p50_sample = sample_times[mid];

        (p50_hit, p50_sample)
    }

    pub fn p90(&self) -> (time::Duration, time::Duration) {
        let mut hit_times: Vec<time::Duration> = self.hit_stats.iter().map(|s| s.value).collect();
        let mut sample_times: Vec<time::Duration> =
            self.sample_stats.iter().map(|s| s.value).collect();

        hit_times.sort();
        sample_times.sort();

        let idx = (hit_times.len() as f32 * 0.9).ceil() as usize - 1;
        let p90_hit = hit_times[idx];
        let p90_sample = sample_times[idx];

        (p90_hit, p90_sample)
    }

    pub fn p90_by_name(&self, name: &str) -> (time::Duration, time::Duration) {
        let mut hit_times: Vec<time::Duration> = self
            .hit_stats
            .iter()
            .filter(|s| s.name == name)
            .map(|s| s.value)
            .collect();
        let mut sample_times: Vec<time::Duration> = self
            .sample_stats
            .iter()
            .filter(|s| s.name == name)
            .map(|s| s.value)
            .collect();

        hit_times.sort();
        sample_times.sort();

        let idx = (hit_times.len() as f32 * 0.9).ceil() as usize - 1;
        let p90_hit = hit_times[idx];
        let p90_sample = sample_times[idx];

        (p90_hit, p90_sample)
    }

    pub fn p99(&self) -> (time::Duration, time::Duration) {
        let mut hit_times: Vec<time::Duration> = self.hit_stats.iter().map(|s| s.value).collect();
        let mut sample_times: Vec<time::Duration> =
            self.sample_stats.iter().map(|s| s.value).collect();

        hit_times.sort();
        sample_times.sort();

        let idx = (hit_times.len() as f32 * 0.99).ceil() as usize - 1;
        let p99_hit = hit_times[idx];
        let p99_sample = sample_times[idx];

        (p99_hit, p99_sample)
    }

    pub fn p99_by_name(&self, name: &str) -> (time::Duration, time::Duration) {
        let mut hit_times: Vec<time::Duration> = self
            .hit_stats
            .iter()
            .filter(|s| s.name == name)
            .map(|s| s.value)
            .collect();
        let mut sample_times: Vec<time::Duration> = self
            .sample_stats
            .iter()
            .filter(|s| s.name == name)
            .map(|s| s.value)
            .collect();

        hit_times.sort();
        sample_times.sort();

        let idx = (hit_times.len() as f32 * 0.99).ceil() as usize - 1;
        let p99_hit = hit_times[idx];
        let p99_sample = sample_times[idx];

        (p99_hit, p99_sample)
    }

    pub fn total_hit_time(&self) -> time::Duration {
        self.hit_stats.iter().map(|s| s.value).sum()
    }

    pub fn total_hit_time_by_name(&self, name: &str) -> time::Duration {
        self.hit_stats
            .iter()
            .filter(|s| s.name == name)
            .map(|s| s.value)
            .sum()
    }

    pub fn total_sample_time(&self) -> time::Duration {
        self.sample_stats.iter().map(|s| s.value).sum()
    }

    pub fn total_sample_time_by_name(&self, name: &str) -> time::Duration {
        self.sample_stats
            .iter()
            .filter(|s| s.name == name)
            .map(|s| s.value)
            .sum()
    }

    pub fn total_time(&self) -> time::Duration {
        self.total_hit_time() + self.total_sample_time()
    }

    pub fn total_time_by_name(&self, name: &str) -> time::Duration {
        self.total_hit_time_by_name(name) + self.total_sample_time_by_name(name)
    }

    pub fn total_hits(&self) -> usize {
        self.hit_stats.len()
    }

    pub fn total_samples(&self) -> usize {
        self.sample_stats.len()
    }
}

pub static DIELECTRIC_HIT:  &str = "dielectric_hit";
pub static DIELECTRIC_SAMPLE:  &str = "dielectric_sample";
pub static DIFFUSE_HIT:  &str = "diffuse_hit";
pub static DIFFUSE_SAMPLE:  &str = "diffuse_sample";
pub static METALLIC_HIT:  &str = "metallic_hit";
pub static METALLIC_SAMPLE:  &str = "metallic_sample";
pub static SCENE_HIT:  &str = "scene_hit";
pub static SCENE_SAMPLE:  &str = "scene_sample";

static STATS: sync::LazyLock<sync::Mutex<Stats>> =
    sync::LazyLock::new(|| sync::Mutex::new(Stats::new()));

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
