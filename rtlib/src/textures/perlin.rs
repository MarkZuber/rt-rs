use crate::next_rand_f32;
use crate::Vector3;

struct PerlinStaticData {
    pub ran_float: Vec<f32>,
    pub perm_x: Vec<usize>,
    pub perm_y: Vec<usize>,
    pub perm_z: Vec<usize>,
}

fn perlin_generate() -> Vec<f32> {
    vec![next_rand_f32(); 256]
}

fn perlin_generate_perm() -> Vec<usize> {
    let mut p = vec![0_usize; 256];
    for i in 0..p.len() - 1 {
        p[i] = i;
    }

    permute(&mut p);
    p
}

fn permute(p: &mut [usize]) {
    for i in (1..p.len() - 1).rev() {
        let target = (next_rand_f32() * ((i + 1) as f32)) as usize;
        p.swap(i, target)
    }
}

lazy_static! {
    static ref PERLIN_DATA: PerlinStaticData = {
        PerlinStaticData {
            ran_float: perlin_generate(),
            perm_x: perlin_generate_perm(),
            perm_y: perlin_generate_perm(),
            perm_z: perlin_generate_perm(),
        }
    };
}

pub fn perlin_noise(p: Vector3<f32>, interpolate: bool) -> f32 {
    let mut u = p.x - p.y.floor();
    let mut v = p.y - p.y.floor();
    let mut w = p.z - p.z.floor();

    if interpolate {
        let i = p.x.floor() as i32;
        let j = p.y.floor() as i32;
        let k = p.z.floor() as i32;

        u = u * u * (3.0 - (2.0 * u));
        v = v * v * (3.0 - (2.0 * v));
        w = w * w * (3.0 - (2.0 * w));

        let mut o = [
            [[0.0, 0.0, 0.0], [0.0, 0.0, 0.0], [0.0, 0.0, 0.0]],
            [[0.0, 0.0, 0.0], [0.0, 0.0, 0.0], [0.0, 0.0, 0.0]],
            [[0.0, 0.0, 0.0], [0.0, 0.0, 0.0], [0.0, 0.0, 0.0]],
        ];

        for di in 0..2 {
            for dj in 0..2 {
                for dk in 0..2 {
                    o[di][dj][dk] = PERLIN_DATA.ran_float[(PERLIN_DATA.perm_x
                        [((i + di as i32) & 255) as usize])
                        ^ PERLIN_DATA.perm_y[((j + dj as i32) & 255) as usize]
                        ^ PERLIN_DATA.perm_z[((k + dk as i32) & 255) as usize]];
                }
            }
        }

        trilinear_interpolate(&o, u, v, w)
    } else {
        let i = ((4.0 * p.x) as usize) & 255;
        let j = ((4.0 * p.y) as usize) & 255;
        let k = ((4.0 * p.z) as usize) & 255;

        PERLIN_DATA.ran_float[PERLIN_DATA.perm_x[i] ^ PERLIN_DATA.perm_y[j] ^ PERLIN_DATA.perm_z[k]]
    }
}

fn trilinear_interpolate(o: &[[[f32; 3]; 3]; 3], u: f32, v: f32, w: f32) -> f32 {
    let mut accum = 0.0_f32;
    for i in 0..2 {
        let dubi = i as f32;
        for j in 0..2 {
            let dubj = j as f32;
            for k in 0..2 {
                let dubk = k as f32;
                accum += ((dubi * u) + ((1.0 - dubi) * (1.0 - u)))
                    * ((dubj * v) + ((1.0 - dubj) * (1.0 - v)))
                    * ((dubk * w) + ((1.0 - dubk) * (1.0 - w)))
                    * o[i][j][k];
            }
        }
    }

    accum
}
