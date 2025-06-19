use macroquad::prelude::*;

#[macroquad::main("My game")]
async fn main() {
    loop {
        let mut x = screen_width() /2.0 ;
        let mut y = screen_height() /2.0 ;

        if is_key_down(KeyCode::Right) {
            x += 1.0;
        }
        if is_key_down(KeyCode::Left) {
            x -= 1.0;
        }
        if is_key_down(KeyCode::Down) {
            y += 1.0;
        }
        if is_key_down(KeyCode::Up) {
            y -= 1.0;
        }

        draw_circle(x, y, 16.0, YELLOW);

        clear_background(DARKBLUE);
        next_frame().await
    }
}