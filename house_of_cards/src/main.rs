use macroquad::prelude as mq;

mod colors;
mod player;
mod world;

const WINDOW_START_SIZE: u32 = 800;
const BACKGROUND_COLOR: mq::Color = colors::NORD1;
const TILES_PER_SCALE: u32 = 10;

/*
    Scale is the size of the window, in pixels.
    Player position is in tiles.
    10 tiles fit in scale.
*/

fn window_conf() -> mq::Conf {
    mq::Conf {
        window_title: "House of Cards".to_owned(),
        window_width: WINDOW_START_SIZE as i32,
        window_height: WINDOW_START_SIZE as i32,
        window_resizable: true,
        ..Default::default()
    }
}

#[macroquad::main(window_conf)]
async fn main() {
    mq::rand::srand(instant::now() as u64);

    let player = player::Player::new();
    let mut world = world::World::new();

    loop {
        mq::clear_background(BACKGROUND_COLOR);

        let delta = mq::get_frame_time();

        let scale = mq::screen_width().min(mq::screen_height());

        world.update_locations_to_build(&player, scale);
        world.build_locations();

        world.draw(&player, scale);

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
