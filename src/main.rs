use std::fs;
use macroquad::prelude::*;
use macroquad::rand::ChooseRandom;
use macroquad::experimental::animation::{AnimatedSprite, Animation};
use macroquad_particles::{self as particles, ColorCurve, Emitter, EmitterConfig};

const FRAGMENT_SHADER: &str = include_str!("starfield-shader.glsl");
const VERTEX_SHADER: &str = "#version 100
attribute vec3 position;
attribute vec2 texcoord;
attribute vec4 color0;
varying float iTime;

uniform mat4 Model;
uniform mat4 Projection;
uniform vec4 _Time;

void main() {
    gl_Position = Projection * Model * vec4(position, 1);
    iTime = _Time.x;
}
";

struct Shape {
    width: f32,
    height: f32,
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

    fn rect(&self) -> Rect {
        Rect {
            x: self.x - self.width / 2.0,
            y: self.y - self.height / 2.0,
            w: self.width,
            h: self.height
        }
    }
}

enum GameState {
    MainMenu,
    Playing,
    Paused,
    GameOver
}

fn draw_playing_scene(
    player: &Shape,
    player_sprite: &mut AnimatedSprite,
    player_texture: &Texture2D,
    player_engine: &mut Emitter,
    bullets: &Vec<Shape>,
    bullet_sprite: &mut AnimatedSprite,
    bullet_texture: &Texture2D,
    squares: &Vec<Shape>,
    explosions: &mut Vec<(Emitter, Vec2)>,
    score: u32,
    high_score: u32
) {
    // Update sprites
    player_sprite.update();
    bullet_sprite.update();

    // Draw Player
    player_engine.draw(vec2(
            player.x,
            player.y + player.height / 3.0
    ));
    let player_frame = player_sprite.frame();
    draw_texture_ex(
        player_texture,
        player.x - player.width / 2.0,
        player.y - player.width / 2.0,
        WHITE,
        DrawTextureParams {
            dest_size: Some(vec2(player.width, player.height)),
            source: Some(player_frame.source_rect),
            ..Default::default()
        }
    );

    // Draw bullets
    for bullet in bullets {
        let bullet_frame = bullet_sprite.frame();
        draw_texture_ex(
            bullet_texture,
            bullet.x - bullet_frame.dest_size.x,
            bullet.y - bullet_frame.dest_size.y,
            WHITE,
            DrawTextureParams {
                dest_size: Some(vec2(bullet.width, bullet.height)),
                source: Some(bullet_frame.source_rect),
                ..Default::default()
            }
        );
    }

    for square in squares {
        draw_rectangle(
            square.x - square.width / 2.0,
            square.y - square.height / 2.0,
            square.width,
            square.height,
            square.color
        );
    }
    for (explosion, coords) in explosions.iter_mut() {
        explosion.draw(*coords);
    }

    draw_text(
        format!("Score: {}", score).as_str(),
        10.0,
        45.0,
        25.0,
        WHITE
    );
    let high_score_text = format!("High Score: {}", high_score);
    let text_dimensions = measure_text(high_score_text.as_str(), None, 25, 1.0);
    draw_text(
        high_score_text.as_str(),
        screen_width() - text_dimensions.width - 10.0,
        45.0,
        25.0,
        WHITE
    );
}

fn particle_explosion() -> particles::EmitterConfig {
    particles::EmitterConfig {
        local_coords: false,
        one_shot: true,
        emitting: true,
        lifetime: 0.6,
        lifetime_randomness: 0.3,
        explosiveness: 0.65,
        initial_direction_spread: 2.0 * std::f32::consts::PI,
        initial_velocity: 300.0,
        initial_velocity_randomness: 0.8,
        size: 3.0,
        size_randomness: 0.3,
        colors_curve: ColorCurve {
            start: RED,
            mid: ORANGE,
            end: RED
        },
        ..Default::default()
    }
}

