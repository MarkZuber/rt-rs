use std::fs::File;
use std::io::BufRead;
use std::io::BufReader;
use std::sync::Arc;

use crate::cameras::{NormalCamera, ThreadCamera};
use crate::hitables::*;
use crate::materials::*;
use crate::render::*;
use crate::textures::*;
use crate::{vec3, InnerSpace, Vector3};

pub struct NffParser {
    background: Color,
    render_config: RenderConfig,
    camera: ThreadCamera,

    hitables: Vec<ThreadHitable>,
    materials: Arc<Box<CompiledMaterials>>,
    light_hitable: ThreadHitable,
}

impl NffParser {
    pub fn new(
        file_path: &str,
        render_config: &RenderConfig,
    ) -> Arc<Box<dyn SceneGenerator + Send>> {
        Arc::new(Box::new(parse_nff_file(file_path, render_config)))
    }
}

impl SceneGenerator for NffParser {
    fn get_scene(&self) -> Scene {
        create_scene(
            &self.hitables,
            self.materials.clone(),
            &self.light_hitable,
            self.get_background_color(),
            true,
        )
    }

    fn get_camera(&self) -> ThreadCamera {
        self.camera.clone()
    }

    fn get_render_config(&self) -> RenderConfig {
        self.render_config.clone()
    }

    fn get_background_color(&self) -> Color {
        self.background
    }
}

fn as_f32(s: &str) -> f32 {
    s.parse::<f32>().unwrap()
}

fn as_u32(s: &str) -> u32 {
    s.parse::<u32>().unwrap()
}

enum LookingFor {
    Instruction,
    ViewpointFrom,
    ViewpointAt,
    ViewpointUp,
    ViewpointAngle,
    ViewpointHither,
    ViewpointResolution,
    Polygon,
}

