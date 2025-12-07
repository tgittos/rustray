use std::time;

pub struct Stat {
    hit_time: time::Duration,
    sample_time: time::Duration,
}

impl Stat {
    pub fn new(hit_time: time::Duration, sample_time: time::Duration) -> Self {
        Stat {
            hit_time,
            sample_time,
        }
    }
}

pub struct Stats {
    stats: Vec<Stat>,
    start_time: time::Instant,
}

impl Stats {
    pub fn new() -> Self {
        Stats {
            stats: vec![],
            start_time: time::Instant::now(),
        }
    }

    pub fn add_stat(&mut self, stat: Stat) {
        self.stats.push(stat);
    }

    pub fn p50(&self) -> (time::Duration, time::Duration) {
        let mut hit_times: Vec<time::Duration> = self.stats.iter().map(|s| s.hit_time).collect();
        let mut sample_times: Vec<time::Duration> =
            self.stats.iter().map(|s| s.sample_time).collect();

        hit_times.sort();
        sample_times.sort();

        let mid = hit_times.len() / 2;
        let p50_hit = hit_times[mid];
        let p50_sample = sample_times[mid];

        (p50_hit, p50_sample)
    }

    pub fn p90(&self) -> (time::Duration, time::Duration) {
        let mut hit_times: Vec<time::Duration> = self.stats.iter().map(|s| s.hit_time).collect();
        let mut sample_times: Vec<time::Duration> =
            self.stats.iter().map(|s| s.sample_time).collect();

        hit_times.sort();
        sample_times.sort();

        let idx = (hit_times.len() as f32 * 0.9).ceil() as usize - 1;
        let p90_hit = hit_times[idx];
        let p90_sample = sample_times[idx];

        (p90_hit, p90_sample)
    }

    pub fn p99(&self) -> (time::Duration, time::Duration) {
        let mut hit_times: Vec<time::Duration> = self.stats.iter().map(|s| s.hit_time).collect();
        let mut sample_times: Vec<time::Duration> =
            self.stats.iter().map(|s| s.sample_time).collect();

        hit_times.sort();
        sample_times.sort();

        let idx = (hit_times.len() as f32 * 0.99).ceil() as usize - 1;
        let p99_hit = hit_times[idx];
        let p99_sample = sample_times[idx];

        (p99_hit, p99_sample)
    }

    pub fn total_hit_time(&self) -> time::Duration {
        self.stats.iter().map(|s| s.hit_time).sum()
    }

    pub fn total_sample_time(&self) -> time::Duration {
        self.stats.iter().map(|s| s.sample_time).sum()
    }

    pub fn total_time(&self) -> time::Duration {
        self.start_time.elapsed()
    }
}
