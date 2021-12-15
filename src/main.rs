extern crate minifb;
use ui::color::{rgb_2_int, int_2_rgb};
use ui::texts::text::Text;

use minifb::{Key, Window, WindowOptions};

const WIDTH: usize = 640;
const HEIGHT: usize = 360;

fn main() {
    let mut buffer: Vec<u32> = vec![0; WIDTH * HEIGHT];
    let mut text = Text::new();

    let c = 16712640;
    let t = int_2_rgb(c);

    let mut window = Window::new(
        "Test - ESC to exit",
        WIDTH,
        HEIGHT,
        WindowOptions::default(),
    )
    .unwrap_or_else(|e| {
        panic!("{}", e);
    });

    // Limit to max ~60 fps update rate
    window.limit_update_rate(Some(std::time::Duration::from_micros(16600)));
    println!("{}", buffer.len());
    for i in 0..1000 {
            buffer[i] = i as u32;
    }
    while window.is_open() && !window.is_key_down(Key::Escape) {
        text.render(&mut buffer);
        
        // We unwrap here as we want this code to exit if it fails. Real applications may want to handle this in a different way
        window
            .update_with_buffer(&buffer, WIDTH, HEIGHT)
            .unwrap();
    }
}