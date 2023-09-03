use futures::join;
use macroquad::audio as mq_audio;
use macroquad::prelude as mq;

mod bullet;
mod camera;
mod colors;
mod consts;
mod deck;
mod enemy;
mod game_state;
mod hitbox;
mod joystick;
mod mouse;
mod player;
mod powerup;
mod timer;
mod touch_button;
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

struct TouchControls {
    movement_joystick: joystick::Joystick,
    aim_joystick: joystick::Joystick,
    reload_button: touch_button::TouchButton,
    start_pause_button: touch_button::TouchButton,
    fullscreen_button: touch_button::TouchButton,
}

#[derive(Clone, Copy)]
struct Resources {
    cards_texture: mq::Texture2D,
    font: mq::Font,
    music: mq_audio::Sound,
}

fn create_touch_controls(scale: f32) -> TouchControls {
    let joystick_height = consts::JOYSTICK_HEIGHT * mq::screen_height();
    let joystick_y = mq::screen_height() - joystick_height;

    let movement_joystick = joystick::Joystick::new(
        consts::JOYSTICK_MAX_RADIUS * scale,
        mq::Rect::new(0.0, joystick_y, mq::screen_width() / 2.0, joystick_height),
        joystick::Side::Left,
    );
    let aim_joystick = joystick::Joystick::new(
        consts::JOYSTICK_MAX_RADIUS * scale,
        mq::Rect::new(
            mq::screen_width() / 2.0,
            joystick_y,
            mq::screen_width() / 2.0,
            joystick_height,
        ),
        joystick::Side::Right,
    );

    let reload_button_width = consts::RELOAD_BUTTON_WIDTH * scale;
    let reload_button_height = consts::RELOAD_BUTTON_HEIGHT * scale;
    let reload_button = touch_button::TouchButton::new(mq::Rect::new(
        mq::screen_width() - reload_button_width,
        0.0,
        reload_button_width,
        reload_button_height,
    ));

    let start_pause_button = touch_button::TouchButton::new(mq::Rect::new(
        0.0,
        0.0,
        consts::PAUSE_BUTTON_WIDTH * mq::screen_width(),
        consts::PAUSE_BUTTON_HEIGHT * mq::screen_height(),
    ));
    let fullscreen_button = touch_button::TouchButton::new(mq::Rect::new(
        0.0,
        0.0,
        mq::screen_width(),
        mq::screen_height(),
    ));

    TouchControls {
        movement_joystick,
        aim_joystick,
        reload_button,
        start_pause_button,
        fullscreen_button,
    }
}

async fn create_resources() -> Resources {
    mq::clear_background(consts::BACKGROUND_COLOR);
    mq::next_frame().await;

    let font_fut = mq::load_ttf_font(consts::FONT_PATH);
    mq::clear_background(consts::BACKGROUND_COLOR);
    let (_, font) = join!(mq::next_frame(), font_fut);
    let font = font.unwrap();

    let scale = mq::screen_width().min(mq::screen_height());
    mq::clear_background(consts::BACKGROUND_COLOR);
    draw_overlay(
        colors::NORD0_BIG_ALPHA,
        "House of Cards",
        vec!["Loading..."],
        font,
        LargeFont::Static,
        scale,
    );
    mq::next_frame().await;

    mq::clear_background(consts::BACKGROUND_COLOR);
    draw_overlay(
        colors::NORD0_BIG_ALPHA,
        "House of Cards",
        vec!["Loading..."],
        font,
        LargeFont::Static,
        scale,
    );

    let music_fut = mq_audio::load_sound(consts::MUSIC_PATH);
    let cards_texture_fut = mq::load_texture(consts::CARDS_TEXTURE_PATH);

    let (music, cards_texture, _) = join!(music_fut, cards_texture_fut, mq::next_frame());

    Resources {
        cards_texture: cards_texture.unwrap(),
        font,
        music: music.unwrap(),
    }
}

enum LargeFont {
    Bounce(f32), // time_counter
    Static,
}

fn draw_overlay(
    overlay_color: mq::Color,
    large_text: &str,
    small_texts: Vec<&str>,
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
        let font_size = (scale * consts::SMALL_FONT_SIZE).round() as u16;
        let mut y = mq::screen_height() / 2.0 + scale * consts::LARGE_FONT_SIZE / 2.0;
        let text_height = mq::measure_text(small_texts[0], Some(font), font_size, 1.0).height;
        for (i, small_text) in small_texts.iter().enumerate() {
            let text_dims = mq::measure_text(small_text, Some(font), font_size, 1.0);

            let x = mq::screen_width() / 2.0 - text_dims.width / 2.0;
            y -= text_height / 2.0;

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

            y += text_height / 2.0 + scale * consts::SMALL_FONT_SPACING;
            if i == 0 {
                y += scale * consts::SMALL_FONT_LARGE_SPACING;
            }
        }
    }
}

