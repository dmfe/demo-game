use macroquad::prelude::*;

#[macroquad::main("Demo Game")]
async fn main() {

    const MOVEMENT_SPEED: f32 = 200.0;

    let mut is_game_over = false;
    let mut x = screen_width() / 2.0;
    let mut y = screen_height() / 2.0;

    let player_radius = 16.0;
    let error_spot_radius = 16.0;
    let error_spot_x = x + 100.0;
    let error_spot_y = y + 100.0;

    let mut distance = ((error_spot_x - x).powf(2.0) + (error_spot_y - y).powf(2.0)).sqrt();

    loop {
        clear_background(BLUE);

        let fps_text = format!("FPS: {}", get_fps());
        draw_text(
            &fps_text,
            20.0,
            20.0,
            30.0,
            RED
        );

        if !is_game_over {
            let delta_time = get_frame_time();

            draw_circle(error_spot_x, error_spot_y, error_spot_radius, RED);
            draw_circle(x, y, player_radius, YELLOW);

            if is_key_down(KeyCode::Right) || is_key_down(KeyCode::D) {
                x += MOVEMENT_SPEED * delta_time;
            }
            if is_key_down(KeyCode::Left) || is_key_down(KeyCode::A) {
                x -= MOVEMENT_SPEED * delta_time;
            }
            if is_key_down(KeyCode::Down) || is_key_down(KeyCode::S) {
                y += MOVEMENT_SPEED * delta_time;
            }
            if is_key_down(KeyCode::Up) || is_key_down(KeyCode::W) {
                y -= MOVEMENT_SPEED * delta_time;
            }

            distance = ((error_spot_x - x).powf(2.0) + (error_spot_y - y).powf(2.0)).sqrt();

            x = clamp(x, 0.0 + player_radius, screen_width() - player_radius);
            y = clamp(y, 0.0 + player_radius, screen_height() - player_radius)
        } else {
            draw_text(
                "CRITICAL ERROR DETECTED!!!",
                200.0,
                200.0,
                50.0,
                WHITE
            );
        }

        if distance <= player_radius + error_spot_radius {
            is_game_over = true;
        }

        next_frame().await
    }
}
