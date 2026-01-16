//! Simple visualization of the complex plane using egui_plot.
//!
//! Run with: cargo run --example visualize

use eframe::egui::{self, ColorImage};
use egui_plot::{Line, Plot, PlotImage, PlotPoints};
use num_complex::Complex;
use eframe::egui::Color32;
use mobius_applicatio::MobiusTransform;

fn main() -> eframe::Result<()> {
    let options = eframe::NativeOptions {
        viewport: egui::ViewportBuilder::default().with_inner_size([700.0, 700.0]),
        ..Default::default()
    };
    
    eframe::run_native(
        "Complex Plane",
        options,
        Box::new(|_cc| Ok(Box::new(ComplexPlaneApp { image_texture: None }))),
    )
}

struct ComplexPlaneApp {
    image_texture: Option<egui::TextureHandle>,
}

impl eframe::App for ComplexPlaneApp {
    fn update(&mut self, ctx: &egui::Context, _frame: &mut eframe::Frame) {
        egui::CentralPanel::default().show(ctx, |ui| {
            ui.heading("Complex Plane Visualization");
            ui.label("Real axis (horizontal), Imaginary axis (vertical)");
            
            // Generate the color-mapped image if not already created
            if self.image_texture.is_none() {
                let image = self.generate_color_image(400, 400, -2.0, 2.0, -2.0, 2.0);
                self.image_texture = Some(ui.ctx().load_texture(
                    "complex_plane_colors",
                    image,
                    Default::default(),
                ));
            }
            
            Plot::new("complex_plane")
                .view_aspect(1.0)
                .width(600.0)
                .height(600.0)
                .show(ui, |plot_ui| {
                    // Render the color-mapped image
                    if let Some(texture) = &self.image_texture {
                        plot_ui.image(
                            PlotImage::new(
                                texture,
                                egui_plot::PlotPoint::new(0.0, 0.0), // center
                                [4.0, 4.0], // size: -2 to +2 in both dimensions
                            )
                        );
                    }
                    
                    // Draw grid lines on top
                    // Add a unit circle for reference
                    let circle: PlotPoints = (0..=100)
                        .map(|i| {
                            let t = 2.0 * std::f64::consts::PI * i as f64 / 100.0;
                            [t.cos(), t.sin()]
                        })
                        .collect();
                    
                    plot_ui.line(
                        Line::new(circle)
                            .color(egui::Color32::from_rgb(255, 255, 255))
                            .width(2.0)
                            .name("Unit Circle"),
                    );
                    
                    // Origin marker
                    let origin = vec![[0.0, 0.0]];
                    plot_ui.points(
                        egui_plot::Points::new(origin)
                            .radius(5.0)
                            .color(egui::Color32::WHITE)
                            .name("Origin"),
                    );
                });
        });
    }
}

impl ComplexPlaneApp {
    /// Generate a color image by sampling the complex plane
    fn generate_color_image(
        &self,
        width: usize,
        height: usize,
        x_min: f64,
        x_max: f64,
        y_min: f64,
        y_max: f64,
    ) -> ColorImage {
        let mut pixels = Vec::with_capacity(width * height);
        
        for row in 0..height {
            for col in 0..width {
                // Map pixel coordinates to complex plane coordinates
                let real = x_min + (col as f64 / width as f64) * (x_max - x_min);
                // Note: flip y-axis (screen coords go down, imaginary axis goes up)
                let imag = y_max - (row as f64 / height as f64) * (y_max - y_min);
                
                let z = Complex::new(real, imag);
                
                // Use the point_color function to determine color
                let color = match point_color(z) {
                    Some(c) => c,
                    None => Color32::TRANSPARENT, // Handle transparent pixels
                };
                
                pixels.push(color);
            }
        }
        
        ColorImage {
            size: [width, height],
            pixels,
        }
    }
}

/// Determine the color for a point in the complex plane.
/// Returns None for transparent (no color) pixels.
fn point_color(z: Complex<f64>) -> Option<Color32> {
    // Apply Möbius transform
    let transform = MobiusTransform::new(
        Complex::new(1.0, 0.0),  // a
        Complex::new(-1.0, 0.0),  // b
        Complex::new(1.0, 0.0),  // c
        Complex::new(1.0, 0.0),  // d
    );
    let z = transform.apply(z);
    
    plane_grid(z, 0.01)
}

/// Create a grid pattern on the complex plane.
/// Returns colored bars at regular intervals.
fn plane_grid(z: Complex<f64>, thickness: f64) -> Option<Color32> {
    // Check if we're in a red vertical bar (every 0.2 units)
    let re_mod = (z.re.abs() % 0.2).abs();
    let in_red_bar = re_mod >= 0.1 - thickness && re_mod < 0.1 + thickness;
    
    // Check if we're in a blue horizontal bar (every 0.2 units)
    let im_mod = (z.im.abs() % 0.2).abs();
    let in_blue_bar = im_mod >= 0.1 - thickness && im_mod < 0.1 + thickness;
    
    if in_red_bar && in_blue_bar {
        Some(Color32::from_rgb(255, 0, 255)) // Purple where bars overlap
    } else if in_red_bar {
        Some(Color32::from_rgb(255, 0, 0)) // Red vertical bars
    } else if in_blue_bar {
        Some(Color32::from_rgb(0, 0, 255)) // Blue horizontal bars
    } else {
        None // Transparent
    }
}