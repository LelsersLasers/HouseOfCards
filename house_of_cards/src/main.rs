use futures::join;
use macroquad::audio as mq_audio;
use macroquad::prelude as mq;
use touch_button::TouchButton;

mod bullet;
mod camera;
mod colors;
mod consts;
mod damage_number;
mod deck;
mod enemy;
mod game_state;
mod hand;
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
    start_pause_button: touch_button::TouchButton,
    select_slot_buttons: Vec<touch_button::TouchButton>,
    fullscreen_button: touch_button::TouchButton,
}

struct Resources {
    cards_texture: mq::Texture2D,
    chess_texture: mq::Texture2D,
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

    let mut select_slot_buttons = Vec::with_capacity(consts::HAND_CARD_COUNT);
    let hand::HandDrawDimensions {
        mut x,
        y,
        card_width,
        card_height,
        spacing,
    } = hand::Hand::hand_draw_dimensions(scale);

    for _ in 0..consts::HAND_CARD_COUNT {
        select_slot_buttons.push(touch_button::TouchButton::new(mq::Rect::new(
            x,
            y,
            card_width,
            card_height,
        )));
        x += card_width + spacing;
    }

    TouchControls {
        movement_joystick,
        aim_joystick,
        start_pause_button,
        select_slot_buttons,
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
        &[SmallText::Simple("Loading...")],
        &font,
        LargeFont::Static,
        scale,
    );
    mq::next_frame().await;

    mq::clear_background(consts::BACKGROUND_COLOR);
    draw_overlay(
        colors::NORD0_BIG_ALPHA,
        "House of Cards",
        &[SmallText::Simple("Loading...")],
        &font,
        LargeFont::Static,
        scale,
    );

    let music_fut = mq_audio::load_sound(consts::MUSIC_PATH);
    let cards_texture_fut = mq::load_texture(consts::CARDS_TEXTURE_PATH);
    let chess_texture_fut = mq::load_texture(consts::CHESS_TEXTURE_PATH);

    let (music, cards_texture, chess_texture, _) = join!(music_fut, cards_texture_fut, chess_texture_fut, mq::next_frame());

    Resources {
        cards_texture: cards_texture.unwrap(),
        chess_texture: chess_texture.unwrap(),
        font,
        music: music.unwrap(),
    }
}

struct ExtraUIButtons {
    music: Option<TouchButton>,
}

enum LargeFont {
    Bounce(f32), // time_counter
    Static,
}

