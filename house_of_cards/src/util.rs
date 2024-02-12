use macroquad::prelude as mq;

use crate::mouse;

// pub fn rad_to_deg(rad: f32) -> f32 {
//     rad * 180.0 / std::f32::consts::PI
// }

pub fn clicked_on(
    rect: mq::Rect,
    need_click_after: f32,
    mouse_info: &mouse::MouseInfo,
    on_release: bool,
) -> bool {
    if need_click_after > mouse_info.last_click_time() {
        return false;
    }

    let mouse_pos_click = mouse_info.get_last_click();
    let mouse_pos_now = mouse_info.get_last_pos();

    (mouse_info.mouse_released() || !on_release)
        && rect.contains(mouse_pos_click)
        && rect.contains(mouse_pos_now)
}

pub struct Shot(pub bool);
pub struct Moved(pub bool);
pub struct Ticked(pub bool);
