use macroquad::prelude as mq;

mod colors;
mod world;
mod player;


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

    let mut scale = WINDOW_START_SIZE as f32;

    let mut player = player::Player::new();
    let mut world = world::World::new();


    loop {
        mq::clear_background(BACKGROUND_COLOR);

        scale = mq::screen_width().min(mq::screen_height());

        world.update(&player, scale);
        world.draw(&player, scale);

        
        mq::next_frame().await
    }

}
