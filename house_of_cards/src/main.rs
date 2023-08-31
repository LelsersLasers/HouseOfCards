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
mod powerup;
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
    // let mut game_state = game_state::GameState::new();
    let mut game_state = game_state::GameState::Powerup;

    let mut player = player::Player::new(consts::AR);
    let mut score = 0;

    let mut powerups = powerup::Powerups::new();

    let mut camera = camera::Camera::new();

    let mut time_counter = 0.0;

    let mut fps_timer = timer::Timer::new_with_state(consts::FPS_TEXT_UPDATE_PERIOD, 1.0 / 60.0);

    let mut world = world::World::new();
    world.update_locations_to_build(&camera, consts::WINDOW_START_SIZE as f32);

    let mut enemy_manager = enemy::EnemyManager::new();

    let mut power_up_choices = powerup::Powerup::pick_three();

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

    loop {
        mq::clear_background(consts::BACKGROUND_COLOR);

        let delta = mq::get_frame_time();
        time_counter += delta;

        if let util::Ticked(true) = fps_timer.update(delta) {
            fps_timer.update_state(delta);
        }

        let mouse_shown = game_state == game_state::GameState::Powerup;
        mq::show_mouse(mouse_shown);

        let scale = mq::screen_width().min(mq::screen_height());

        let mut should_update_locations_to_build = false;

        let resized = old_height != mq::screen_height() || old_width != mq::screen_width();
        if resized {
            old_width = mq::screen_width();
            old_height = mq::screen_height();

            should_update_locations_to_build = true;
        }

        mouse_info.update(delta);

        //----------------------------------------------------------------------------//
        if game_state == game_state::GameState::Alive {
            let player_shot = player.handle_input(&mut mouse_info, &powerups, delta);
            let camera_moved = camera.update(&player, delta);
            if let util::Moved(true) = camera_moved {
                should_update_locations_to_build = true;
            }

            if let util::Shot(true) = player_shot {
                let card = deck.draw_card();
                if let Some(card) = card {
                    let bullet = bullet::Bullet::new(
                        player.pos,
                        player.direction,
                        player.weapon.bullet_speed,
                        player.weapon.range,
                        bullet::BulletDamage::Card(card),
                        powerups.diamonds_bullet_hp(),
                    );
                    player_bullets.push(bullet);
                } else if consts::AUTO_RELOAD {
                    deck.combine();
                    player.weapon.reload(&powerups);
                }
            }

            player_bullets
                .iter_mut()
                .for_each(|bullet| bullet.update(delta));

            for bullet in player_bullets.iter_mut() {
                for enemy in enemy_manager.enemies.iter_mut() {
                    if hitbox::rectangle_circle_collide(enemy, bullet) {
                        let bullet::BulletHitResult {
                            damage,
                            stun_time,
                            heal_amount,
                        } = bullet.hit_result(&powerups);

                        enemy.health -= damage;
                        enemy.enemy_stunned.time_remaining += stun_time;

                        player.health += heal_amount;
                        player.health = player.health.min(player.max_health);

                        bullet.hit();
                    }
                }
            }

            player_bullets.retain(bullet::Bullet::should_keep);

            let enemies_killed = enemy_manager.update(&mut player, delta);
            score += enemies_killed.0;

            player.xp += enemies_killed.0;
            if player.xp >= consts::XP_PER_LEVEL(player.level) {
                player.xp -= consts::XP_PER_LEVEL(player.level);
                player.level += 1;

                game_state = game_state::GameState::Powerup;
                power_up_choices = powerup::Powerup::pick_three();
            }

            if mq::is_key_pressed(mq::KeyCode::R) && !deck.is_full() {
                deck.combine();
                player.weapon.reload(&powerups);
            }

            if player.health <= 0.0 {
                game_state = game_state::GameState::Dead;
                player.health = player.health.max(0.0);
            }
        }
        // do even if dead/in pause
        if should_update_locations_to_build {
            world.update_locations_to_build(&camera, scale);
        }
        world.build_locations();
        //----------------------------------------------------------------------------//

        //----------------------------------------------------------------------------//
        world.draw(&camera, scale);
        player.draw(&camera, scale);
        enemy_manager.draw(&camera, scale);
        for bullet in player_bullets.iter() {
            bullet.draw(&camera, scale);
        }
        enemy_manager.draw_hp_bars(&camera, scale);
        deck.draw(&player.weapon, scale);
        if !mouse_shown {
            mouse_info.draw(scale);
        }
        player.draw_bars(font, scale);

        let texts = [
            (0.05, format!("FPS: {:.0}", 1.0 / fps_timer.get_state())),
            // (-0.1, format!("Wave: {}", enemy_manager.wave)),
            // (
            //     0.05,
            //     format!("Enemies alive: {}", enemy_manager.enemies.len()),
            // ),
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
        } else if game_state == game_state::GameState::Powerup {
            powerup::Powerup::draw_outline(scale);
            let all_locations = powerup::PowerupPickLocation::all_locations();
            for (powerup, location) in power_up_choices.iter().zip(all_locations.iter()) {
                powerup.draw(*location, font, scale);
            }

            let mut selected_powerup = None;
            let keys = [mq::KeyCode::Key1, mq::KeyCode::Key2, mq::KeyCode::Key3];
            for (i, (key, powerup)) in keys.iter().zip(power_up_choices.iter()).enumerate() {
                if mq::is_key_pressed(*key)
                    || powerup.clicked_on(all_locations[i], &mouse_info, scale)
                {
                    selected_powerup = Some(power_up_choices[i]);
                }
            }

            if let Some(powerup) = selected_powerup {
                powerups.add(powerup);

                if powerup == powerup::Powerup::Health {
                    player.health += consts::HEALTH_ADD;
                    player.max_health += consts::HEALTH_ADD;
                }

                game_state = game_state::GameState::Alive;
            }
        }

        if mq::is_key_pressed(mq::KeyCode::Escape) || mq::is_key_pressed(mq::KeyCode::P) {
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