async fn play(resources: Resources) {
    let mut is_mobile = false;

    let mut game_state = game_state::GameStateManager::new();

    let mut player = player::Player::new(consts::AR);
    let mut score = 0;

    let mut powerups = powerup::Powerups::new();

    let mut camera = camera::Camera::new();

    let mut time_counter = 0.0;

    let mut fps_timer = timer::Timer::new_with_state(consts::FPS_TEXT_UPDATE_PERIOD, 1.0 / 60.0);

    let mut world = world::World::new();
    world.update_locations_to_build(&camera, consts::WINDOW_START_SIZE as f32);

    let mut enemy_manager = enemy::EnemyManager::new();

    let mut power_up_choices = powerup::Powerup::pick_three(powerup::Powerup::pick_stat);
    let mut need_click_after = 0.0;

    let mut player_bullets: Vec<bullet::Bullet> = Vec::new();

    mq_audio::stop_sound(resources.music);
    mq_audio::play_sound(
        resources.music,
        mq_audio::PlaySoundParams {
            looped: true,
            volume: 1.0,
        },
    );

    let mut deck = deck::Deck::new(resources.cards_texture);

    let mut old_width = mq::screen_width();
    let mut old_height = mq::screen_height();

    let mut mouse_info = mouse::MouseInfo::new();

    let mut auto_shoot = false;
    let mut auto_reload = true;

    let scale = mq::screen_width().min(mq::screen_height());

    let mut touch_controls = create_touch_controls(scale);

    loop {
        mq::clear_background(consts::BACKGROUND_COLOR);

        let delta = mq::get_frame_time();
        if delta > consts::MAX_DELTA {
            mq::next_frame().await;
            continue;
        }

        time_counter += delta;

        if let util::Ticked(true) = fps_timer.update(delta) {
            fps_timer.update_state(delta);
        }

        let mouse_shown = game_state.show_mouse() && !is_mobile;
        mq::show_mouse(mouse_shown);

        let scale = mq::screen_width().min(mq::screen_height());

        let mut should_update_locations_to_build = false;

        let resized = old_height != mq::screen_height() || old_width != mq::screen_width();
        if resized {
            old_width = mq::screen_width();
            old_height = mq::screen_height();

            should_update_locations_to_build = true;

            touch_controls = create_touch_controls(scale);
        }

        mouse_info.update(time_counter, delta);

        mq::simulate_mouse_with_touch(game_state.current_state() != game_state::GameState::Alive);
        let touches = mq::touches();

        is_mobile = is_mobile || !touches.is_empty();
        if is_mobile {
            mouse_info.set_active(false);
        }

        let movement_joystick_result = touch_controls.movement_joystick.update(touches.clone());
        let aim_joystick_result = touch_controls.aim_joystick.update(touches.clone());

        //----------------------------------------------------------------------------//
        if game_state.current_state() == game_state::GameState::Alive {
            let player_shot = player.handle_input(
                &mut mouse_info,
                movement_joystick_result,
                aim_joystick_result,
                &powerups,
                auto_shoot,
                delta,
            );

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
                } else if auto_reload {
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
                        let bullet::BulletHitResult {
                            damage,
                            stun_time,
                            heal_amount,
                        } = bullet.hit_result(&powerups);

                        if !(enemy.enemy_type == enemy::EnemyType::Super && damage == f32::INFINITY)
                        {
                            enemy.health -= damage;
                        }
                        enemy.enemy_stunned.time_remaining += stun_time;

                        player.health += heal_amount;
                        player.health = player.health.min(player.max_health);

                        bullet.hit();
                    }
                }
            }

            player_bullets.retain(bullet::Bullet::should_keep);

            let enemies_killed = enemy_manager.update(&mut player, delta);
            score += enemies_killed.count;

            player.xp += enemies_killed.count;
            if player.xp >= consts::XP_PER_LEVEL(player.level) {
                player.xp -= consts::XP_PER_LEVEL(player.level);
                player.level += 1;

                game_state.next(game_state::GameState::PowerupStat);
                power_up_choices = powerup::Powerup::pick_three(powerup::Powerup::pick_stat);
                need_click_after = time_counter;
                player.xp_bar_ratio = 1.0;
            }
            if enemies_killed.super_killed {
                game_state.next(game_state::GameState::PowerupCard);
                power_up_choices = powerup::Powerup::pick_three(powerup::Powerup::pick_card);
                need_click_after = time_counter;
            }

            if (mq::is_key_pressed(mq::KeyCode::R)
                || touch_controls.reload_button.touched(touches.clone()))
                && !deck.is_full()
            {
                deck.combine();
                player.weapon.reload();
            }

            if player.health <= 0.0 {
                game_state.next(game_state::GameState::Dead);
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
        player.draw_bars(resources.font, scale);
        powerups.draw(scale);
        if !mouse_shown {
            mouse_info.draw(scale);
        }
        if game_state.current_state() == game_state::GameState::Alive {
            touch_controls.movement_joystick.draw(is_mobile, scale);
            touch_controls.aim_joystick.draw(is_mobile, scale);
        } else {
            touch_controls.movement_joystick.reset();
            touch_controls.aim_joystick.reset();
        }

        {
            let text = format!("FPS: {:.0}", 1.0 / fps_timer.get_state());
            let font_size = (scale * consts::FPS_FONT_SIZE).round() as u16;
            let font_spacing = scale * consts::FPS_FONT_SPACING;
            let text_dims = mq::measure_text(&text, Some(resources.font), font_size, 1.0);

            let x = font_spacing;
            let y = text_dims.offset_y + font_spacing;

            mq::draw_text_ex(
                &text,
                x,
                y,
                mq::TextParams {
                    font: resources.font,
                    font_size,
                    color: colors::NORD6,
                    ..Default::default()
                },
            );
        }
        {
            let text = format!("Score: {}", score);
            let font_size = (scale * consts::SCORE_FONT_SIZE).round() as u16;
            let font_spacing = scale * consts::SCORE_FONT_SPACING;
            let text_dims = mq::measure_text(&text, Some(resources.font), font_size, 1.0);

            let x = (mq::screen_width() - text_dims.width) / 2.0;
            let y = text_dims.offset_y + font_spacing;

            mq::draw_text_ex(
                &text,
                x,
                y,
                mq::TextParams {
                    font: resources.font,
                    font_size,
                    color: colors::NORD6,
                    ..Default::default()
                },
            );
        }

        if game_state.current_state() == game_state::GameState::Dead {
            draw_overlay(
                colors::NORD0_BIG_ALPHA,
                "You died!",
                vec!["Press R to restart"],
                resources.font,
                LargeFont::Bounce(time_counter),
                scale,
            );

            if mq::is_key_pressed(mq::KeyCode::R)
                || touch_controls.fullscreen_button.touched(touches.clone())
            {
                mq::next_frame().await;
                return;
            }
        } else if game_state.current_state() == game_state::GameState::Paused {
            let small_texts = if is_mobile {
                vec!["Touch the screen to unpause"]
            } else {
                let auto_shoot_text = if auto_shoot {
                    "Auto shoot: on"
                } else {
                    "Auto shoot: off"
                };
                let auto_reload_text = if auto_reload {
                    "Auto reload: on"
                } else {
                    "Auto reload: off"
                };
                vec!["Press Esc to unpause", auto_shoot_text, auto_reload_text]
            };
            draw_overlay(
                colors::NORD0_BIG_ALPHA,
                "Paused",
                small_texts,
                resources.font,
                LargeFont::Static,
                scale,
            );
        } else if game_state.powerup() {
            player.update_bar_ratios(delta);

            powerup::Powerup::draw_outline(scale);
            let all_locations = powerup::PowerupPickLocation::all_locations();
            for (powerup, location) in power_up_choices.iter().zip(all_locations.iter()) {
                powerup.draw(*location, resources.font, scale);
            }

            let mut selected_powerup = None;
            let keys = [mq::KeyCode::Key1, mq::KeyCode::Key2, mq::KeyCode::Key3];
            for (i, (key, powerup)) in keys.iter().zip(power_up_choices.iter()).enumerate() {
                if mq::is_key_pressed(*key)
                    || powerup.clicked_on(all_locations[i], need_click_after, &mouse_info, scale)
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

                game_state.back();
            }
        }

        if mq::is_key_pressed(mq::KeyCode::Q) {
            auto_shoot = !auto_shoot;
        }
        if mq::is_key_pressed(mq::KeyCode::T) {
            auto_reload = !auto_reload;
        }

        if mq::is_key_pressed(mq::KeyCode::Escape)
            || mq::is_key_pressed(mq::KeyCode::P)
            || (game_state.current_state() == game_state::GameState::Paused
                && touch_controls.fullscreen_button.touched(touches.clone()))
            || (game_state.current_state() == game_state::GameState::Alive
                && touch_controls.start_pause_button.touched(touches.clone()))
        {
            game_state.toggle_pause();
            if game_state.current_state() == game_state::GameState::Paused {
                mq_audio::set_sound_volume(resources.music, 0.0);
            } else {
                // mq_audio::play_sound(resources.music, mq_audio::PlaySoundParams { looped: true, ..Default::default() });
                mq_audio::set_sound_volume(resources.music, 1.0);
            }
        }
        //----------------------------------------------------------------------------//

        mq::next_frame().await
    }
}

#[macroquad::main(window_conf)]
async fn main() {
    mq::rand::srand(instant::now() as u64);

    let resources = create_resources().await;

    loop {
        play(resources).await;
    }
}
