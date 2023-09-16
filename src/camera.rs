use rand::Rng;

use crate::{write_color, Color, HitRecord, Hittable, Interval, Point3, Ray, Vec3};

#[derive(Default)]
pub struct Camera {
    pub aspect_ratio: f64,        // Ratio of image width over height
    pub image_width: usize,       // Rendered image width in pixel count
    pub samples_per_pixel: usize, // Count of random samples for each pixel
    pub max_depth: usize,         // Maximum number of ray bounces into scene
    pub vfov: f64,                // Vertical view angle (field of view)
    pub lookfrom: Point3,         // Point camera is looking from
    pub lookat: Point3,           // Point camera is looking at
    pub vup: Vec3,                // Camera-relative "up" direction
    pub defocus_angle: f64,       // Variation angle of rays through each pixel
    pub focus_dist: f64,          // Distance from camera lookfrom point to plane of perfect focus

    image_height: usize, // Rendered image height
    center: Point3,      // Camera center
    pixel00_loc: Point3, // Location of pixel 0, 0
    pixel_delta_u: Vec3, // Offset to pixel to the right
    pixel_delta_v: Vec3, // Offset to pixel below
    u: Vec3,             // Camera frame basis vectors
    v: Vec3,             // Camera frame basis vectors
    w: Vec3,             // Camera frame basis vectors

    defocus_disk_u: Vec3, // Defocus disk horizontal radius
    defocus_disk_v: Vec3, // Defocus disk vertical radius
}

impl Camera {
    pub fn new() -> Self {
        Self {
            aspect_ratio: 16. / 9.,
            image_width: 800,
            samples_per_pixel: 10,
            max_depth: 50,
            vfov: 90.,
            defocus_angle: 0.,
            focus_dist: 10.,
            lookfrom: Point3::from(0, 0, -1),
            lookat: Point3::from(0, 0, 0),
            vup: Vec3::from(0, 1, 0),
            ..Default::default()
        }
    }

    pub fn render(&mut self, world: &impl Hittable) {
        self.initialize();

        println!("P3");
        println!("{} {}", self.image_width, self.image_height);
        println!("255");

        for j in 0..self.image_height {
            eprintln!("Scanline remaining: {}", self.image_height - j);
            for i in 0..self.image_width {
                let mut pixel_color = Color::from(0, 0, 0);
                for _sample in 0..self.samples_per_pixel {
                    let r = self.get_ray(i, j);
                    pixel_color += self.ray_color(&r, self.max_depth, world);
                }
                write_color(pixel_color, self.samples_per_pixel);
            }
        }
        eprintln!("done");
    }

    fn initialize(&mut self) {
        self.image_height = (self.image_width as f64 / self.aspect_ratio as f64).ceil() as usize;

        self.center = self.lookfrom;

        // Determine viewport dimensions.
        let theta = self.vfov.to_radians();
        let h = (theta / 2.).tan();
        let viewport_height = 2. * h * self.focus_dist;

        let viewport_width = viewport_height * (self.image_width as f64 / self.image_height as f64);

        // Calculate the u,v,w unit basis vectors for the camera coordinate frame.
        self.w = (self.lookfrom - self.lookat).unit_vector();
        self.u = self.vup.cross(self.w).unit_vector();
        self.v = self.w.cross(self.u);

        // Calculate the vectors across the horizontal and down the vertical viewport edges.
        let viewport_u = viewport_width * self.u; // Vector across viewport horizontal edge
        let viewport_v = viewport_height * -self.v; // Vector down viewport vertical edge

        // Calculate the horizontal and vertical delta vectors from pixel to pixel.
        self.pixel_delta_u = viewport_u / self.image_width as f64;
        self.pixel_delta_v = viewport_v / self.image_height as f64;

        // Calculate the location of the upper left pixel.
        let viewport_upper_left =
            self.center - (self.focus_dist * self.w) - viewport_u / 2. - viewport_v / 2.;

        self.pixel00_loc = viewport_upper_left + 0.5 * (self.pixel_delta_u + self.pixel_delta_v);

        // Calculate the camera defocus disk basis vectors.
        let defocus_radius = self.focus_dist * (self.defocus_angle / 2.).to_radians().tan();
        self.defocus_disk_u = self.u * defocus_radius;
        self.defocus_disk_v = self.v * defocus_radius;
    }

    fn ray_color(&self, r: &Ray, depth: usize, world: &impl Hittable) -> Color {
        // If we've exceeded the ray bounce limit, no more light is gathered.
        if depth <= 0 {
            return Color::new();
        }

        let mut rec = HitRecord::default();

        if world.hit(r, Interval::from(0.001, f64::INFINITY), &mut rec) {
            let mut scattered = Ray::default();
            let mut attenuation = Color::new();
            if rec
                .material
                .scatter(r, &rec, &mut attenuation, &mut scattered)
            {
                if attenuation == Color::new() {
                    eprintln!("here at iteration {}", self.max_depth - depth);
                }
                return attenuation * self.ray_color(&scattered, depth - 1, world);
            }
            return Color::from(0, 0, 0);
        }

        let unit_direction = r.direction().unit_vector();
        let a = 0.5 * unit_direction.y() + 1.0;
        (1.0 - a) * Color::from(1.0, 1.0, 1.0) + a * Color::from(0.5, 0.7, 1.0)
    }

    fn get_ray(&self, i: usize, j: usize) -> Ray {
        // Get a randomly-sampled camera ray for the pixel at location i,j, originating from
        // the camera defocus disk.

        let pixel_center =
            self.pixel00_loc + (i as f64 * self.pixel_delta_u) + (j as f64 * self.pixel_delta_v);
        let pixel_sample = pixel_center + self.pixel_sample_square();

        let ray_origin = if self.defocus_angle <= 0. {
            self.center
        } else {
            self.defocus_disk_sample()
        };
        let ray_direction = pixel_sample - ray_origin;

        return Ray::new(ray_origin, ray_direction);
    }

    fn defocus_disk_sample(&self) -> Vec3 {
        // Returns a random point in the camera defocus disk.
        let p = Vec3::random_in_unit_disk();
        self.center + (p.x() * self.defocus_disk_u) + (p.y() * self.defocus_disk_v)
    }

    fn pixel_sample_square(&self) -> Vec3 {
        // Returns a random point in the square surrounding a pixel at the origin.
        let mut rng = rand::thread_rng();
        let px = -0.5 + rng.gen::<f64>();
        let py = -0.5 + rng.gen::<f64>();
        return (px * self.pixel_delta_u) + (py * self.pixel_delta_v);
    }
}
