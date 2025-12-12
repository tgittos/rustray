use serde::{Deserialize, Deserializer, Serialize, Serializer};
use std::ops;

use crate::math::vec;

#[derive(Copy, Clone)]
pub struct Mat3 {
    pub rows: [vec::Vec3; 3],
}

impl Mat3 {
    pub fn new(rows: [vec::Vec3; 3]) -> Self {
        Mat3 { rows }
    }

    pub fn transpose(&self) -> Mat3 {
        let mut cols = [vec::Vec3::new(0.0, 0.0, 0.0); 3];
        for i in 0..3 {
            for j in 0..3 {
                cols[i][j] = self.rows[j][i];
            }
        }
        Mat3 { rows: cols }
    }
}

impl ops::Mul<vec::Vec3> for &Mat3 {
    type Output = vec::Vec3;

    fn mul(self, rhs: vec::Vec3) -> vec::Vec3 {
        vec::Vec3::new(
            self.rows[0].dot(&rhs),
            self.rows[1].dot(&rhs),
            self.rows[2].dot(&rhs),
        )
    }
}

impl ops::Mul<vec::Vec3> for Mat3 {
    type Output = vec::Vec3;

    fn mul(self, rhs: vec::Vec3) -> vec::Vec3 {
        vec::Vec3::new(
            self.rows[0].dot(&rhs),
            self.rows[1].dot(&rhs),
            self.rows[2].dot(&rhs),
        )
    }
}

impl ops::Mul<Mat3> for Mat3 {
    type Output = Mat3;

    fn mul(self, rhs: Mat3) -> Mat3 {
        let mut result_rows = [vec::Vec3::new(0.0, 0.0, 0.0); 3];
        let rhs_t = rhs.transpose();
        for i in 0..3 {
            for j in 0..3 {
                result_rows[i][j] = self.rows[i].dot(&rhs_t.rows[j]);
            }
        }
        Mat3 { rows: result_rows }
    }
}

impl Serialize for Mat3 {
    fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error>
    where
        S: Serializer,
    {
        self.rows.serialize(serializer)
    }
}

impl<'de> Deserialize<'de> for Mat3 {
    fn deserialize<D>(deserializer: D) -> Result<Self, D::Error>
    where
        D: Deserializer<'de>,
    {
        let rows: [[f32; 3]; 3] = <[[f32; 3]; 3]>::deserialize(deserializer)?;
        Ok(Mat3 {
            rows: rows.map(|row| vec::Vec3::new(row[0], row[1], row[2])),
        })
    }
}

impl ops::Mul<Mat3> for &Mat3 {
    type Output = Mat3;

    fn mul(self, rhs: Mat3) -> Mat3 {
        let mut result_rows = [vec::Vec3::new(0.0, 0.0, 0.0); 3];
        let rhs_t = rhs.transpose();
        for i in 0..3 {
            for j in 0..3 {
                result_rows[i][j] = self.rows[i].dot(&rhs_t.rows[j]);
            }
        }
        Mat3 { rows: result_rows }
    }
}

impl ops::Mul<&Mat3> for Mat3 {
    type Output = Mat3;

    fn mul(self, rhs: &Mat3) -> Mat3 {
        let mut result_rows = [vec::Vec3::new(0.0, 0.0, 0.0); 3];
        let rhs_t = rhs.transpose();
        for i in 0..3 {
            for j in 0..3 {
                result_rows[i][j] = self.rows[i].dot(&rhs_t.rows[j]);
            }
        }
        Mat3 { rows: result_rows }
    }
}

impl ops::Mul<&Mat3> for &Mat3 {
    type Output = Mat3;

    fn mul(self, rhs: &Mat3) -> Mat3 {
        let mut result_rows = [vec::Vec3::new(0.0, 0.0, 0.0); 3];
        let rhs_t = rhs.transpose();
        for i in 0..3 {
            for j in 0..3 {
                result_rows[i][j] = self.rows[i].dot(&rhs_t.rows[j]);
            }
        }
        Mat3 { rows: result_rows }
    }
}
