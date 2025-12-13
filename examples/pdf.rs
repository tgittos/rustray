use rand::Rng;
use std::cmp;

struct Sample {
    pub x: f64,
    pub p_x: f64,
}

impl cmp::PartialEq for Sample {
    fn eq(&self, other: &Self) -> bool {
        self.x == other.x
    }
}

fn main() {
    let n = 10000;
    let mut samples: Vec<Sample> = Vec::with_capacity(n);
    let mut sum: f64 = 0.0;

    for _i in 0..n {
        let x = rand::rng().random_range(0.0..2.0 * std::f64::consts::PI);
        let sin_x = x.sin();
        let p_x = (-x / (2.0 * std::f64::consts::PI)).exp() * sin_x * sin_x;
        sum += p_x;
        samples.push(Sample { x, p_x });
    }

    samples.sort_by(|a, b| a.x.partial_cmp(&b.x).unwrap());

    let mut cumulative_probability = 0.0;
    let half_total_probability = samples.iter().map(|s| s.p_x).sum::<f64>() / 2.0;
    let mut x: f64 = 0.0;
    for sample in &samples {
        cumulative_probability += sample.p_x;
        if cumulative_probability >= half_total_probability {
            x = sample.x;
            break;
        }
    }

    println!("avg x: {}", sum / (n as f64));
    println!("total probability: {}", 2.0 * std::f64::consts::PI * (sum / (n as f64)));
    println!("x at half total probability: {}", x);
}