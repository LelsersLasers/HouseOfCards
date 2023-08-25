use consts::WINDOW_START_SIZE;
use macroquad::prelude as mq;

mod colors;
mod consts;
mod player;
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

    let mut player = player::Player::new();

    let mut world = world::World::new();
    world.update_locations_to_build(&player, WINDOW_START_SIZE as f32);

    let mut old_width = mq::screen_width();
    let mut old_height = mq::screen_height();

    loop {
        mq::clear_background(consts::BACKGROUND_COLOR);

        let delta = mq::get_frame_time();

        let scale = mq::screen_width().min(mq::screen_height());

        let resized = old_height != mq::screen_height() || old_width != mq::screen_width();
        if resized {
            old_width = mq::screen_width();
            old_height = mq::screen_height();
        }

        let moved = player.handle_movement(delta);

        if moved || resized {
            world.update_locations_to_build(&player, scale);
        }
        world.build_locations();

        world.draw(&player, scale);

        player.draw(scale);

        {
            let fps = 1.0 / delta;
            let text = format!("FPS: {:.0}", fps);
            let x = scale * 0.02;
            let y = scale * (0.02 + 0.075 / 2.0);
            let font_size = scale * 0.075;
            let color = colors::NORD6;

            mq::draw_text(&text, x, y, font_size, color);
        }

        mq::next_frame().await
    }
}
