use macroquad::prelude as mq;

mod bullet;
mod camera;
mod colors;
mod consts;
mod deck;
mod enemy;
mod game_state;
mod hitbox;
mod mouse;
mod player;
mod timer;
mod util;
mod weapon;
mod world;

fn window_conf() -> mq::Conf {
    mq::Conf {
        window_title: "House of Cards".to_owned(),
        window_width: consts::WINDOW_START_SIZE as i32,
        window_height: consts::WINDOW_START_SIZE as i32,
        window_resizable: true,
        ..Default::default()
    }
}

enum LargeFont {
    Bounce(f32), // time_counter
    Static,
}

fn draw_overlay(
    overlay_color: mq::Color,
    large_text: &str,
    small_text: &str,
    font: mq::Font,
    large_font: LargeFont,
    scale: f32,
) {
    mq::draw_rectangle(
        0.0,
        0.0,
        mq::screen_width(),
        mq::screen_height(),
        overlay_color,
    );

    {
        let wrap_fn: fn(f32) -> f32 = |time_counter| (time_counter % 2.0 - 1.0).abs();
        let font_size = (match large_font {
            LargeFont::Bounce(time_counter) => {
                (scale * consts::LARGE_FONT_SIZE)
                    * (1.0
                        + consts::LARGE_FONT_BOUNCE_MAX
                            * wrap_fn(consts::LARGE_FONT_BOUNCE_SPEED * time_counter))
            }
            LargeFont::Static => scale * consts::LARGE_FONT_SIZE,
        })
        .round() as u16;
        let text_dims = mq::measure_text(large_text, Some(font), font_size, 1.0);

        let x = mq::screen_width() / 2.0 - text_dims.width / 2.0;
        let y = mq::screen_height() / 2.0 - text_dims.height / 2.0 + text_dims.offset_y / 2.0;

        mq::draw_text_ex(
            large_text,
            x,
            y,
            mq::TextParams {
                font,
                font_size,
                color: colors::NORD6,
                ..Default::default()
            },
        );
    }

    {
        let font_size = (scale * consts::FONT_SIZE).round() as u16;
        let text_dims = mq::measure_text(small_text, Some(font), font_size, 1.0);

        let x = mq::screen_width() / 2.0 - text_dims.width / 2.0;
        let y = mq::screen_height() / 2.0 - text_dims.height / 2.0
            + scale * consts::LARGE_FONT_SIZE / 2.0;

        mq::draw_text_ex(
            small_text,
            x,
            y,
            mq::TextParams {
                font,
                font_size,
                color: colors::NORD4,
                ..Default::default()
            },
        );
    }
}

