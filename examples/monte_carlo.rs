/*
#include "rtweekend.h"

#include <iostream>
#include <iomanip>

int main() {
    std::cout << std::fixed << std::setprecision(12);

    int inside_circle = 0;
    int N = 100000;

    for (int i = 0; i < N; i++) {
        auto x = random_double(-1,1);
        auto y = random_double(-1,1);
        if (x*x + y*y < 1)
            inside_circle++;
    }

    std::cout << "Estimate of Pi = " << (4.0 * inside_circle) / N << '\n';
}
*/

use rand::Rng;

fn main() {
    let mut rng = rand::rng();

    let mut inside_circle = 0;
    let mut inside_circle_strat = 0;
    let sqrt_n = 1000;

    for i in 0..sqrt_n {
        for j in 0..sqrt_n {
            let x: f64 = rng.random_range(-1.0..1.0);
            let y: f64 = rng.random_range(-1.0..1.0);
            if x * x + y * y < 1.0 {
                inside_circle += 1;
            }

            // Stratified sampling
            let xs = 2.0 * (i as f64 + rng.random::<f64>()) / (sqrt_n as f64) - 1.0;
            let ys = 2.0 * (j as f64 + rng.random::<f64>()) / (sqrt_n as f64) - 1.0;
            if xs * xs + ys * ys < 1.0 {
                inside_circle_strat += 1;
            }
        }
    }

    let pi_estimate = 4.0 * (inside_circle as f32) / ((sqrt_n * sqrt_n) as f32);
    println!("Estimate of Pi = {}", pi_estimate);

    let pi_estimate_strat = 4.0 * (inside_circle_strat as f64) / ((sqrt_n * sqrt_n) as f64);
    println!(
        "Estimate of Pi with stratified sampling = {}",
        pi_estimate_strat
    );
}
