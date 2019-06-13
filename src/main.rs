extern crate minifb;

use minifb::{Key, WindowOptions, Window};
use std::f64;
use std::cmp;

const WIDTH: usize = 640;
const HEIGHT: usize = 480;

fn hsl_to_rgb(h: i32, s: f64, l: f64) -> u32 {
    //println!("h: {}, s: {}, l: {}", h, s, l);
    let c: f64 = (1.0 - (2.0 * l - 1.0).abs()) * s;
    let x: f64 = c * (1.0 - ((h as f64 / 60.0) % 2.0 - 1.0).abs());
    let m: f64 = l - c / 2.0;
    //println!("c: {}, x: {}, m: {}", c, x, m);
    let (rp, gp, bp) = match h {
        0...59 => (c, x, 0.0),
        60...119 => (x, c, 0.0),
        120...179 => (0.0, c, x),
        180...239 => (0.0, x, c),
        240...299 => (x, 0.0, c),
        300...359 => (c, 0.0, x),
        _ => (0.0, 0.0, 0.0)
    };
    //println!("r': {}, g': {}, b': {}", rp, gp, bp);
    let (r, g, b) = ((rp + m) * 255.0, (gp + m) * 255.0, (bp + m) * 255.0);
    //println!("r: {}, g: {}, b: {}", r, g, b);

    b.round() as u32 + ((g.round() as u32) << 8) + ((r.round() as u32) << 16)
}

fn distance_from_center(x: i32, y: i32, center_x: i32, center_y: i32) -> f64 {
    return (((x - center_x) * (x - center_x) + (y - center_y) * (y - center_y)) as f64).sqrt()
}

fn angle_from_center(x: i32, y: i32, center_x: i32, center_y: i32) -> f64 {
    let mut angle = ((y - center_y) as f64).atan2((x - center_x) as f64) * 180.0 / f64::consts::PI;
    if angle < 0.0 {
        angle = angle + 360.0;
    }
    return angle
}

fn main() {
    let mut buffer: Vec<u32> = vec![0; WIDTH * HEIGHT];

    let mut window = Window::new("Test - ESC to exit",
                                 WIDTH,
                                 HEIGHT,
                                 WindowOptions::default()).unwrap_or_else(|e| {
        panic!("{}", e);
    });

    let mut time: i32 = 0;
    while window.is_open() && !window.is_key_down(Key::Escape) {
        let s: f64 = 1.0;
        let (center_x, center_y) = ((WIDTH / 2) as i32, (HEIGHT / 2) as i32);
        let (radius_min, radius_max) = (0.35 * cmp::min(WIDTH, HEIGHT) as f64,
                                        0.4 * cmp::min(WIDTH, HEIGHT) as f64);
        
        for y in 0..HEIGHT {
            for x in 0..WIDTH {
                let distance = distance_from_center(x as i32, y as i32, center_x, center_y);
                let angle = (angle_from_center(x as i32, y as i32, center_x, center_y) as i32 + time) % 360;

                if distance > radius_min && distance < radius_max {
                    buffer[x + y * WIDTH] = hsl_to_rgb(angle, s, 0.5);
                } else {
                    buffer[x + y * WIDTH] = 0;
                }
            }
        }

        // We unwrap here as we want this code to exit if it fails. Real applications may want to handle this in a different way
        window.update_with_buffer(&buffer).unwrap();
        time += 10;
    }
}

/*
 * Run unit tests with :
 * 
 * ```sh
 * $ cargo test
 * ```
 */
#[cfg(test)]
mod tests {
    // Note this useful idiom: importing names from outer (for mod tests) scope.
    use super::*;

    fn test_hsl_to_rgb(h: i32, s: f64, l: f64, expected: u32, name: &str) {
        let computed = hsl_to_rgb(h, s, l);
        assert_eq!(computed, expected, "HSL to RGB conversion failed: {}. Got {:06X}, expected {:06X}", name, computed, expected);
    }

    #[test]
    fn test_angle_from_center() {
        assert_eq!(angle_from_center(2, 2, 1, 1), 45.0);
        assert_eq!(angle_from_center(0, 2, 1, 1), 135.0);
        assert_eq!(angle_from_center(0, 0, 1, 1), 225.0);
        assert_eq!(angle_from_center(2, 0, 1, 1), 315.0);
    }

    #[test]
    fn test_distance_from_center() {
        assert_eq!(distance_from_center(6, 7, 3, 3), 5.0);
    }

    // Test cases from https://www.rapidtables.com/convert/color/hsl-to-rgb.html
    #[test]
    fn test_hsl_to_rgb_black() {
        test_hsl_to_rgb(0, 0.0, 0.0, 0x000000, "Black");
    }

    #[test]
    fn test_hsl_to_rgb_white() {
        test_hsl_to_rgb(0, 0.0, 1.0, 0xFFFFFF, "White");
    }

    #[test]
    fn test_hsl_to_rgb_red() {
        test_hsl_to_rgb(0, 1.0, 0.5, 0xFF0000, "Red");
    }

    #[test]
    fn test_hsl_to_rgb_lime() {
        test_hsl_to_rgb(120, 1.0, 0.5, 0x00FF00, "Lime");
    }

    #[test]
    fn test_hsl_to_rgb_blue() {
        test_hsl_to_rgb(240, 1.0, 0.5, 0x0000FF, "Blue");
    }

    #[test]
    fn test_hsl_to_rgb_yellow() {
        test_hsl_to_rgb(60, 1.0, 0.5, 0xFFFF00, "Yellow");
    }

    #[test]
    fn test_hsl_to_rgb_cyan() {
        test_hsl_to_rgb(180, 1.0, 0.5, 0x00FFFF, "Cyan");
    }

    #[test]
    fn test_hsl_to_rgb_magenta() {
        test_hsl_to_rgb(300, 1.0, 0.5, 0xFF00FF, "Magenta");
    }

    #[test]
    fn test_hsl_to_rgb_silver() {
        test_hsl_to_rgb(0, 0.0, 0.75, 0xBFBFBF, "Silver");
    }

    #[test]
    fn test_hsl_to_rgb_gray() {
        test_hsl_to_rgb(0, 0.0, 0.5, 0x808080, "Gray");
    }

    #[test]
    fn test_hsl_to_rgb_maroon() {
        test_hsl_to_rgb(0, 1.0, 0.25, 0x800000, "Maroon");
    }

    #[test]
    fn test_hsl_to_rgb_olive() {
        test_hsl_to_rgb(60, 1.0, 0.25, 0x808000, "Olive");
    }

    #[test]
    fn test_hsl_to_rgb_green() {
        test_hsl_to_rgb(120, 1.0, 0.25, 0x008000, "Green");
    }

    #[test]
    fn test_hsl_to_rgb_purple() {
        test_hsl_to_rgb(300, 1.0, 0.25, 0x800080, "Purple");
    }

    #[test]
    fn test_hsl_to_rgb_teal() {
        test_hsl_to_rgb(180, 1.0, 0.25, 0x008080, "Teal");
    }

    #[test]
    fn test_hsl_to_rgb_navy() {
        test_hsl_to_rgb(240, 1.0, 0.25, 0x000080, "Navy");
    }
}