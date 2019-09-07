use {gradient, math};
use math::{Point2, Point3, Point4, Vector2, Vector3, Vector4, interp::*};
use noise_fns::{NoiseFn, Seedable};
use permutationtable::PermutationTable;

/// Noise function that outputs 2/3/4-dimensional Perlin noise.
#[derive(Clone, Copy, Debug)]
pub struct Perlin {
    seed: u32,
    perm_table: PermutationTable,
}

impl Perlin {
    pub const DEFAULT_SEED: u32 = 0;

    pub fn new() -> Self {
        Perlin {
            seed: Self::DEFAULT_SEED,
            perm_table: PermutationTable::new(Self::DEFAULT_SEED),
        }
    }
}

impl Default for Perlin {
    fn default() -> Self {
        Self::new()
    }
}

impl Seedable for Perlin {
    /// Sets the seed value for Perlin noise
    fn set_seed(self, seed: u32) -> Self {
        // If the new seed is the same as the current seed, just return self.
        if self.seed == seed {
            return self;
        }

        // Otherwise, regenerate the permutation table based on the new seed.
        Perlin {
            seed,
            perm_table: PermutationTable::new(seed),
        }
    }

    fn seed(&self) -> u32 {
        self.seed
    }
}

/// 2-dimensional perlin noise
impl NoiseFn<Point2<f64>> for Perlin {
    fn get(&self, point: Point2<f64>) -> f64 {
        #[inline]
        #[cfg_attr(rustfmt, rustfmt_skip)]
        fn gradient_dot_v(perm: usize, point: [f64; 2]) -> f64 {
            let x = point[0];
            let y = point[1];

            match perm & 0b11 {
                0 =>  x + y, // ( 1,  1)
                1 => -x + y, // (-1,  1)
                2 =>  x - y, // ( 1, -1)
                3 => -x - y, // (-1, -1)
                _ => unreachable!(),
            }
        }

        let floored = math::map2(point, f64::floor);
        let near_corner = math::to_isize2(floored);
        let far_corner = math::add2(near_corner, [1; 2]);
        let near_distance = math::sub2(point, floored);
        let far_distance = math::sub2(near_distance, [1.0; 2]);

        let u = s_curve5(near_distance[0]);
        let v = s_curve5(near_distance[1]);

        let a = gradient_dot_v(self.perm_table.get2(near_corner), near_distance);
        let b = gradient_dot_v(
            self.perm_table.get2([far_corner[0], near_corner[1]]),
            [far_distance[0], near_distance[1]],
        );
        let c = gradient_dot_v(
            self.perm_table.get2([near_corner[0], far_corner[1]]),
            [near_distance[0], far_distance[1]],
        );
        let d = gradient_dot_v(self.perm_table.get2(far_corner), far_distance);

        let k0 = a;
        let k1 = b - a;
        let k2 = c - a;
        let k3 = a + d - b - c;

        k0 + k1 * u + k2 * v + k3 * u * v
    }
}

