use macroquad::prelude::*;

#[macroquad::main("Demo Game")]
async fn main() {
    loop {
        clear_background(DARKPURPLE);
        next_frame().await
    }
}
