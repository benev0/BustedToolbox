use raylib::prelude::*;

pub fn main() {
    let (mut rl, thread) = raylib::init()
        .size(640, 480)
        .title("this is a test")
        .build();

    while !rl.window_should_close() {
        let mut d = rl.begin_drawing(&thread);

        d.clear_background(Color::RAYWHITE);
        d.draw_text("this is a test", 12, 12, 20, Color::BLACK);
    }
}
