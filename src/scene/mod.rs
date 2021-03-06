pub mod item;
pub mod light;
pub mod material;

use crate::rendering::{Intersectable, Light};

pub type Distance = f64;

pub struct Scene {
    pub width: u32,
    pub height: u32,
    pub fov: Distance,
    pub items: Vec<Box<dyn Intersectable + Send + Sync>>,
    pub lights: Vec<Box<dyn Light + Send + Sync>>,
}
