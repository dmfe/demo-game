use macroquad::prelude::*;
use macroquad::rand::ChooseRandom;

struct Shape {
    size: f32,
    speed: f32,
    x: f32,
    y: f32,
    collided: bool,
    color: Color
}

impl Shape {
    fn collides_with(&self, other: &Self) -> bool {
        self.rect().overlaps(&other.rect())
    }

    fn collides_with_square(&self, other: &Self) -> bool {
        let closest_x = clamp(self.x, other.x - other.size / 2.0, other.x + other.size / 2.0);
        let closest_y = clamp(self.y, other.y - other.size / 2.0, other.y + other.size / 2.0);

        let distance_x = self.x - closest_x;
        let distance_y = self.y - closest_y;
        let distance_squared = distance_x * distance_x + distance_y * distance_y;

        distance_squared <= (self.size / 2.0) * (self.size / 2.0)
    }

    fn rect(&self) -> Rect {
        Rect {
            x: self.x - self.size / 2.0,
            y: self.y - self.size / 2.0,
            w: self.size,
            h: self.size
        }
    }
}

#[macroquad::main("Demo Game")]
async fn main() {
    const MOVEMENT_SPEED: f32 = 200.0;
    const RELOAD_TIME_SECONDS: f64 = 0.5;

    let square_colors = vec![ORANGE, RED, PURPLE, GREEN];

    let mut is_game_over = false;
    let mut squares: Vec<Shape> = vec![];
    let mut bullets: Vec<Shape> = vec![];
    let mut last_shot_time: f64 = 0.0;
    let mut player_circle = Shape {
        size: 32.0,
        speed: MOVEMENT_SPEED,
        x: screen_width() / 2.0,
        y: screen_height() / 2.0,
        collided: false,
        color: YELLOW
    };

    rand::srand(miniquad::date::now() as u64);

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

            if is_key_down(KeyCode::Right) || is_key_down(KeyCode::D) {
                player_circle.x += player_circle.speed * delta_time;
            }
            if is_key_down(KeyCode::Left) || is_key_down(KeyCode::A) {
                player_circle.x -= player_circle.speed * delta_time;
            }
            if is_key_down(KeyCode::Down) || is_key_down(KeyCode::S) {
                player_circle.y += player_circle.speed * delta_time;
            }
            if is_key_down(KeyCode::Up) || is_key_down(KeyCode::W) {
                player_circle.y -= player_circle.speed * delta_time;
            }
            if is_key_pressed(KeyCode::Space) {
                let current_time = get_time();
                if current_time - last_shot_time >= RELOAD_TIME_SECONDS {
                    bullets.push(Shape {
                        x: player_circle.x,
                        y: player_circle.y,
                        speed: player_circle.speed * 2.0,
                        size: 10.0,
                        collided: false,
                        color: RED
                    });
                    last_shot_time = current_time;
                }
            }

            // Clamp X and Y of player circle to be within screen
            player_circle.x = clamp(player_circle.x, 0.0 + player_circle.size, screen_width() - player_circle.size);
            player_circle.y = clamp(player_circle.y, 0.0 + player_circle.size, screen_height() - player_circle.size);

            // Ganerate a new square
            if rand::gen_range(0, 99) >= 70 {
                let size = rand::gen_range(36.0, 84.0);
                squares.push(Shape {
                    size: size,
                    speed: rand::gen_range(300.0, 400.0),
                    x: rand::gen_range(size / 2.0, screen_width() - size / 2.0),
                    y: -size,
                    collided: false,
                    color: *square_colors.choose().unwrap()
                });
            }

            // Movement
            for square in &mut squares {
                square.y += square.speed * delta_time;
            }
            for bullet in &mut bullets {
                bullet.y -= bullet.speed * delta_time;
            }

            // Remove shapes outside of the screen
            squares.retain(|square| square.y < screen_height() + square.size);
            bullets.retain(|bullet| bullet.y > 0.0 - bullet.size);

            // Remove collided shaped
            squares.retain(|square| !square.collided);
            bullets.retain(|bullet| !bullet.collided);
        }

        // Check for collisions
        if squares.iter().any(|square| player_circle.collides_with_square(square)) {
            is_game_over = true;
        }
        for square in squares.iter_mut() {
            for bullet in bullets.iter_mut() {
                if bullet.collides_with(square) {
                    bullet.collided = true;
                    square.collided = true;
                }
            }
        }

        if is_game_over && is_key_pressed(KeyCode::Space) {
            squares.clear();
            bullets.clear();
            player_circle.x = screen_width() / 2.0;
            player_circle.y = screen_height() / 2.0;
            is_game_over = false;
        }

        // Draw everything
        draw_circle(player_circle.x, player_circle.y, player_circle.size / 2.0, player_circle.color);
        for bullet in &bullets {
            draw_circle(bullet.x, bullet.y, bullet.size / 2.0, bullet.color);
        }
        for square in &squares {
            draw_rectangle(
                square.x - square.size / 2.0,
                square.y - square.size / 2.0,
                square.size,
                square.size,
                square.color
            );
        }

        if is_game_over {
            let text = "GAME OVER!!!";
            let text_dimensions = measure_text(text, None, 50, 1.0);
            draw_text(
                text,
                screen_width() / 2.0 - text_dimensions.width / 2.0,
                screen_height() / 2.0  - text_dimensions.height / 2.0 + text_dimensions.offset_y,
                50.0,
                RED
            );
        }

        next_frame().await
    }
}

