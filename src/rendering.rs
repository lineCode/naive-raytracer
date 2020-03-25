use crate::math::{Point, Vector3};
use crate::scene::{Scene, Color};

use std::f64;

use image::{DynamicImage, Rgba, RgbaImage, ImageBuffer};

pub struct Ray {
    pub origin: Point,
    pub direction: Vector3,
}

impl Ray {
    /// 坐标系是z向外，x向右，y向上。是个右手系。
    /// 相机放在z=0处，朝负z方向看；胶片在-1.0处摆放，东西都放到负z那边去
    /// 所以这里的射线的x和y就是从原点出发到胶片的某个像素的中心，z都是-1.0
    /// y这里反一下是因为image的y是朝下的，我们是y朝上
    pub fn new_prime(x: u32, y: u32, scene: &Scene) -> Self {
        assert!(scene.width > scene.height);
        let aspect_ratio = (scene.width as f64) / (scene.height as f64);
        let fov_adjustment = (scene.fov.to_radians() / 2.0).tan();
        let sensor_x = (((x as f64 + 0.5) / scene.width as f64) * 2.0 - 1.0) * aspect_ratio * fov_adjustment;
        let sensor_y = - (((y as f64 + 0.5) / scene.height as f64) * 2.0 - 1.0) * fov_adjustment;

        Self {
            origin: Point::zero(),
            direction: Vector3 {
                x: sensor_x,
                y: sensor_y,
                z: -1.0,
            }
            .normalize()
        }
    }
}

pub trait Intersectable {
    fn intersect(&self, ray: &Ray) -> Option<f64>;
    fn get_color(&self) -> Color;
}

pub struct Intersection<'a> {
    pub distance: f64,
    pub item: &'a Box<dyn Intersectable>,
}

impl<'a> Intersection<'a> {
    pub fn new<'b> (distance: f64, item: &'b Box<dyn Intersectable>) -> Intersection<'b> {
        Intersection { distance, item }
    }
}

pub fn cast_ray<'a>(scene: &'a Scene, ray: &Ray) -> Option<Intersection<'a>>
{
    scene.items.iter()
        .filter_map(|i| i.intersect(ray).map(|d| Intersection::new(d, i)))
        .min_by(|i1, i2| i1.distance.partial_cmp(&i2.distance).unwrap())
}

pub fn render(scene: &Scene) -> DynamicImage {
    let black = Rgba::from([0, 0, 0, 0]);
    let mut image = ImageBuffer::from_fn(scene.width, scene.height, |x, y| {
        let ray = Ray::new_prime(x, y, scene);
        if let Some(intersection) = cast_ray(scene, &ray) {
            Rgba::from(intersection.item.get_color().to_rgba8())
        }
        else {
            Rgba::from([0,0,0,0])
        }
    });
    DynamicImage::ImageRgba8(image)
}

