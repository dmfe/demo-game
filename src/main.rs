use std::fs;
use macroquad::prelude::*;
use macroquad::rand::ChooseRandom;
use macroquad_particles::{self as particles, AtlasConfig, ColorCurve, Emitter, EmitterConfig};

mod resource_manager;
mod sound_manager;
mod game_object;

use resource_manager::ResourceManager;
use sound_manager::SoundManager;
use game_object::GameObject;

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

enum GameState {
    MainMenu,
    Playing,
    Paused,
    GameOver
}

fn draw_playing_scene(
    resource_manager: &ResourceManager,
    player: &mut GameObject,
    player_engine: &mut Emitter,
    bullets: &mut Vec<GameObject>,
    enemies: &mut Vec<GameObject>,
    explosions: &mut Vec<(Emitter, Vec2)>,
    score: u32,
    high_score: u32
) {
    // Draw Player
    player_engine.draw(vec2(
            player.x,
            player.y + player.height / 3.0
    ));
    player.draw(resource_manager);

    // Draw bullets
    for bullet in bullets {
        bullet.draw(resource_manager);
    }

    // Draw enemies
    for enemy in enemies {
        enemy.draw(resource_manager);
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
        lifetime: 1.2,
        lifetime_randomness: 0.3,
        explosiveness: 0.65,
        initial_direction_spread: 2.0 * std::f32::consts::PI,
        initial_velocity: 200.0,
        initial_velocity_randomness: 0.8,
        size: 16.0,
        size_randomness: 0.3,
        atlas: Some(AtlasConfig::new(5, 1, 0..)),
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

    // Resources initialization
    let mut resource_manager = ResourceManager::new();
    resource_manager.load_resources().await;
    let explosion_texture = resource_manager
        .get_texture(resource_manager::constants::EXPLOSION_TEX_ID).unwrap();

    // Sound Manager initialization
    let mut sound_manager = SoundManager::new(&resource_manager);

    let mut game_state = GameState::MainMenu;
    let mut enemies: Vec<GameObject> = vec![];
    let mut bullets: Vec<GameObject> = vec![];
    let mut explosions: Vec<(Emitter, Vec2)> = vec![];
    let mut player = GameObject {
        width: 64.0,
        height: 96.0,
        speed: MOVEMENT_SPEED,
        x: screen_width() / 2.0,
        y: screen_height() / 2.0,
        collided: false,
        texture_id: resource_manager::constants::PLAYER_TEX_ID.to_string(),
        sprite: resource_manager::animations::player_animation(),
        animation_num: 0,
        
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
                sound_manager.start_playing(resource_manager::constants::THEME_MUSIC, 0.3);

                if is_key_pressed(KeyCode::Escape) {
                    std::process::exit(0);
                }
                if is_key_pressed(KeyCode::Space) {
                    enemies.clear();
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
                sound_manager.start_playing(resource_manager::constants::THEME_MUSIC, 0.7);
                sound_manager.set_volume(resource_manager::constants::THEME_MUSIC, 0.7);

                let delta_time = get_frame_time();
                player.set_animation_num(0);

                if is_key_down(KeyCode::Right) || is_key_down(KeyCode::D) {
                    player.x += player.speed * delta_time;
                    direction_modifier += 0.05 * delta_time;

                    if get_time() - last_right_key_time >= SIDE_ANIMATION_SWITCH_SECONDS {
                        player.set_animation_num(4);
                    } else {
                        player.set_animation_num(3);
                    }
                }
                if is_key_pressed(KeyCode::Right) || is_key_pressed(KeyCode::D) {
                    last_right_key_time = get_time();
                }

                if is_key_down(KeyCode::Left) || is_key_down(KeyCode::A) {
                    player.x -= player.speed * delta_time;
                    direction_modifier -= 0.05 * delta_time;

                    if get_time() - last_left_key_time >= SIDE_ANIMATION_SWITCH_SECONDS {
                        player.set_animation_num(2);
                    } else {
                        player.set_animation_num(1);
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
                        bullets.push(GameObject {
                            width: 32.0,
                            height: 32.0,
                            x: player.x,
                            y: player.y - 24.0,
                            speed: player.speed * 2.0,
                            collided: false,
                            texture_id: resource_manager::constants::BULLET_TEX_ID.to_string(),
                            sprite: resource_manager::animations::bullet_animation(),
                            animation_num: 1,
                        });
                        sound_manager.play_once(resource_manager::constants::LASER_SOUND);
                        last_shot_time = current_time;
                    }
                }
                if is_key_pressed(KeyCode::Escape) {
                    game_state = GameState::Paused;
                }

                // Clamp X and Y of player circle to be within screen
                player.x = clamp(player.x, 0.0 + player.width, screen_width() - player.width);
                player.y = clamp(player.y, 0.0 + player.height, screen_height() - player.height);

                // Ganerate a new enemy 
                if rand::gen_range(0, 99) >= 90 {
                    let enemy_texture = resource_manager::constants::ENEMY_TEXTURES.choose().unwrap();
                    let enemy_sprite = match *enemy_texture {
                        resource_manager::constants::ENEMY_SMALL_TEX_ID =>
                            resource_manager::animations::enemy_small_animation(),
                        resource_manager::constants::ENEMY_MEDIUM_TEX_ID =>
                            resource_manager::animations::enemy_medium_animation(),
                        resource_manager::constants::ENEMY_BIG_TEX_ID =>
                            resource_manager::animations::enemy_big_animation(),
                        &_ => resource_manager::animations::enemy_small_animation(),
                    };
                    let size_mult = rand::gen_range(3.0, 5.0);
                    let enemy_frame = enemy_sprite.frame();
                    let enemy_width = enemy_frame.dest_size.x * size_mult;
                    let enemy_height = enemy_frame.dest_size.y * size_mult;
                    enemies.push(GameObject {
                        width: enemy_width,
                        height: enemy_height,
                        speed: rand::gen_range(300.0, 400.0),
                        x: rand::gen_range(enemy_width / 2.0, screen_width() - enemy_width / 2.0),
                        y: -enemy_height,
                        collided: false,
                        texture_id: enemy_texture.to_string(),
                        sprite: enemy_sprite,
                        animation_num: 0,
                    });
                }

                // Movement
                for enemy in &mut enemies {
                    enemy.y += enemy.speed * delta_time;
                }
                for bullet in &mut bullets {
                    bullet.y -= bullet.speed * delta_time;
                }

                // Check for collisions
                if enemies.iter().any(|enemy| player.collides_with(enemy)) {
                    game_state = GameState::GameOver;
                    player_engine.config.emitting = false;
                    continue;
                }
                for enemy in enemies.iter_mut() {
                    for bullet in bullets.iter_mut() {
                        if bullet.collides_with(enemy) {
                            bullet.collided = true;
                            enemy.collided = true;
                            score += enemy.height.round() as u32;
                            high_score = high_score.max(score);

                            // Start new explosion
                            explosions.push((
                                Emitter::new(EmitterConfig {
                                    amount: enemy.height.round() as u32,
                                    texture: Some(explosion_texture.clone()),
                                    ..particle_explosion()
                                }),
                                vec2(enemy.x, enemy.y)
                            ));
                            sound_manager.play_once(resource_manager::constants::EXPLOSION_SOUND);
                        }
                    }
                }

                // Remove shapes outside of the screen
                enemies.retain(|enemy| enemy.y < screen_height() + enemy.height);
                bullets.retain(|bullet| bullet.y > 0.0 - bullet.height);

                // Remove collided shaped
                enemies.retain(|enemy| !enemy.collided);
                bullets.retain(|bullet| !bullet.collided);

                // Remove the old explosions
                explosions.retain(|(explosion, _)| explosion.config.emitting);

                // Draw playing scene
                draw_playing_scene(
                    &resource_manager,
                    &mut player,
                    &mut player_engine,
                    &mut bullets,
                    &mut enemies,
                    &mut explosions,
                    score,
                    high_score
                );
            },
            GameState::Paused => {
                sound_manager.stop_playing(resource_manager::constants::THEME_MUSIC);

                if is_key_pressed(KeyCode::Space) {
                    game_state = GameState::Playing;
                }
                if is_key_pressed(KeyCode::Escape) {
                    game_state = GameState::MainMenu;
                }

                // Draw playing scene
                draw_playing_scene(
                    &resource_manager,
                    &mut player,
                    &mut player_engine,
                    &mut bullets,
                    &mut enemies,
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
                sound_manager.stop_playing(resource_manager::constants::THEME_MUSIC);

                if is_key_pressed(KeyCode::Space) {
                    enemies.clear();
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
                    &resource_manager,
                    &mut player,
                    &mut player_engine,
                    &mut bullets,
                    &mut enemies,
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

