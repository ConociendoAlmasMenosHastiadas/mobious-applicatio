//! Simple visualization of the complex plane using egui_plot.
//!
//! Run with: cargo run --example visualize

use eframe::egui::{self, ColorImage};
use egui_plot::{Line, Plot, PlotImage, PlotPoints};
use num_complex::Complex;
use eframe::egui::Color32;
use mobius_applicatio::{MobiusTransform, plane_functions};

// Window and plot sizing
const WINDOW_SIZE: f32 = 1280.0;
const PLOT_SIZE: f32 = 1200.0;
const IMAGE_RESOLUTION: usize = 1280;
const PLANE_RANGE: f64 = 2.0;  // Complex plane spans from -PLANE_RANGE to +PLANE_RANGE

fn main() -> eframe::Result<()> {
    let options = eframe::NativeOptions {
        viewport: egui::ViewportBuilder::default().with_inner_size([WINDOW_SIZE, WINDOW_SIZE]),
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
                let image = self.generate_color_image(
                    IMAGE_RESOLUTION, 
                    IMAGE_RESOLUTION, 
                    -PLANE_RANGE, 
                    PLANE_RANGE, 
                    -PLANE_RANGE, 
                    PLANE_RANGE
                );
                self.image_texture = Some(ui.ctx().load_texture(
                    "complex_plane_colors",
                    image,
                    Default::default(),
                ));
            }
            
            Plot::new("complex_plane")
                .view_aspect(1.0)
                .width(PLOT_SIZE)
                .height(PLOT_SIZE)
                .show(ui, |plot_ui| {
                    // Render the color-mapped image
                    if let Some(texture) = &self.image_texture {
                        plot_ui.image(
                            PlotImage::new(
                                texture,
                                egui_plot::PlotPoint::new(0.0, 0.0), // center
                                [(2.0 * PLANE_RANGE) as f32, (2.0 * PLANE_RANGE) as f32], // size in plot coordinates
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
    ).expect("Valid transform coefficients");
    // let transform = MobiusTransform::identity();
    let z = transform.apply(z);
    
    // Test each grid type in order, return first match with its color
    if plane_functions::vertical_grid(z, 0.2, 0.01) {
        return Some(Color32::from_rgb(255, 0, 0)); // Red vertical bars
    }
    if plane_functions::horizontal_grid(z, 0.2, 0.01) {
        return Some(Color32::from_rgb(0, 0, 255)); // Blue horizontal bars
    }
    if plane_functions::radial_grid(z, 0.2, 0.01) {
        return Some(Color32::from_rgb(0, 255, 0)); // Green circles
    }
    if plane_functions::angular_grid(z, std::f64::consts::PI / 12.0, 0.01) {
        return Some(Color32::from_rgb(255, 0, 255)); // Magenta angular lines
    }
    
    None
}