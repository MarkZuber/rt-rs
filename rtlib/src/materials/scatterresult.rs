use crate::pdfs::Pdf;
use crate::render::Color;
use crate::render::Ray;
use std::sync::Arc;

pub struct ScatterResult {
    is_scattered: bool,
    specular_ray: Option<Ray>,
    attenuation: Color,
    pdf: Option<Arc<dyn Pdf>>,
}

impl ScatterResult {
    pub fn new(
        is_scattered: bool,
        attenuation: Color,
        specular_ray: Option<Ray>,
        pdf: Option<Arc<dyn Pdf>>,
    ) -> ScatterResult {
        ScatterResult {
            is_scattered,
            attenuation,
            specular_ray,
            pdf,
        }
    }

    pub fn new_false() -> ScatterResult {
        ScatterResult::new(false, Color::zero(), None, None)
    }

    pub fn is_scattered(&self) -> bool {
        self.is_scattered
    }

    pub fn get_specular_ray(&self) -> Option<Ray> {
        self.specular_ray
    }

    pub fn get_attenuation(&self) -> Color {
        self.attenuation
    }

    pub fn get_pdf(&self) -> Option<Arc<dyn Pdf>> {
        self.pdf.clone()
    }
}
