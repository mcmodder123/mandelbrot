use mandelbrot::image::generate_png_mandelbrot;
use std::env;

fn main() {
    let args: Vec<String> = env::args().collect();

    let mut filename = "mandelbrot.png";
    let mut center_re = -0.5;
    let mut center_im = 0.0;
    let mut zoom_factor: f64 = 1.0;

    let mut i = 1;
    while i < args.len() {
        match args[i].as_str() {
            "-o" | "--output" => {
                if i + 1 < args.len() {
                    filename = args[i + 1].as_str();
                    i += 1;
                }
            }
            "-c" | "--center" => {
                // Center coordinates
                if i + 2 < args.len() {
                    center_re = args[i + 1]
                        .parse()
                        .expect("Failed to parse center real part");
                    center_im = args[i + 2]
                        .parse()
                        .expect("Failed to parse center imaginary part");
                    i += 2;
                }
            }
            "-z" | "--zoom" => {
                if i + 1 < args.len() {
                    zoom_factor = args[i + 1].parse().expect("Failed to parse zoom factor");
                    i += 1;
                }
            }
            _ => {
                if filename == "mandelbrot.png" {
                    filename = args[i].as_str();
                }
            }
        }
        i += 1;
    }

    let max_iters: u32 = 4096;
    let width: u32 = 800;
    let height: u32 = 600;

    let initial_width_complex = 3.0;
    let initial_height_complex = 3.0;

    let view_width_complex = initial_width_complex / zoom_factor;
    let view_height_complex = initial_height_complex / zoom_factor;

    let xmin = center_re - view_width_complex / 2.0;
    let xmax = center_re + view_width_complex / 2.0;
    let ymin = center_im - view_height_complex / 2.0;
    let ymax = center_im + view_height_complex / 2.0;

    let adjusted_max_iters = (max_iters as f64 * zoom_factor.sqrt() * 0.5) as u32;
    let final_max_iters = adjusted_max_iters.max(max_iters);

    println!(
        "Generating {} with center ({}, {}) and zoom {}x...",
        filename, center_re, center_im, zoom_factor
    );
    println!(
        "Complex plane: x: [{}, {}], y: [{}, {}]",
        xmin, xmax, ymin, ymax
    );
    println!("Using max_iters: {}", final_max_iters);

    match generate_png_mandelbrot(
        width,
        height,
        xmin,
        ymin,
        xmax,
        ymax,
        final_max_iters,
        filename,
    ) {
        Ok(_) => println!("Successfully generated {}", filename),
        Err(e) => eprintln!("Error generating image: {}", e),
    }
}