enum SmallText<'a> {
    Simple(&'a str),
    Button(&'a str),
}
impl<'a> SmallText<'a> {
    fn as_str(&self) -> &str {
        match self {
            SmallText::Simple(text) => text,
            SmallText::Button(text) => text,
        }
    }
}

fn draw_controls_screen(
    font: &mq::Font,
    scale: f32,
) {
    let large_text = "Controls";

    let large_y = {
        let font_size = (scale * consts::LARGE_FONT_SIZE).round() as u16;
        let text_dims = mq::measure_text(large_text, Some(font), font_size, 1.0);

        let x = mq::screen_width() / 2.0 - text_dims.width / 2.0;
        let y = mq::screen_height() * consts::CONTROLS_LARGE_TEXT_SPACING - text_dims.height / 2.0 + text_dims.offset_y / 2.0;

        mq::draw_text_ex(
            large_text,
            x,
            y,
            mq::TextParams {
                font: Some(font),
                font_size,
                color: colors::NORD6,
                ..Default::default()
            },
        );

        y
    };

    let small_texts = [
        "MOVE: WASD, RMB, left joystick",
        "AIM: arrow keys, mouse, right joystick",
        "SHOOT: LMB, space, left joystick",
        "CHANGE CARD: 1/2/3, scroll wheel, tap card",
        "SELECT POWERUP/CARD: 8/9/0, tap powerup",
        "SWAP/CARD: enter, tap swap button",
        "DISCARD CARD: backspace, delete, tap discard button",
        "PAUSE: escape, P, tap top left corner",
        "UNPAUSE: escape, P, tap anywhere",
        "TOGGLE MUSIC: M or tap in pause menu",
        "TOGGLE AUTO-SHOOT: Q",
        "RESTART: R, tap anywhere",
        "",
        "START GAME: space, enter, LMB, tap anywhere",
    ];

    {
        let font_size = (scale * consts::SMALL_FONT_SIZE).round() as u16;
        let mut y = large_y + scale * consts::LARGE_FONT_SIZE;
        let first_text = small_texts[0];
        let text_height = mq::measure_text(first_text, Some(font), font_size, 1.0).height;
        for small_text in small_texts.iter() {
            let text_dims = mq::measure_text(small_text, Some(font), font_size, 1.0);

            let x = mq::screen_width() / 2.0 - text_dims.width / 2.0;
            y -= text_height / 2.0;

            mq::draw_text_ex(
                small_text,
                x,
                y,
                mq::TextParams {
                    font: Some(font),
                    font_size,
                    color: colors::NORD4,
                    ..Default::default()
                },
            );

            y += text_height / 2.0 + scale * consts::CONTROLS_SMALL_FONT_SPACING;
        }
    }
}

fn draw_overlay(
    overlay_color: mq::Color,
    large_text: &str,
    small_texts: &[SmallText],
    font: &mq::Font,
    large_font: LargeFont,
    scale: f32,
) -> ExtraUIButtons {
    let mut extra_ui_buttons = ExtraUIButtons { music: None };

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
                font: Some(font),
                font_size,
                color: colors::NORD6,
                ..Default::default()
            },
        );
    }

    {
        let font_size = (scale * consts::SMALL_FONT_SIZE).round() as u16;
        let mut y = mq::screen_height() / 2.0 + scale * consts::LARGE_FONT_SIZE / 2.0;
        let first_text = small_texts[0].as_str();
        let text_height = mq::measure_text(first_text, Some(font), font_size, 1.0).height;
        for (i, small_text) in small_texts.iter().enumerate() {
            let text = small_text.as_str();
            let text_dims = mq::measure_text(text, Some(font), font_size, 1.0);

            let x = mq::screen_width() / 2.0 - text_dims.width / 2.0;
            y -= text_height / 2.0;

            if let SmallText::Button(_) = small_text {
                // Rect::new(X, Y - dimensions.offset_y, dimensions.width, dimensions.height)

                let button_width =
                    text_dims.width + text_dims.height * consts::SMALL_FONT_BUTTON_PADDING;
                let button_height =
                    text_height + text_dims.height * consts::SMALL_FONT_BUTTON_PADDING;

                let button_x = x - text_dims.height * consts::SMALL_FONT_BUTTON_PADDING / 2.0;

                let raw_y = y - text_dims.offset_y;
                let button_y = raw_y - text_dims.height * consts::SMALL_FONT_BUTTON_PADDING / 2.0;

                mq::draw_rectangle(
                    button_x,
                    button_y,
                    button_width,
                    button_height,
                    colors::NORD14_BIG_ALPHA,
                );

                extra_ui_buttons.music = Some(TouchButton::new(mq::Rect::new(
                    button_x,
                    button_y,
                    button_width,
                    button_height,
                )));
            }

            mq::draw_text_ex(
                text,
                x,
                y,
                mq::TextParams {
                    font: Some(font),
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

    extra_ui_buttons
}

struct Continuity {
    play_music: bool,
}

async fn play(resources: &Resources, continuity: &mut Continuity) {
    let mut is_mobile = false;

    let mut game_state = game_state::GameStateManager::new();

    let mut score = 0;

    let mut powerups = powerup::Powerups::new();

    let mut camera = camera::Camera::new();

    let mut time_counter = 0.0;

    let mut fps_timer = timer::Timer::new_with_state(consts::FPS_TEXT_UPDATE_PERIOD, 1.0 / 60.0);

    let mut world = world::World::new();
    world.update_locations_to_build(&camera, consts::WINDOW_START_SIZE as f32);

    let mut enemy_manager = enemy::EnemyManager::new();

    let mut deck = deck::Deck::new();
    let hand = hand::Hand::new(&mut deck);
    let mut player = player::Player::new(hand);

    let mut powerup_choices = powerup::Powerup::pick_three();

    let mut card_choices = deck.draw_three_cards();
    let mut selected_card_choice = 0;

    let mut need_click_after = 0.0;

    let mut player_bullets: Vec<bullet::Bullet> = Vec::new();

    let mut damage_numbers: Vec<damage_number::DamageNumber> = Vec::new();

    mq_audio::stop_sound(&resources.music);
    mq_audio::play_sound(
        &resources.music,
        mq_audio::PlaySoundParams {
            looped: true,
            volume: 1.0,
        },
    );
    if !continuity.play_music {
        mq_audio::set_sound_volume(&resources.music, 0.0);
    }

    let mut old_width = mq::screen_width();
    let mut old_height = mq::screen_height();

    let mut mouse_info = mouse::MouseInfo::new();

    let mut auto_shoot = false;

    let scale = mq::screen_width().min(mq::screen_height());

    let mut touch_controls = create_touch_controls(scale);

    let mut controls_screen = true;

    loop {
        mq::clear_background(consts::BACKGROUND_COLOR);

        let scale = mq::screen_width().min(mq::screen_height());

        mq::simulate_mouse_with_touch(game_state.current_state() != game_state::GameState::Alive);
        let touches = mq::touches();

        if controls_screen {
            if mq::is_key_pressed(mq::KeyCode::Space)
                || mq::is_key_pressed(mq::KeyCode::Enter)
                || mq::is_mouse_button_pressed(mq::MouseButton::Left)
                || touch_controls.fullscreen_button.touched_selected(&touches) {
                controls_screen = false;
            }
            draw_controls_screen(&resources.font, scale);
            mq::next_frame().await;
            continue;
        }

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

        let mut should_update_locations_to_build = false;

        let resized = old_height != mq::screen_height() || old_width != mq::screen_width();
        if resized {
            old_width = mq::screen_width();
            old_height = mq::screen_height();

            should_update_locations_to_build = true;

            touch_controls = create_touch_controls(scale);
        }

        mouse_info.update(time_counter, delta);

        is_mobile = is_mobile || !touches.is_empty();
        if is_mobile {
            mouse_info.set_active(false);
        }

        let hand_keys = [
            mq::KeyCode::Key1,
            mq::KeyCode::Key2,
            mq::KeyCode::Key3,
        ];
        let scroll_wheel_target = if mouse_info.mouse_wheel_y() != 0 {
            Some(((player.hand.active as i32 + mouse_info.mouse_wheel_y()).rem_euclid(consts::HAND_CARD_COUNT as i32)) as usize)
        } else {
            None
        };
        
        let mut used_touch_ids = Vec::new();
        for (i, (key, slot_button)) in hand_keys
            .iter()
            .zip(touch_controls.select_slot_buttons.iter_mut())
            .enumerate()
        {
            if mq::is_key_pressed(*key) || scroll_wheel_target == Some(i) {
                player.hand.active = i;
            } else if let Some(id) = slot_button.touched_down(&touches) {
                player.hand.active = i;
                used_touch_ids.push(id);
            } else if game_state.current_state() == game_state::GameState::ChooseCard
                && util::clicked_on(slot_button.rect, need_click_after, &mouse_info, false)
            {
                player.hand.active = i;
            }
        }

        // match mouse_info.mouse_wheel_y() {
        //     1.0 => player.hand.scroll_up(),
        //     -1.0 => player.hand.scroll_down(),
        //     _ => {}
        // }

        let movement_joystick_result = touch_controls
            .movement_joystick
            .update(&touches, &used_touch_ids);
        let aim_joystick_result = touch_controls
            .aim_joystick
            .update(&touches, &used_touch_ids);
        //----------------------------------------------------------------------------//
        if game_state.current_state() == game_state::GameState::Alive {
            let player_shot = player.handle_input(player::PlayerInputInfo {
                mouse_info: &mut mouse_info,
                movement_joystick_result,
                aim_joystick_result,
                auto_shoot,
                scale,
                delta,
            });

            let camera_moved = camera.update(&player, delta);
            if let util::Moved(true) = camera_moved {
                should_update_locations_to_build = true;
            }

            if let util::Shot(true) = player_shot {
                let weapon = player.hand.active_weapon();
                let card = player.hand.active_card();
                let hp = match card.suit {
                    deck::Suit::Diamonds => powerups.diamonds_bullet_hp(),
                    _ => 1,
                };
                let bullet = bullet::Bullet::new(
                    player.pos,
                    player.direction,
                    weapon.bullet_speed,
                    weapon.range,
                    bullet::BulletDamage::Card(card),
                    hp,
                );
                player_bullets.push(bullet);
            }

            player_bullets
                .iter_mut()
                .for_each(|bullet| bullet.update(delta));

            'bullet: for bullet in player_bullets.iter_mut() {
                for enemy in enemy_manager.enemies.iter_mut() {
                    if bullet.not_already_hit(enemy.id)
                        && hitbox::rectangle_circle_collide(enemy, bullet)
                    {
                        let bullet::BulletHitResult {
                            damage,
                            stun_time,
                            heal_amount,
                        } = bullet.hit_result(&powerups);

                        enemy.health -= damage;
                        let enemy_dn_text = if damage.is_infinite() {
                            "∞".to_owned()
                        } else {
                            format!("{}", damage).to_owned()
                        };
                        damage_numbers.push(damage_number::DamageNumber::new(
                            enemy_dn_text,
                            consts::DAMAGE_NUMBER_TIME,
                            enemy.pos,
                            damage_number::DamageNumberColor::EnemyDamage,
                        ));
                        enemy.enemy_stunned.time_remaining += stun_time;

                        player.health += heal_amount;
                        player.health = player.health.min(player.max_health);

                        if heal_amount > 0.0 {
                            damage_numbers.push(damage_number::DamageNumber::new(
                                format!("+{}", heal_amount).to_owned(),
                                consts::DAMAGE_NUMBER_TIME,
                                player.pos,
                                damage_number::DamageNumberColor::PlayerHeal,
                            ));
                        }

                        bullet.hit(enemy.id);
                        continue 'bullet;
                    }
                }
            }
            

            player_bullets.retain(bullet::Bullet::should_keep);

            let max_dist = mq::Vec2::new(mq::screen_width(), mq::screen_height()).length();
            let tile_size = scale / consts::TILES_PER_SCALE as f32;
            let max_tiles = max_dist / tile_size;
            let (enemies_killed, new_damage_numbers) = enemy_manager.update(&mut player, max_tiles, delta);
            score += enemies_killed.count;

            damage_numbers.extend(new_damage_numbers);
            damage_numbers.iter_mut().for_each(|dn| dn.update(delta));
            damage_numbers.retain(damage_number::DamageNumber::should_keep);

            player.xp += enemies_killed.count;
            if player.xp >= consts::XP_PER_LEVEL(player.level) {
                player.xp -= consts::XP_PER_LEVEL(player.level);
                player.level += 1;

                game_state.next(game_state::GameState::ChooseCard);
                card_choices = deck.draw_three_cards();
                need_click_after = time_counter;
                player.xp_bar_ratio = 1.0;
            }
            if enemies_killed.super_killed {
                game_state.next(game_state::GameState::PowerupCard);
                powerup_choices = powerup::Powerup::pick_three();
                need_click_after = time_counter;
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
        enemy_manager.draw(&camera, &resources.chess_texture, scale);
        for bullet in player_bullets.iter() {
            bullet.draw(&camera, scale);
        }
        for damage_number in damage_numbers.iter() {
            damage_number.draw(&camera, &resources.font, scale);
        }
        enemy_manager.draw_hp_bars(&camera, scale);
        player.draw_bars(&resources.font, scale);
        let hand_top_y = player.hand.draw(&resources.cards_texture, &resources.font, scale);
        powerups.draw(&resources.cards_texture, scale);

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
            let text_dims = mq::measure_text(&text, Some(&resources.font), font_size, 1.0);

            let x = font_spacing;
            let y = text_dims.offset_y + font_spacing;

            mq::draw_text_ex(
                &text,
                x,
                y,
                mq::TextParams {
                    font: Some(&resources.font),
                    font_size,
                    color: colors::NORD6,
                    ..Default::default()
                },
            );
        }
        let score_text_bottom_y = {
            let text = format!("Score: {}", score);
            let font_size = (scale * consts::SCORE_FONT_SIZE).round() as u16;
            let font_spacing = scale * consts::SCORE_FONT_SPACING;
            let text_dims = mq::measure_text(&text, Some(&resources.font), font_size, 1.0);

            let x = (mq::screen_width() - text_dims.width) / 2.0;
            let y = text_dims.offset_y + font_spacing;

            mq::draw_text_ex(
                &text,
                x,
                y,
                mq::TextParams {
                    font: Some(&resources.font),
                    font_size,
                    color: colors::NORD6,
                    ..Default::default()
                },
            );

            y - text_dims.offset_y + text_dims.height
        };

        let mut extra_ui_buttons = ExtraUIButtons { music: None };
        if game_state.current_state() == game_state::GameState::Dead {
            player.update_bar_ratios(delta);

            draw_overlay(
                colors::NORD0_BIG_ALPHA,
                "You died!",
                &[SmallText::Simple("Press R to restart")],
                &resources.font,
                LargeFont::Bounce(time_counter),
                scale,
            );

            if mq::is_key_pressed(mq::KeyCode::R)
                || touch_controls.fullscreen_button.touched_selected(&touches)
            {
                mq::next_frame().await;
                return;
            }
        } else if game_state.current_state() == game_state::GameState::Paused {
            let small_texts = if is_mobile {
                let music_button = if continuity.play_music {
                    SmallText::Button("Music: on")
                } else {
                    SmallText::Button("Music: off")
                };
                vec![
                    SmallText::Simple("Touch the screen to unpause"),
                    music_button,
                ]
            } else {
                let auto_shoot_text = SmallText::Simple(if auto_shoot {
                    "Auto shoot: on"
                } else {
                    "Auto shoot: off"
                });
                let music_text = SmallText::Simple(if continuity.play_music {
                    "Music: on"
                } else {
                    "Music: off"
                });
                vec![
                    SmallText::Simple("Press Esc to unpause"),
                    auto_shoot_text,
                    music_text,
                ]
            };
            extra_ui_buttons = draw_overlay(
                colors::NORD0_BIG_ALPHA,
                "Paused",
                &small_texts,
                &resources.font,
                LargeFont::Static,
                scale,
            );
        } else if game_state.current_state == game_state::GameState::PowerupCard {
            player.update_bar_ratios(delta);

            let powerup_rects = powerup::draw_powerup_choices(
                &powerup_choices,
                &resources.font,
                score_text_bottom_y,
                hand_top_y,
                scale,
            );

            let mut selected_powerup = None;
            let keys = [mq::KeyCode::Key8, mq::KeyCode::Key9, mq::KeyCode::Key0];
            for (i, (key, rect)) in keys.iter().zip(powerup_rects.iter()).enumerate() {
                if mq::is_key_pressed(*key)
                    || util::clicked_on(*rect, need_click_after, &mouse_info, true)
                {
                    selected_powerup = Some(powerup_choices[i]);
                }
            }

            if let Some(powerup) = selected_powerup {
                powerups.add(powerup);
                game_state.back();
            }
        } else if game_state.current_state == game_state::GameState::ChooseCard {
            player.update_bar_ratios(delta);

            let card_choices_button_rects = hand::draw_card_choices(
                &card_choices,
                &resources.cards_texture,
                &resources.font,
                selected_card_choice,
                score_text_bottom_y,
                hand_top_y,
                scale,
            );

            let keys = [mq::KeyCode::Key8, mq::KeyCode::Key9, mq::KeyCode::Key0];
            for (i, (key, rect)) in keys
                .iter()
                .zip(card_choices_button_rects.cards.iter())
                .enumerate()
            {
                if mq::is_key_pressed(*key)
                    || util::clicked_on(*rect, need_click_after, &mouse_info, false)
                {
                    selected_card_choice = i;
                }
            }

            if mq::is_key_pressed(mq::KeyCode::Enter)
                || util::clicked_on(
                    card_choices_button_rects.swap_button,
                    need_click_after,
                    &mouse_info,
                    true,
                )
            {
                player.hand.set_card(card_choices[selected_card_choice]);
                game_state.back();
            } else if mq::is_key_pressed(mq::KeyCode::Backspace)
                || mq::is_key_pressed(mq::KeyCode::Delete)
                || util::clicked_on(
                    card_choices_button_rects.discard_button,
                    need_click_after,
                    &mouse_info,
                    true,
                )
            {
                game_state.back();
            }
        }

        if mq::is_key_pressed(mq::KeyCode::Q) {
            auto_shoot = !auto_shoot;
        }
        let music_toggle_pressed_id = extra_ui_buttons.music.and_then(|mut button| button.touched_down(&touches));
        if mq::is_key_pressed(mq::KeyCode::M) || music_toggle_pressed_id.is_some()
        {
            continuity.play_music = !continuity.play_music;
            if continuity.play_music {
                mq_audio::set_sound_volume(&resources.music, 1.0);
            } else {
                mq_audio::set_sound_volume(&resources.music, 0.0);
            }
        }
        if mq::is_key_pressed(mq::KeyCode::Escape)
            || mq::is_key_pressed(mq::KeyCode::P)
            || (game_state.current_state() == game_state::GameState::Paused
                && touch_controls.fullscreen_button.touched_selected_not_used(&touches, music_toggle_pressed_id))
            || (game_state.current_state() == game_state::GameState::Alive
                && touch_controls.start_pause_button.touched_down(&touches).is_some())
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

    let resources = create_resources().await;
    let mut continuity = Continuity { play_music: true };

    loop {
        play(&resources, &mut continuity).await;
    }
}