/// 3-dimensional perlin noise
impl NoiseFn<Point3<f64>> for Perlin {
    fn get(&self, point: Point3<f64>) -> f64 {
        #[inline]
        #[cfg_attr(rustfmt, rustfmt_skip)]
        fn gradient_dot_v(perm: usize, point: [f64; 3]) -> f64 {
            let x = point[0];
            let y = point[1];
            let z = point[2];

            match perm & 0b1111 {
                 0 =>  x + y, // ( 1,  1,  0)
                 1 => -x + y, // (-1,  1,  0)
                 2 =>  x - y, // ( 1, -1,  0)
                 3 => -x - y, // (-1, -1,  0)
                 4 =>  x + z, // ( 1,  0,  1)
                 5 => -x + z, // (-1,  0,  1)
                 6 =>  x - z, // ( 1,  0, -1)
                 7 => -x - z, // (-1,  0, -1)
                 8 =>  y + z, // ( 0,  1,  1)
                 9 => -y + z, // ( 0, -1,  1)
                10 =>  y - z, // ( 0,  1, -1)
                11 => -y - z, // ( 0, -1, -1)
                12 =>  x + y, // ( 1,  1,  0)
                13 => -x + y, // (-1,  1,  0)
                14 => -y + z, // ( 0, -1,  1)
                15 => -y - z, // ( 0, -1, -1)
                _ => unreachable!(),
            }
        }

        let floored = math::map3(point, f64::floor);
        let near_corner = math::to_isize3(floored);
        let far_corner = math::add3(near_corner, [1; 3]);
        let near_distance = math::sub3(point, floored);
        let far_distance = math::sub3(near_distance, [1.0; 3]);

        let u = s_curve5(near_distance[0]);
        let v = s_curve5(near_distance[1]);
        let w = s_curve5(near_distance[2]);

        let a = gradient_dot_v(self.perm_table.get3(near_corner), near_distance);
        let b = gradient_dot_v(
            self.perm_table
                .get3([far_corner[0], near_corner[1], near_corner[2]]),
            [far_distance[0], near_distance[1], near_distance[2]],
        );
        let c = gradient_dot_v(
            self.perm_table
                .get3([near_corner[0], far_corner[1], near_corner[2]]),
            [near_distance[0], far_distance[1], near_distance[2]],
        );
        let d = gradient_dot_v(
            self.perm_table
                .get3([far_corner[0], far_corner[1], near_corner[2]]),
            [far_distance[0], far_distance[1], near_distance[2]],
        );
        let e = gradient_dot_v(
            self.perm_table
                .get3([near_corner[0], near_corner[1], far_corner[2]]),
            [near_distance[0], near_distance[1], far_distance[2]],
        );
        let f = gradient_dot_v(
            self.perm_table
                .get3([far_corner[0], near_corner[1], far_corner[2]]),
            [far_distance[0], near_distance[1], far_distance[2]],
        );
        let g = gradient_dot_v(
            self.perm_table
                .get3([near_corner[0], far_corner[1], far_corner[2]]),
            [near_distance[0], far_distance[1], far_distance[2]],
        );
        let h = gradient_dot_v(self.perm_table.get3(far_corner), far_distance);

        let k0 = a;
        let k1 = b - a;
        let k2 = c - a;
        let k3 = e - a;
        let k4 = a + d - b - c;
        let k5 = a + f - b - e;
        let k6 = a + g - c - e;
        let k7 = b + c + e + h - a - d - f - g;

        k0 + k1 * u + k2 * v + k3 * w + k4 * u * v + k5 * u * w + k6 * v * w + k7 * u * v * w
    }
}