fn particle_engine() -> particles::EmitterConfig {
    particles::EmitterConfig {
        local_coords: false,
        one_shot: false,
        emitting: false,
        lifetime: 0.6,
        lifetime_randomness: 0.3,
        explosiveness: 0.65,
        initial_direction: vec2(0.0, 1.0),
        initial_direction_spread: 0.3 * std::f32::consts::PI,
        initial_velocity: 300.0,
        initial_velocity_randomness: 0.8,
        size: 3.0,
        size_randomness: 0.3,
        colors_curve: ColorCurve {
            start: SKYBLUE,
            mid: SKYBLUE,
            end: SKYBLUE
        },
        ..Default::default()
    }
}

#[macroquad::main("Space Warior")]
async fn main() {
    const MOVEMENT_SPEED: f32 = 200.0;
    const RELOAD_TIME_SECONDS: f64 = 0.1;
    const SIDE_ANIMATION_SWITCH_SECONDS: f64 = 0.5;

    let square_colors = vec![ORANGE, RED, PURPLE, GREEN];

    let mut game_state = GameState::MainMenu;
    let mut squares: Vec<Shape> = vec![];
    let mut bullets: Vec<Shape> = vec![];
    let mut explosions: Vec<(Emitter, Vec2)> = vec![];
    let mut player = Shape {
        width: 64.0,
        height: 96.0,
        speed: MOVEMENT_SPEED,
        x: screen_width() / 2.0,
        y: screen_height() / 2.0,
        collided: false,
        color: YELLOW
    };
    let mut player_engine: Emitter = Emitter::new(EmitterConfig {
        amount: player.height.round() as u32 * 2,
        ..particle_engine()
    });
    let mut last_shot_time: f64 = 0.0;
    let mut last_left_key_time: f64 = 0.0;
    let mut last_right_key_time: f64 = 0.0;
    let mut score: u32 = 0;
    let mut high_score: u32 = fs::read_to_string("highscore.dat")
        .map_or(Ok(0), |i| i.parse::<u32>())
        .unwrap_or(0);

    rand::srand(miniquad::date::now() as u64);

    let i_resolution: [f32; 2] = [screen_width(), screen_height()];
    let mut direction_modifier: f32 = 0.0;
    let render_target = render_target(320, 150);
    render_target.texture.set_filter(FilterMode::Nearest);
    let material = load_material(
        ShaderSource::Glsl {
            vertex: VERTEX_SHADER,
            fragment: FRAGMENT_SHADER
        },
        MaterialParams {
            uniforms: vec![
                UniformDesc::new("direction_modifier", UniformType::Float1),
                UniformDesc::new("iResolution", UniformType::Float2),
            ],
            ..Default::default()
        }
    )
    .unwrap();

    // Resources initialization
    set_pc_assets_folder("assets");

    let player_texture: Texture2D = load_texture("ship.png")
        .await
        .expect("Couldn't load texture file.");
    player_texture.set_filter(FilterMode::Nearest);
    let bullet_texture: Texture2D = load_texture("laser-bolts.png")
        .await
        .expect("Couldn't load texture file.");
    bullet_texture.set_filter(FilterMode::Nearest);
    build_textures_atlas();

    // Animations
    let mut player_sprite = AnimatedSprite::new(
        16,
        24,
        &[
            Animation {
                name: "idle".to_string(),
                row: 0,
                frames: 2,
                fps: 12,
            },
            Animation {
                name: "slight-left".to_string(),
                row: 1,
                frames: 2,
                fps: 12,
            },
            Animation {
                name: "left".to_string(),
                row: 2,
                frames: 2,
                fps: 12,
            },
            Animation {
                name: "slight-right".to_string(),
                row: 3,
                frames: 2,
                fps: 12,
            },
            Animation {
                name: "right".to_string(),
                row: 4,
                frames: 2,
                fps: 12,
            }
        ],
        true
    );
    let mut bullet_sprite = AnimatedSprite::new(
        16,
        16,
        &[
            Animation {
                name: "bullet".to_string(),
                row: 0,
                frames: 2,
                fps: 12
            },
            Animation {
                name: "bolt".to_string(),
                row: 1,
                frames: 2,
                fps: 12
            },
        ],
        true
    );
    bullet_sprite.set_animation(1);

    loop {
        clear_background(BLACK);

        material.set_uniform("direction_modifier", direction_modifier);
        material.set_uniform("iResolution", i_resolution);
        gl_use_material(&material);
        draw_texture_ex(
            &render_target.texture,
            0.,
            0.,
            WHITE,
            DrawTextureParams {
                dest_size: Some(vec2(screen_width(), screen_height())),
                ..Default::default()
            }
        );
        gl_use_default_material();

        match game_state {
            GameState::MainMenu => {
                if is_key_pressed(KeyCode::Escape) {
                    std::process::exit(0);
                }
                if is_key_pressed(KeyCode::Space) {
                    squares.clear();
                    bullets.clear();
                    explosions.clear();
                    player_engine.config.emitting = true;
                    player.x = screen_width() / 2.0;
                    player.y = screen_height() / 2.0;
                    score = 0;
                    game_state = GameState::Playing;
                }

                let title_text = "SPACE WARIOR";
                let title_text_dimensions = measure_text(title_text, None, 100, 1.0);
                draw_text(
                    title_text,
                    screen_width() / 2.0 - title_text_dimensions.width / 2.0,
                    200.0,
                    100.0,
                    WHITE
                );

                let text = "Press space";
                let text_dimensions = measure_text(text, None, 50, 1.0);
                draw_text(
                    text,
                    screen_width() / 2.0 - text_dimensions.width / 2.0,
                    screen_height() / 2.0 - text_dimensions.height / 2.0 + text_dimensions.offset_y,
                    50.0,
                    WHITE
                );
            },
            GameState::Playing => {
                let delta_time = get_frame_time();
                player_sprite.set_animation(0);

                if is_key_down(KeyCode::Right) || is_key_down(KeyCode::D) {
                    player.x += player.speed * delta_time;
                    direction_modifier += 0.05 * delta_time;

                    if get_time() - last_right_key_time >= SIDE_ANIMATION_SWITCH_SECONDS {
                        player_sprite.set_animation(4);
                    } else {
                        player_sprite.set_animation(3);
                    }
                }
                if is_key_pressed(KeyCode::Right) || is_key_pressed(KeyCode::D) {
                    last_right_key_time = get_time();
                }

                if is_key_down(KeyCode::Left) || is_key_down(KeyCode::A) {
                    player.x -= player.speed * delta_time;
                    direction_modifier -= 0.05 * delta_time;

                    if get_time() - last_left_key_time >= SIDE_ANIMATION_SWITCH_SECONDS {
                        player_sprite.set_animation(2);
                    } else {
                        player_sprite.set_animation(1);
                    }
                }
                if is_key_pressed(KeyCode::Left) || is_key_pressed(KeyCode::A) {
                    last_left_key_time = get_time();
                }

                if is_key_down(KeyCode::Down) || is_key_down(KeyCode::S) {
                    player.y += player.speed * delta_time;
                }
                if is_key_down(KeyCode::Up) || is_key_down(KeyCode::W) {
                    player.y -= player.speed * delta_time;
                }
                if is_key_pressed(KeyCode::Space) {
                    let current_time = get_time();
                    if current_time - last_shot_time >= RELOAD_TIME_SECONDS {
                        bullets.push(Shape {
                            width: 32.0,
                            height: 32.0,
                            x: player.x,
                            y: player.y - 24.0,
                            speed: player.speed * 2.0,
                            collided: false,
                            color: RED
                        });
                        last_shot_time = current_time;
                    }
                }
                if is_key_pressed(KeyCode::Escape) {
                    game_state = GameState::Paused;
                }

                // Clamp X and Y of player circle to be within screen
                player.x = clamp(player.x, 0.0 + player.width, screen_width() - player.width);
                player.y = clamp(player.y, 0.0 + player.height, screen_height() - player.height);

                // Ganerate a new square
                if rand::gen_range(0, 99) >= 90 {
                    let size = rand::gen_range(36.0, 84.0);
                    squares.push(Shape {
                        width: size,
                        height: size,
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
                squares.retain(|square| square.y < screen_height() + square.height);
                bullets.retain(|bullet| bullet.y > 0.0 - bullet.height);

                // Remove collided shaped
                squares.retain(|square| !square.collided);
                bullets.retain(|bullet| !bullet.collided);

                // Remove the old explosions
                explosions.retain(|(explosion, _)| explosion.config.emitting);

                // Check for collisions
                if squares.iter().any(|square| player.collides_with(square)) {
                    game_state = GameState::GameOver;
                    player_engine.config.emitting = false;
                }
                for square in squares.iter_mut() {
                    for bullet in bullets.iter_mut() {
                        if bullet.collides_with(square) {
                            bullet.collided = true;
                            square.collided = true;
                            score += square.height.round() as u32;
                            high_score = high_score.max(score);

                            // Start new explosion
                            explosions.push((
                                Emitter::new(EmitterConfig {
                                    amount: square.height.round() as u32 * 2,
                                    ..particle_explosion()
                                }),
                                vec2(square.x, square.y)
                            ));
                        }
                    }
                }

                // Draw playing scene
                draw_playing_scene(
                    &player,
                    &mut player_sprite,
                    &player_texture,
                    &mut player_engine,
                    &bullets,
                    &mut bullet_sprite,
                    &bullet_texture,
                    &squares,
                    &mut explosions,
                    score,
                    high_score
                );
            },
            GameState::Paused => {
                if is_key_pressed(KeyCode::Space) {
                    game_state = GameState::Playing;
                }
                if is_key_pressed(KeyCode::Escape) {
                    game_state = GameState::MainMenu;
                }

                // Draw playing scene
                draw_playing_scene(
                    &player,
                    &mut player_sprite,
                    &player_texture,
                    &mut player_engine,
                    &bullets,
                    &mut bullet_sprite,
                    &bullet_texture,
                    &squares,
                    &mut explosions,
                    score,
                    high_score
                );

                let text = "Paused";
                let text_dimensions = measure_text(text, None, 50, 1.0);
                draw_text(
                    text,
                    screen_width() / 2.0 - text_dimensions.width / 2.0,
                    screen_height() / 2.0 - text_dimensions.height / 2.0 + text_dimensions.offset_y,
                    50.0,
                    WHITE
                );
            },
            GameState::GameOver => {
                if is_key_pressed(KeyCode::Space) {
                    squares.clear();
                    bullets.clear();
                    explosions.clear();
                    player_engine.config.emitting = true;
                    player.x = screen_width() / 2.0;
                    player.y = screen_height() / 2.0;
                    score = 0;
                    game_state = GameState::Playing;
                }
                if is_key_pressed(KeyCode::Escape) {
                    game_state = GameState::MainMenu;
                }
                
                // Draw playing scene
                draw_playing_scene(
                    &player,
                    &mut player_sprite,
                    &player_texture,
                    &mut player_engine,
                    &bullets,
                    &mut bullet_sprite,
                    &bullet_texture,
                    &squares,
                    &mut explosions,
                    score,
                    high_score
                );

                let game_over_text = "GAME OVER!";
                let go_text_dimensions = measure_text(game_over_text, None, 50, 1.0);
                draw_text(
                    game_over_text,
                    screen_width() / 2.0 - go_text_dimensions.width / 2.0,
                    screen_height() / 2.0 - go_text_dimensions.height / 2.0 + go_text_dimensions.offset_y,
                    50.0,
                    RED
                );
                if high_score > 0 && score == high_score {
                    fs::write("highscore.dat", high_score.to_string()).ok();

                    let congratulation_text = "Congratulations! You've achived the high score!";
                    let co_text_dimensions = measure_text(congratulation_text, None, 50, 1.0);
                    draw_text(
                        congratulation_text,
                        screen_width() / 2.0 - co_text_dimensions.width / 2.0,
                        screen_height() / 2.0 +
                            go_text_dimensions.height + go_text_dimensions.offset_y -
                            co_text_dimensions.height / 2.0 + co_text_dimensions.offset_y,
                        50.0,
                        RED
                    );
                }
            }
        }

        draw_text(
            format!("FPS: {}", get_fps()).as_str(),
            10.0,
            20.0,
            25.0,
            RED
        );

        next_frame().await
    }
}

