use macroquad::prelude as mq;

mod bullet;
mod colors;
mod consts;
mod deck;
mod mouse;
mod player;
mod weapon;
mod world;

/*
    Scale is the size of the window, in pixels.
    Player position is in tiles.
    10 tiles fit in scale.
*/

fn window_conf() -> mq::Conf {
    mq::Conf {
        window_title: "House of Cards".to_owned(),
        window_width: consts::WINDOW_START_SIZE as i32,
        window_height: consts::WINDOW_START_SIZE as i32,
        window_resizable: true,
        ..Default::default()
    }
}

#[macroquad::main(window_conf)]
async fn main() {
    mq::rand::srand(instant::now() as u64);

    let mut player = player::Player::new(consts::AR);

    let mut world = world::World::new();
    world.update_locations_to_build(&player, consts::WINDOW_START_SIZE as f32);

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
            }
        }

        bullets.iter_mut().for_each(|bullet| bullet.update(delta));
        bullets.retain(bullet::Bullet::should_remove);

        if mq::is_key_pressed(mq::KeyCode::R) && !deck.is_full() {
            deck.combine();
            player.weapon.reload();
        }

        world.draw(&player, scale);
        player.draw(scale);
        for bullet in bullets.iter() {
            bullet.draw(&player, scale);
        }
        deck.draw(&player.weapon, scale);
        mouse_info.draw(scale);

        {
            let fps = 1.0 / delta;
            let text = format!("FPS: {:.0}", fps);
            let x = scale * consts::FPS_SPACING;
            let y = scale * (consts::FPS_SPACING + consts::FPS_FONT_SIZE / 2.0);
            let font_size = scale * consts::FPS_FONT_SIZE;
            let color = colors::NORD6;

            mq::draw_text(&text, x, y, font_size, color);
        }

        mq::next_frame().await
    }
}