/// 4-dimensional perlin noise
impl NoiseFn<Point4<f64>> for Perlin {
    fn get(&self, point: Point4<f64>) -> f64 {
        #[inline(always)]
        fn surflet(
            perm_table: &PermutationTable,
            corner: Point4<isize>,
            distance: Vector4<f64>,
        ) -> f64 {
            let attn = 1.0 - math::dot4(distance, distance);
            if attn > 0.0 {
                attn.powi(4) * math::dot4(distance, gradient::get4(perm_table.get4(corner)))
            } else {
                0.0
            }
        }

        let floored = math::map4(point, f64::floor);
        let near_corner = math::to_isize4(floored);
        let far_corner = math::add4(near_corner, math::one4());
        let near_distance = math::sub4(point, floored);
        let far_distance = math::sub4(near_distance, math::one4());

        let f0000 = surflet(
            &self.perm_table,
            [
                near_corner[0],
                near_corner[1],
                near_corner[2],
                near_corner[3],
            ],
            [
                near_distance[0],
                near_distance[1],
                near_distance[2],
                near_distance[3],
            ],
        );
        let f1000 = surflet(
            &self.perm_table,
            [
                far_corner[0],
                near_corner[1],
                near_corner[2],
                near_corner[3],
            ],
            [
                far_distance[0],
                near_distance[1],
                near_distance[2],
                near_distance[3],
            ],
        );
        let f0100 = surflet(
            &self.perm_table,
            [
                near_corner[0],
                far_corner[1],
                near_corner[2],
                near_corner[3],
            ],
            [
                near_distance[0],
                far_distance[1],
                near_distance[2],
                near_distance[3],
            ],
        );
        let f1100 = surflet(
            &self.perm_table,
            [far_corner[0], far_corner[1], near_corner[2], near_corner[3]],
            [
                far_distance[0],
                far_distance[1],
                near_distance[2],
                near_distance[3],
            ],
        );
        let f0010 = surflet(
            &self.perm_table,
            [
                near_corner[0],
                near_corner[1],
                far_corner[2],
                near_corner[3],
            ],
            [
                near_distance[0],
                near_distance[1],
                far_distance[2],
                near_distance[3],
            ],
        );
        let f1010 = surflet(
            &self.perm_table,
            [far_corner[0], near_corner[1], far_corner[2], near_corner[3]],
            [
                far_distance[0],
                near_distance[1],
                far_distance[2],
                near_distance[3],
            ],
        );
        let f0110 = surflet(
            &self.perm_table,
            [near_corner[0], far_corner[1], far_corner[2], near_corner[3]],
            [
                near_distance[0],
                far_distance[1],
                far_distance[2],
                near_distance[3],
            ],
        );
        let f1110 = surflet(
            &self.perm_table,
            [far_corner[0], far_corner[1], far_corner[2], near_corner[3]],
            [
                far_distance[0],
                far_distance[1],
                far_distance[2],
                near_distance[3],
            ],
        );
        let f0001 = surflet(
            &self.perm_table,
            [
                near_corner[0],
                near_corner[1],
                near_corner[2],
                far_corner[3],
            ],
            [
                near_distance[0],
                near_distance[1],
                near_distance[2],
                far_distance[3],
            ],
        );
        let f1001 = surflet(
            &self.perm_table,
            [far_corner[0], near_corner[1], near_corner[2], far_corner[3]],
            [
                far_distance[0],
                near_distance[1],
                near_distance[2],
                far_distance[3],
            ],
        );
        let f0101 = surflet(
            &self.perm_table,
            [near_corner[0], far_corner[1], near_corner[2], far_corner[3]],
            [
                near_distance[0],
                far_distance[1],
                near_distance[2],
                far_distance[3],
            ],
        );
        let f1101 = surflet(
            &self.perm_table,
            [far_corner[0], far_corner[1], near_corner[2], far_corner[3]],
            [
                far_distance[0],
                far_distance[1],
                near_distance[2],
                far_distance[3],
            ],
        );
        let f0011 = surflet(
            &self.perm_table,
            [near_corner[0], near_corner[1], far_corner[2], far_corner[3]],
            [
                near_distance[0],
                near_distance[1],
                far_distance[2],
                far_distance[3],
            ],
        );
        let f1011 = surflet(
            &self.perm_table,
            [far_corner[0], near_corner[1], far_corner[2], far_corner[3]],
            [
                far_distance[0],
                near_distance[1],
                far_distance[2],
                far_distance[3],
            ],
        );
        let f0111 = surflet(
            &self.perm_table,
            [near_corner[0], far_corner[1], far_corner[2], far_corner[3]],
            [
                near_distance[0],
                far_distance[1],
                far_distance[2],
                far_distance[3],
            ],
        );
        let f1111 = surflet(
            &self.perm_table,
            [far_corner[0], far_corner[1], far_corner[2], far_corner[3]],
            [
                far_distance[0],
                far_distance[1],
                far_distance[2],
                far_distance[3],
            ],
        );

        // Multiply by arbitrary value to scale to -1..1
        math::clamp(
            (f0000 + f1000 + f0100 + f1100 + f0010 + f1010 + f0110 + f1110 + f0001 + f1001 + f0101
                + f1101 + f0011 + f1011 + f0111 + f1111) * 4.424369240215691,
            -1.0,
            1.0,
        )
    }
}
