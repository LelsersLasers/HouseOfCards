pub fn rad_to_deg(rad: f32) -> f32 {
    rad * 180.0 / std::f32::consts::PI
}

pub struct Shot(pub bool);
pub struct Moved(pub bool);
pub struct Ticked(pub bool);