async fn play() {
    let mut game_state = game_state::GameState::new();

    let mut player = player::Player::new(consts::AR);
    let mut score = 0;

    let mut camera = camera::Camera::new();

    let mut time_counter = 0.0;
    let mut fps_timer = timer::Timer::new_with_state(0.1, 1.0 / 60.0);

    let mut world = world::World::new();
    world.update_locations_to_build(&camera, consts::WINDOW_START_SIZE as f32);

    let mut enemy_manager = enemy::EnemyManager::new();

    let mut player_bullets: Vec<bullet::Bullet> = Vec::new();

    let cards_texture = mq::load_texture("./resources/nord-cards-transparent.png")
        .await
        .unwrap();

    let font = mq::load_ttf_font_from_bytes(include_bytes!(
        "../resources/AnnieUseYourTelescope-Regular.ttf"
    ))
    .unwrap();

    let mut deck = deck::Deck::new(cards_texture);

    let mut old_width = mq::screen_width();
    let mut old_height = mq::screen_height();

    let mut mouse_info = mouse::MouseInfo::new();
    mq::show_mouse(false);

    loop {
        mq::clear_background(consts::BACKGROUND_COLOR);

        let delta = mq::get_frame_time();
        time_counter += delta;

        if fps_timer.update(time_counter) {
            fps_timer.update_state(delta);
        }

        let scale = mq::screen_width().min(mq::screen_height());

        let resized = old_height != mq::screen_height() || old_width != mq::screen_width();
        if resized {
            old_width = mq::screen_width();
            old_height = mq::screen_height();
        }

        mouse_info.update(delta);

        //----------------------------------------------------------------------------//
        if game_state == game_state::GameState::Alive {
            let player_shot = player.handle_input(&mut mouse_info, delta);
            let camera_moved = camera.update(&player, delta);
            if camera_moved || resized {
                world.update_locations_to_build(&camera, scale);
            }

            if player_shot {
                let card = deck.draw_card();
                if let Some(card) = card {
                    let bullet = bullet::Bullet::new(
                        player.pos,
                        player.direction,
                        player.weapon.bullet_speed,
                        player.weapon.range,
                        bullet::BulletDamage::Card(card),
                    );
                    player_bullets.push(bullet);
                } else if consts::AUTO_RELOAD {
                    deck.combine();
                    player.weapon.reload();
                }
            }

            player_bullets
                .iter_mut()
                .for_each(|bullet| bullet.update(delta));

            for bullet in player_bullets.iter_mut() {
                for enemy in enemy_manager.enemies.iter_mut() {
                    if hitbox::rectangle_circle_collide(enemy, bullet) {
                        enemy.health -= match bullet.bullet_damage {
                            bullet::BulletDamage::Standard(damage) => damage,
                            bullet::BulletDamage::Card(card) => card.damage(),
                        };

                        bullet.remove();
                    }
                }
            }

            player_bullets.retain(bullet::Bullet::should_keep);

            let enemy_manager_update_result = enemy_manager.update(&mut player, delta);
            score += enemy_manager_update_result.enemies_killed;

            // heal or increase max health
            if enemy_manager_update_result.wave_finished {
                player.health += 1.0;
                player.max_health = player.max_health.max(player.health);

                score += consts::ENEMY_WAVE_SCORE(enemy_manager.wave);
            }

            if mq::is_key_pressed(mq::KeyCode::R) && !deck.is_full() {
                deck.combine();
                player.weapon.reload();
            }

            if player.health <= 0.0 {
                game_state = game_state::GameState::Dead;
                player.health = player.health.max(0.0);
            }
        }
        // do even if dead/in pause
        world.build_locations();
        //----------------------------------------------------------------------------//

        //----------------------------------------------------------------------------//
        world.draw(&camera, scale);
        player.draw(&camera, scale);
        enemy_manager.draw(&camera, &player, scale);
        for bullet in player_bullets.iter() {
            bullet.draw(&camera, scale);
        }
        deck.draw(&player.weapon, scale);
        mouse_info.draw(scale);

        {
            // Player HP bar
            let bar_width = scale * consts::PLAYER_HP_BAR_WIDTH;
            let bar_height = scale * consts::PLAYER_HP_BAR_HEIGHT;
            let bar_thickness = scale * consts::PLAYER_HP_BAR_THICKNESS;

            // center horizontally
            let x = mq::screen_width() / 2.0 - bar_width / 2.0;
            let y =
                mq::screen_height() - bar_height / 2.0 - scale * consts::PLAYER_HP_BAR_BOT_OFFSET;

            let hp = player.health;
            let max_hp = player.max_health;
            let hp_ratio = hp / max_hp;

            // background
            mq::draw_rectangle(x, y, bar_width, bar_height, colors::NORD6_ALPHA);
            // hp
            mq::draw_rectangle(x, y, bar_width * hp_ratio, bar_height, colors::NORD14);
            // background outline
            mq::draw_rectangle_lines(x, y, bar_width, bar_height, bar_thickness, colors::NORD6);
        }

        let texts = [
            (0.05, format!("FPS: {:.0}", 1.0 / fps_timer.get_state())),
            (-0.19, format!("Wave: {}", enemy_manager.wave)),
            (
                0.05,
                format!("Enemies left: {}", enemy_manager.enemies_left()),
            ),
            (0.05, format!("Score: {}", score)),
        ];
        let font_size = (scale * consts::FONT_SIZE).round() as u16;
        let x = scale * consts::FONT_SPACING;
        let mut y = scale * consts::FONT_SPACING;
        let color = colors::NORD6;
        for (extra_spacing, text) in texts.iter() {
            let text_dims = mq::measure_text(text, Some(font), font_size, 1.0);
            y += text_dims.offset_y;

            mq::draw_text_ex(
                text,
                x,
                y,
                mq::TextParams {
                    font,
                    font_size,
                    color,
                    ..Default::default()
                },
            );

            y += scale * consts::FONT_SIZE * extra_spacing;
        }

        if game_state == game_state::GameState::Dead {
            draw_overlay(
                colors::NORD0_BIG_ALPHA,
                "You died!",
                "Press R to restart",
                font,
                LargeFont::Bounce(time_counter),
                scale,
            );

            if mq::is_key_pressed(mq::KeyCode::R) {
                mq::next_frame().await;
                return;
            }
        } else if game_state == game_state::GameState::Paused {
            draw_overlay(
                colors::NORD0_BIG_ALPHA,
                "Paused",
                "Press Esc to unpause",
                font,
                LargeFont::Static,
                scale,
            );
        }

        if game_state != game_state::GameState::Dead
            && (mq::is_key_pressed(mq::KeyCode::Escape) || mq::is_key_pressed(mq::KeyCode::P))
        {
            game_state.toggle_pause();
        }
        //----------------------------------------------------------------------------//

        mq::next_frame().await
    }
}

#[macroquad::main(window_conf)]
async fn main() {
    mq::rand::srand(instant::now() as u64);

    loop {
        play().await;
    }
}
