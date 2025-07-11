pub mod complex {
    // complex number representation
    #[derive(Debug)]
    pub struct Complex {
        pub re: f64,
        pub im: f64,
    }

    impl Complex {
        pub fn new(re: f64, im: f64) -> Complex {
            Complex { re, im }
        }

        pub fn sqr(&self) -> Complex {
            Complex::new(
                self.re * self.re - self.im * self.im,
                2.0 * self.re * self.im,
            )
        }

        pub fn add(&self, other: &Complex) -> Complex {
            Complex::new(self.re + other.re, self.im + other.im)
        }

        pub fn magnitude_sq(&self) -> f64 {
            self.re * self.re + self.im * self.im
        }
    }
}

pub mod mandelbrot {
    use crate::complex::Complex;
    // determines if point 'c' is in the Mandelbrot set
    pub fn mandelbrot_iter(c: &Complex, max_iters: u32) -> u32 {
        let mut z = Complex::new(0.0, 0.0);
        for i in 0..max_iters {
            if z.magnitude_sq() > 4.0 {
                return i;
            }
            z = z.sqr().add(c);
        }
        max_iters
    }
}

pub mod image {
    use crate::{complex::Complex, mandelbrot::mandelbrot_iter};
    // converts a pixel coordinate (x, y) to a complex number 'c'
    pub fn pixel_to_complex(
        x: u32,
        y: u32,
        width: u32,
        height: u32,
        xmin: f64,
        ymin: f64,
        xmax: f64,
        ymax: f64,
    ) -> Complex {
        let cre = xmin + (x as f64 / width as f64) * (xmax - xmin);
        let cim = ymin + (y as f64 / height as f64) * (ymax - ymin);
        Complex::new(cre, cim)
    }

    use image::{Rgb, RgbImage};
    pub fn generate_png_mandelbrot(
        width: u32,
        height: u32,
        xmin: f64,
        ymin: f64,
        xmax: f64,
        ymax: f64,
        max_iters: u32,
        filename: &str,
    ) -> Result<(), image::ImageError> {
        let mut img = RgbImage::new(width, height);

        for y in 0..height {
            for x in 0..width {
                let c = pixel_to_complex(x, y, width, height, xmin, ymin, xmax, ymax);
                let iters = mandelbrot_iter(&c, max_iters);

                let color = if iters == max_iters {
                    Rgb([0, 0, 0])
                } else {
                    let intensity = (iters as f64 / max_iters as f64 * 255.0) as u8;

                    let base_r: u8 = 50;
                    let base_g: u8 = 50;
                    let base_b: u8 = 50;

                    let r = (base_r as u16 + intensity as u16).min(255);
                    let g = (base_g as u16 + intensity as u16).min(255);
                    let b = (base_b as u16 + intensity as u16).min(255);

                    Rgb([r as u8, g as u8, b as u8])
                };
                img.put_pixel(x, y, color);
            }
        }
        img.save(filename)?;
        Ok(())
    }
}
