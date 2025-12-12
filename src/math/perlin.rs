use rand::{Rng, rngs::ThreadRng};

use crate::math::vec;

const POINT_COUNT: usize = 256;

#[derive(Default)]
pub struct PerlinGenerator {
    rand_vectors: Vec<vec::Vec3>,
    perm_x: Vec<usize>,
    perm_y: Vec<usize>,
    perm_z: Vec<usize>,
}

fn random_unit_vectors(rng: &mut ThreadRng) -> Vec<vec::Vec3> {
    (0..POINT_COUNT)
        .map(|_| {
            let mut v = vec::random_in_unit_sphere(rng);
            while v.squared_length() == 0.0 {
                v = vec::random_in_unit_sphere(rng);
            }
            v.normalize()
        })
        .collect()
}

fn generate_permutation(rng: &mut ThreadRng) -> Vec<usize> {
    let mut p: Vec<usize> = (0..POINT_COUNT).collect();
    for i in (1..POINT_COUNT).rev() {
        let target = rng.random_range(0..=i);
        p.swap(i, target);
    }
    p
}

fn perlin_interp(c: &[[[vec::Vec3; 2]; 2]; 2], u: f32, v: f32, w: f32) -> f32 {
    let uu = u * u * (3.0 - 2.0 * u);
    let vv = v * v * (3.0 - 2.0 * v);
    let ww = w * w * (3.0 - 2.0 * w);
    let mut accum = 0.0;

    for i in 0..2 {
        for j in 0..2 {
            for k in 0..2 {
                let weight = vec::Vec3::new(u - i as f32, v - j as f32, w - k as f32);
                let influence = c[i][j][k].dot(&weight);
                let i_blend = i as f32 * uu + (1.0 - i as f32) * (1.0 - uu);
                let j_blend = j as f32 * vv + (1.0 - j as f32) * (1.0 - vv);
                let k_blend = k as f32 * ww + (1.0 - k as f32) * (1.0 - ww);
                accum += i_blend * j_blend * k_blend * influence;
            }
        }
    }

    accum
}

impl PerlinGenerator {
    pub fn new(rng: &mut ThreadRng) -> Self {
        Self {
            rand_vectors: random_unit_vectors(rng),
            perm_x: generate_permutation(rng),
            perm_y: generate_permutation(rng),
            perm_z: generate_permutation(rng),
        }
    }

    pub fn noise(&self, point: vec::Point3) -> f32 {
        let u = point.x - point.x.floor();
        let v = point.y - point.y.floor();
        let w = point.z - point.z.floor();
        let i = point.x.floor() as isize;
        let j = point.y.floor() as isize;
        let k = point.z.floor() as isize;

        let mut c = [[[vec::Vec3::new(0.0, 0.0, 0.0); 2]; 2]; 2];
        for di in 0..2 {
            for dj in 0..2 {
                for dk in 0..2 {
                    let idx = self.perm_x
                        [((i + di as isize) & (POINT_COUNT as isize - 1)) as usize]
                        ^ self.perm_y[((j + dj as isize) & (POINT_COUNT as isize - 1)) as usize]
                        ^ self.perm_z[((k + dk as isize) & (POINT_COUNT as isize - 1)) as usize];
                    c[di][dj][dk] = self.rand_vectors[idx];
                }
            }
        }

        perlin_interp(&c, u, v, w)
    }

    pub fn turbulence(&self, point: vec::Point3, depth: usize) -> f32 {
        let mut accum = 0.0;
        let mut temp_point = point;
        let mut weight = 1.0;

        for _ in 0..depth {
            accum += weight * self.noise(temp_point);
            weight *= 0.5;
            temp_point = temp_point * 2.0;
        }

        accum.abs()
    }
}
