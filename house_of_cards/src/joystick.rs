use macroquad::prelude as mq;

use crate::colors;

#[derive(Clone, Copy)]
pub struct JoystickUpdateResult {
    pub pos: mq::Vec2, // normalized
    pub moved: bool,
    pub active: bool,
}

#[derive(PartialEq, Eq)]
pub enum Side {
    Left,
    Right,
}

pub struct Joystick {
    touch_id: Option<u64>,
    current_center: mq::Vec2,
    radius_max: f32,
    start: mq::Rect,
    side: Side,
    last_result: JoystickUpdateResult,
}

impl Joystick {
    pub fn new(radius_max: f32, start: mq::Rect, side: Side) -> Self {
        let mut joystick = Self {
            touch_id: None,
            current_center: start.center(),
            radius_max,
            start,
            side,
            last_result: JoystickUpdateResult {
                pos: mq::Vec2::ZERO,
                moved: false,
                active: false,
            },
        };
        joystick.reset_center();

        joystick
    }

    fn reset_center(&mut self) {
        let offset = if self.side == Side::Left {
            mq::Vec2::new(-self.start.w / 4.0, self.start.h / 6.0)
        } else {
            mq::Vec2::new(self.start.w / 4.0, self.start.h / 6.0)
        };
        self.current_center = self.start.center() + offset;
    }

    pub fn reset(&mut self) {
        self.touch_id = None;
        self.reset_center();
    }

    pub fn update(
        &mut self,
        touches: &[mq::Touch],
        used_touch_ids: &[u64],
    ) -> JoystickUpdateResult {
        let mut pos = mq::Vec2::ZERO;
        let mut active = false;

        for touch in touches {
            if let Some(id) = self.touch_id {
                if id == touch.id {
                    if touch.phase == mq::TouchPhase::Ended {
                        self.touch_id = None;
                        self.reset_center();
                    } else {
                        let offset = touch.position - self.current_center;
                        let offset_norm = offset.normalize_or_zero();
                        let offset_len = offset.length();
                        let offset_len = offset_len.clamp(0.0, self.radius_max);

                        pos = offset_norm * (offset_len / self.radius_max);
                        active = true;
                        break;
                    }
                }
            } else if touch.phase == mq::TouchPhase::Started
                && self.start.contains(touch.position)
                && !used_touch_ids.contains(&touch.id)
            {
                self.touch_id = Some(touch.id);
                self.current_center = touch.position;
                active = true;
            }
        }

        let moved = pos != mq::Vec2::ZERO;

        let result = JoystickUpdateResult { pos, moved, active };
        self.last_result = result;

        result
    }

    pub fn draw(&self, is_mobile: bool, scale: f32) {
        if is_mobile {
            let thickness = scale * crate::consts::JOYSTICK_THICKNESS;
            let ball = scale * crate::consts::JOYSTICK_BALL_SIZE;

            mq::draw_circle_lines(
                self.current_center.x,
                self.current_center.y,
                self.radius_max,
                thickness,
                colors::NORD4,
            );

            let ball_center = self.current_center + self.last_result.pos * self.radius_max;
            mq::draw_circle(ball_center.x, ball_center.y, ball, colors::NORD4);
        }
    }
}
