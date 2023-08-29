use macroquad::prelude as mq;

mod bullet;
mod colors;
mod consts;
mod deck;
mod enemy;
mod game_state;
mod hitbox;
mod mouse;
mod player;
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

async fn play() {
    let mut game_state = game_state::GameState::new();

    let mut player = player::Player::new(consts::AR);
    let mut score = 0;

    let mut time_counter = 0.0;

    let mut world = world::World::new();
    world.update_locations_to_build(&player, consts::WINDOW_START_SIZE as f32);

    let mut enemy_manager = enemy::EnemyManager::new();

    let mut bullets: Vec<bullet::Bullet> = Vec::new();

    let cards_texture = mq::load_texture("./resources/nord-cards-transparent.png")
        .await
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

        let scale = mq::screen_width().min(mq::screen_height());

        let resized = old_height != mq::screen_height() || old_width != mq::screen_width();
        if resized {
            old_width = mq::screen_width();
            old_height = mq::screen_height();
        }

        mouse_info.update(delta);

        let handle_input_result = player.handle_input(&mut mouse_info, delta);

        if handle_input_result.moved || resized {
            world.update_locations_to_build(&player, scale);
        }
        world.build_locations();

        if handle_input_result.shot {
            let card = deck.draw_card();
            if let Some(card) = card {
                let bullet = bullet::Bullet::new(
                    player.pos,
                    player.direction,
                    player.weapon.bullet_speed,
                    player.weapon.range,
                    card,
                );
                bullets.push(bullet);
            } else if consts::AUTO_RELOAD {
                deck.combine();
                player.weapon.reload();
            }
        }

        bullets.iter_mut().for_each(|bullet| bullet.update(delta));

        for bullet in bullets.iter_mut() {
            for enemy in enemy_manager.enemies.iter_mut() {
                if hitbox::rectangle_circle_collide(enemy, bullet) {
                    enemy.health -= bullet.card.damage();
                    bullet.remove();
                }
            }
        }

        bullets.retain(bullet::Bullet::should_keep);

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

        world.draw(&player, scale);
        player.draw(scale);
        enemy_manager.draw(&player, scale);
        for bullet in bullets.iter() {
            bullet.draw(&player, scale);
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
            format!("FPS: {:.0}", 1.0 / delta),
            format!("Wave: {}", enemy_manager.wave),
            format!("Enemies left: {}", enemy_manager.enemies_left()),
            format!("Score: {}", score),
        ];
        let font_size = scale * consts::FONT_SIZE;
        let x = scale * consts::FONT_SPACING;
        let color = colors::NORD6;
        for (i, text) in texts.iter().enumerate() {
            let y = scale * (consts::FONT_SPACING + consts::FONT_SIZE / 2.0)
                + (font_size * consts::FONT_LINE_SPACING) * i as f32;
            mq::draw_text(text, x, y, font_size, color);
        }

        if game_state == game_state::GameState::Dead {
            mq::draw_rectangle(
                0.0,
                0.0,
                mq::screen_width(),
                mq::screen_height(),
                colors::NORD0_BIG_ALPHA,
            );

            {
                let text = "You died!";
                let wrap_fn: fn(f32) -> f32 = |time_counter| (time_counter % 2.0 - 1.0).abs();
                let font_size = ((scale * consts::DEATH_FONT_SIZE)
                    * (1.0
                        + consts::DEATH_FONT_BOUNCE_MAX
                            * wrap_fn(consts::DEATH_FONT_BOUNCE_SPEED * time_counter)))
                .round() as u16;
                // let font_size = scale * consts::DEATH_FONT_SIZE;
                let text_dims = mq::measure_text(text, None, font_size, 1.0);

                let x = mq::screen_width() / 2.0 - text_dims.width / 2.0;
                let y =
                    mq::screen_height() / 2.0 - text_dims.height / 2.0 + text_dims.offset_y / 2.0;

                mq::draw_text(text, x, y, font_size as f32, colors::NORD6);
            }

            {
                let text = "Press R to restart";
                let font_size = scale * consts::FONT_SIZE;
                let text_dims = mq::measure_text(text, None, font_size as u16, 1.0);

                let x = mq::screen_width() / 2.0 - text_dims.width / 2.0;
                let y = mq::screen_height() / 2.0 - text_dims.height / 2.0
                    + scale * consts::DEATH_FONT_SIZE / 2.0;

                mq::draw_text(text, x, y, font_size, colors::NORD4);
            }

            if mq::is_key_pressed(mq::KeyCode::R) {
                mq::next_frame().await;
                return;
            }
        }

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
