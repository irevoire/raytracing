use crate::{write_color, Color, HitRecord, Hittable, Interval, Point3, Ray, Vec3};

#[derive(Default)]
pub struct Camera {
    pub aspect_ratio: f64,  // Ratio of image width over height
    pub image_width: usize, // Rendered image width in pixel count
    image_height: usize,    // Rendered image height
    center: Point3,         // Camera center
    pixel00_loc: Point3,    // Location of pixel 0, 0
    pixel_delta_u: Vec3,    // Offset to pixel to the right
    pixel_delta_v: Vec3,    // Offset to pixel below
}

impl Camera {
    pub fn new() -> Self {
        Self::default()
    }

    pub fn render(&mut self, world: &impl Hittable) {
        self.initialize();

        println!("P3");
        println!("{} {}", self.image_width, self.image_height);
        println!("255");

        for j in 0..self.image_height {
            eprintln!("Scanline remaining: {}", self.image_height - j);
            for i in 0..self.image_width {
                let pixel_center = self.pixel00_loc
                    + (i as f64 * self.pixel_delta_u)
                    + (j as f64 * self.pixel_delta_v);
                let ray_direction = pixel_center - self.center;
                let r = Ray::new(self.center, ray_direction);
                let color = self.ray_color(&r, world);
                write_color(color);
            }
        }
        eprintln!("done");
    }

    fn initialize(&mut self) {
        self.image_height = (self.image_width as f64 / self.aspect_ratio as f64).ceil() as usize;

        self.center = Point3::from(0, 0, 0);

        // Determine viewport dimensions.
        let focal_length = 1.0;
        let viewport_height = 2.0;
        let viewport_width = viewport_height * (self.image_width as f64 / self.image_height as f64);

        // Calculate the vectors across the horizontal and down the vertical viewport edges.
        let viewport_u = Vec3::from(viewport_width, 0, 0);
        let viewport_v = Vec3::from(0, -viewport_height, 0);

        // Calculate the horizontal and vertical delta vectors from pixel to pixel.
        self.pixel_delta_u = viewport_u / self.image_width as f64;
        self.pixel_delta_v = viewport_v / self.image_height as f64;

        // Calculate the location of the upper left pixel.
        let viewport_upper_left =
            self.center - Vec3::from(0, 0, focal_length) - viewport_u / 2. - viewport_v / 2.;
        self.pixel00_loc = viewport_upper_left + 0.5 * (self.pixel_delta_u + self.pixel_delta_v);
    }

    fn ray_color(&self, r: &Ray, world: &impl Hittable) -> Color {
        let mut rec = HitRecord::default();
        if world.hit(r, Interval::from(0., f64::INFINITY), &mut rec) {
            return 0.5 * (rec.normal + Color::from(1, 1, 1));
        }

        let unit_direction = r.direction().unit_vector();
        let a = 0.5 * unit_direction.y() + 1.0;
        (1.0 - a) * Color::from(1.0, 1.0, 1.0) + a * Color::from(0.5, 0.7, 1.0)
    }
}
