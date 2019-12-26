use crate::next_rand_f32;
use crate::{to_unit_vector, vec3, InnerSpace, Vector3};

struct VectorPerlinStaticData {
    pub ran_vector: Vec<Vector3<f32>>,
    pub perm_x: Vec<usize>,
    pub perm_y: Vec<usize>,
    pub perm_z: Vec<usize>,
}

fn vector_perlin_generate() -> Vec<Vector3<f32>> {
    let mut p = vec![vec3(0.0, 0.0, 0.0); 256];
    for i in 0..p.len() - 1 {
        p[i] = to_unit_vector(vec3(
            -1.0 + (2.0 * next_rand_f32()),
            -1.0 + (2.0 * next_rand_f32()),
            -1.0 + (2.0 * next_rand_f32()),
        ));
    }

    p
}

fn vector_perlin_generate_perm() -> Vec<usize> {
    let mut p = vec![0_usize; 256];
    for i in 0..p.len() - 1 {
        p[i] = i;
    }

    vector_perlin_permute(&mut p);
    p
}

fn vector_perlin_permute(p: &mut [usize]) {
    for i in (1..p.len() - 1).rev() {
        let target = (next_rand_f32() * ((i + 1) as f32)) as usize;
        p.swap(i, target)
    }
}

lazy_static! {
    static ref VECTOR_PERLIN_DATA: VectorPerlinStaticData = {
        VectorPerlinStaticData {
            ran_vector: vector_perlin_generate(),
            perm_x: vector_perlin_generate_perm(),
            perm_y: vector_perlin_generate_perm(),
            perm_z: vector_perlin_generate_perm(),
        }
    };
}

fn vector_perlin_interpolate(c: &[[[Vector3<f32>; 3]; 3]; 3], u: f32, v: f32, w: f32) -> f32 {
    let uu = u * u * (3.0 - (2.0 * u));
    let vv = v * v * (3.0 - (2.0 * v));
    let ww = w * w * (3.0 - (2.0 * w));
    let mut accum = 0.0;
    for i in 0..2 {
        let dubi = i as f32;
        for j in 0..2 {
            let dubj = j as f32;
            for k in 0..2 {
                let dubk = k as f32;
                let weight_vec = vec3(u - dubi, v - dubj, w - dubk);
                accum += ((dubi * uu) + ((1.0 - dubi) * (1.0 - uu)))
                    * ((dubj * vv) + ((1.0 - dubj) * (1.0 - vv)))
                    * ((dubk * ww) + ((1.0 - dubk) * (1.0 - ww)))
                    * c[i][j][k].dot(weight_vec);
            }
        }
    }

    accum
}

pub fn vector_perlin_noise(p: Vector3<f32>) -> f32 {
    let u = p.x - p.x.floor();
    let v = p.y - p.y.floor();
    let w = p.z - p.z.floor();
    let i = p.x.floor() as i32;
    let j = p.y.floor() as i32;
    let k = p.z.floor() as i32;

    let zerovec = vec3(0.0, 0.0, 0.0);

    let mut c = [
        [
            [zerovec, zerovec, zerovec],
            [zerovec, zerovec, zerovec],
            [zerovec, zerovec, zerovec],
        ],
        [
            [zerovec, zerovec, zerovec],
            [zerovec, zerovec, zerovec],
            [zerovec, zerovec, zerovec],
        ],
        [
            [zerovec, zerovec, zerovec],
            [zerovec, zerovec, zerovec],
            [zerovec, zerovec, zerovec],
        ],
    ];

    for di in 0..2 {
        for dj in 0..2 {
            for dk in 0..2 {
                let val1 = VECTOR_PERLIN_DATA.perm_x[((i + di) & 255) as usize];
                //
                //
                let val2 = VECTOR_PERLIN_DATA.perm_y[((j + dj) & 255) as usize];
                //
                //
                //
                let val3 = VECTOR_PERLIN_DATA.perm_z[((k + dk) & 255) as usize];
                //
                //
                //
                let val = VECTOR_PERLIN_DATA.ran_vector[val1 ^ val2 ^ val3];

                c[di as usize][dj as usize][dk as usize] = val;
            }
        }
    }

    vector_perlin_interpolate(&c, u, v, w)
}

pub fn vector_perlin_turbulence(p: Vector3<f32>) -> f32 {
    vector_perlin_turbulence_with_depth(p, 7)
}

pub fn vector_perlin_turbulence_with_depth(p: Vector3<f32>, depth: i32) -> f32 {
    let mut accum = 0.0;
    let mut temp_p = p;
    let mut weight = 1.0;

    for _ in 0..depth {
        accum += weight * vector_perlin_noise(temp_p);
        weight *= 0.5;
        temp_p *= 2.0;
    }

    accum.abs()
}
