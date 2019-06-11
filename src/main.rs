extern crate minifb;

use minifb::{Key, WindowOptions, Window};

const WIDTH: usize = 360;
const HEIGHT: usize = 100;

fn hsl_to_rgb(h: i32, s: f64, l: f64) -> u32 {
    //println!("h: {}, s: {}, l: {}", h, s, l);
    let c: f64 = (1.0 - (2.0 * l - 1.0).abs()) * s;
    let x: f64 = c * (1.0 - ((h as f64 / 60.0) % 2.0 - 1.0).abs()) as f64;
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

    b as u32 + ((g as u32) << 8) + ((r as u32) << 16)
}

fn main() {
    let mut buffer: Vec<u32> = vec![0; WIDTH * HEIGHT];

    let mut window = Window::new("Test - ESC to exit",
                                 WIDTH,
                                 HEIGHT,
                                 WindowOptions::default()).unwrap_or_else(|e| {
        panic!("{}", e);
    });

    while window.is_open() && !window.is_key_down(Key::Escape) {
        let s: f64 = 1.0;
        
        let mut count: i32 = 0;
        for i in buffer.iter_mut() {
            let h: i32 = count % 360;
            let l: f64 = count as f64 / 36000.0; // see http://carols10cents.github.io/rust-conversion-reference/

            *i = hsl_to_rgb(h, s, l);
            count += 1;
        }

        // We unwrap here as we want this code to exit if it fails. Real applications may want to handle this in a different way
        window.update_with_buffer(&buffer).unwrap();
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

    // Test cases from https://www.rapidtables.com/convert/color/hsl-to-rgb.html
    #[test]
    fn test_hsl_to_rgb_black() {
        assert_eq!(hsl_to_rgb(0,   0.0, 0.0),  0x000000);  // Black
    }

    #[test]
    fn test_hsl_to_rgb_white() {
        assert_eq!(hsl_to_rgb(0,   0.0, 1.0),  0xFFFFFF);  // White
    }

    #[test]
    fn test_hsl_to_rgb_red() {
        assert_eq!(hsl_to_rgb(0,   1.0, 0.5),  0xFF0000);  // Red
    }

    #[test]
    fn test_hsl_to_rgb_lime() {
        assert_eq!(hsl_to_rgb(120, 1.0, 0.5),  0x00FF00);  // Lime
    }

    #[test]
    fn test_hsl_to_rgb_blue() {
        assert_eq!(hsl_to_rgb(240, 1.0, 0.5),  0x0000FF);  // Blue
    }

    #[test]
    fn test_hsl_to_rgb_yellow() {
        assert_eq!(hsl_to_rgb(60,  1.0, 0.5),  0xFFFF00);  // Yellow
    }

    #[test]
    fn test_hsl_to_rgb_cyan() {
        assert_eq!(hsl_to_rgb(180, 1.0, 0.5),  0x00FFFF);  // Cyan
    }

    #[test]
    fn test_hsl_to_rgb_magenta() {
        assert_eq!(hsl_to_rgb(300, 1.0, 0.5),  0xFF00FF);  // Magenta
    }

    #[test]
    fn test_hsl_to_rgb_silver() {
        assert_eq!(hsl_to_rgb(0,   0.0, 0.75), 0xC0C0C0);  // Silver
    }

    #[test]
    fn test_hsl_to_rgb_gray() {
        assert_eq!(hsl_to_rgb(0,   0.0, 0.5),  0x808080);  // Gray
    }

    #[test]
    fn test_hsl_to_rgb_maroon() {
        assert_eq!(hsl_to_rgb(0,   1.0, 0.25), 0x800000);  // Maroon
    }

    #[test]
    fn test_hsl_to_rgb_olive() {
        assert_eq!(hsl_to_rgb(60,  1.0, 0.25), 0x808000);  // Olive
    }

    #[test]
    fn test_hsl_to_rgb_green() {
        assert_eq!(hsl_to_rgb(120, 1.0, 0.25), 0x008000);  // Green
    }

    #[test]
    fn test_hsl_to_rgb_purple() {
        assert_eq!(hsl_to_rgb(300, 1.0, 0.25), 0x800080);  // Purple
    }

    #[test]
    fn test_hsl_to_rgb_teal() {
        assert_eq!(hsl_to_rgb(180, 1.0, 0.25), 0x008080);  // Teal
    }

    #[test]
    fn test_hsl_to_rgb_navy() {
        assert_eq!(hsl_to_rgb(240, 1.0, 0.5),  0x000080);  // Navy
    }
}