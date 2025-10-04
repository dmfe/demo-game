use macroquad::prelude::*;

#[macroquad::main("Demo Game")]
async fn main() {
    loop {
        clear_background(BLUE);

        draw_text(
            "CRITICAL ERROR DETECTED!!!",
            200.0,
            200.0,
            50.0,
            WHITE
        );

        let fps_text = format!("FPS: {}", get_fps());
        draw_text(
            &fps_text,
            20.0,
            20.0,
            30.0,
            RED
        );

        next_frame().await
    }
}