// see: http://www.fileformat.info/format/nff/egff.htm
fn parse_nff_file(file_path: &str, render_config: &RenderConfig) -> NffParser {
    let mut camera_from = vec3(0.0, 0.0, 0.0);
    let mut camera_at = vec3(0.0, 0.0, 0.0);
    let mut camera_up = vec3(0.0, 0.0, 0.0);
    let mut image_width = render_config.width;
    let mut image_height = render_config.height;

    let mut background = Color::zero();

    let mut looking_for = LookingFor::Instruction;

    let mut current_item_counter = 0;
    let mut poly_vectors: Vec<Vector3<f32>> = Vec::new();
    let mut materials: CompiledMaterials = CompiledMaterials::new();
    let mut hitables = vec![];
    let mut current_material_id: u64 = 0;
    let mut lights = vec![];

    let f = File::open(file_path).unwrap();
    let file = BufReader::new(&f);
    for (_num, line) in file.lines().enumerate() {
        let l = line.unwrap();

        match looking_for {
            LookingFor::Instruction => {
                let vec: Vec<&str> = l.split_whitespace().collect();
                if vec.len() > 0 {
                    let instruction = vec[0];

                    match instruction {
                        "b" => {
                            // background color
                            background = Color::new(as_f32(vec[1]), as_f32(vec[2]), as_f32(vec[3]));
                        }
                        "v" => {
                            // viewpoint location
                            looking_for = LookingFor::ViewpointFrom;
                        }
                        "l" => {
                            // positional light
                            let light_color = if vec.len() == 7 {
                                Color::new(as_f32(vec[4]), as_f32(vec[5]), as_f32(vec[6]))
                            } else {
                                Color::new(1.0, 1.0, 1.0)
                            };
                            let light_matid = materials.add(DiffuseLight::new(ColorTexture::new(
                                light_color.r(),
                                light_color.g(),
                                light_color.b(),
                            )));
                            lights.push(Sphere::new(
                                vec3(as_f32(vec[1]), as_f32(vec[2]), as_f32(vec[3])),
                                0.01,
                                light_matid,
                            ));
                            hitables.push(Sphere::new(
                                vec3(as_f32(vec[1]), as_f32(vec[2]), as_f32(vec[3])),
                                0.01,
                                light_matid,
                            ));
                        }
                        "f" => {
                            // object material properties
                            // "f" red green blue Kd Ks Shine T index_of_refraction
                            // Kd Diffuse component
                            // Ks Specular
                            // Shine Phong cosine power for highlights
                            // T Transmittance (fraction of contribution of the transmitting ray).
                            // Usually, 0 <= Kd <= 1 and 0 <= Ks <= 1, though it is not required that Kd + Ks = 1. Note that transmitting objects (T > 0) are considered to have two sides for algorithms that need these (normally, objects have one side).
                            // todo: i don't think i'm assigning the correct values into my solidmaterial yet
                            current_material_id = materials.add(LambertianMaterial::new(
                                ColorTexture::new(as_f32(vec[1]), as_f32(vec[2]), as_f32(vec[3])),
                            ));
                            // current_material = SolidMaterial::new(
                            //     as_f64(vec[6]),
                            //     as_f64(vec[5]),
                            //     as_f64(vec[8]),
                            //     as_f64(vec[7]),
                            //     ColorVector::new(as_f64(vec[1]), as_f64(vec[2]), as_f64(vec[3])),
                            // );
                        }
                        "c" => {
                            // cone or cylinder
                        }
                        "s" => {
                            // sphere
                            hitables.push(Sphere::new(
                                vec3(as_f32(vec[1]), as_f32(vec[2]), as_f32(vec[3])),
                                as_f32(vec[4]),
                                current_material_id,
                            ));
                        }
                        "p" => {
                            // polygon
                            current_item_counter = as_u32(vec[1]);
                            poly_vectors = Vec::new();
                            looking_for = LookingFor::Polygon;
                        }
                        "pp" => {
                            // polygon patch
                        }
                        "#" => {
                            // comment
                        }
                        _ => {
                            // unknown
                        }
                    };
                }
            }
            LookingFor::Polygon => {
                if current_item_counter > 0 {
                    current_item_counter = current_item_counter - 1;
                    let vec: Vec<&str> = l.split_whitespace().collect();
                    poly_vectors.push(vec3(as_f32(vec[0]), as_f32(vec[1]), as_f32(vec[2])));
                }

                if current_item_counter == 0 {
                    if poly_vectors.len() >= 3 {
                        let first_vert = poly_vectors[0];
                        let mut prev_vert = poly_vectors[1];
                        let mut this_vert = poly_vectors[2];

                        hitables.push(Triangle::new(
                            vec![first_vert, prev_vert, this_vert],
                            current_material_id,
                        ));

                        for i in 3..poly_vectors.len() {
                            prev_vert = this_vert;
                            this_vert = poly_vectors[i];

                            hitables.push(Triangle::new(
                                vec![first_vert, prev_vert, this_vert],
                                current_material_id,
                            ));
                        }
                    }

                    looking_for = LookingFor::Instruction;
                }
            }
            LookingFor::ViewpointFrom => {
                let vec: Vec<&str> = l.split_whitespace().collect();
                camera_from = vec3(as_f32(vec[1]), as_f32(vec[2]), as_f32(vec[3]));
                looking_for = LookingFor::ViewpointAt;
            }
            LookingFor::ViewpointAt => {
                let vec: Vec<&str> = l.split_whitespace().collect();
                camera_at = vec3(as_f32(vec[1]), as_f32(vec[2]), as_f32(vec[3]));
                looking_for = LookingFor::ViewpointUp;
            }
            LookingFor::ViewpointUp => {
                let vec: Vec<&str> = l.split_whitespace().collect();
                camera_up = vec3(as_f32(vec[1]), as_f32(vec[2]), as_f32(vec[3]));
                looking_for = LookingFor::ViewpointAngle;
            }
            LookingFor::ViewpointAngle => {
                looking_for = LookingFor::ViewpointHither;
            }
            LookingFor::ViewpointHither => {
                looking_for = LookingFor::ViewpointResolution;
            }
            LookingFor::ViewpointResolution => {
                let vec: Vec<&str> = l.split_whitespace().collect();
                image_width = as_u32(vec[1]);
                image_height = as_u32(vec[2]);
                looking_for = LookingFor::Instruction;
            }
        }
    }

    if image_width == 0 {
        image_width = 100;
    }
    if image_height == 0 {
        image_height = 100
    }

    let vertical_fov = 50.0;
    let aspect = image_width as f32 / image_height as f32;
    let aperture = 0.0;
    let focus_distance = (camera_from - camera_at).magnitude();

    let light_hitable = HitableList::from_vec(lights);

    NffParser {
        background,
        hitables,
        materials: Arc::new(Box::new(materials)),
        light_hitable,
        render_config: RenderConfig::new(
            image_width,
            image_height,
            render_config.ray_trace_depth,
            render_config.num_samples,
        ),
        camera: Arc::new(Box::new(NormalCamera::new(
            camera_from,
            camera_at,
            camera_up,
            vertical_fov,
            aspect,
            aperture,
            focus_distance,
        ))),
    }
}
