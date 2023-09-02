use macroquad::prelude as mq;

pub struct TouchButton {
    rect: mq::Rect,
    touch_id: Option<u64>,
}

impl TouchButton {
    pub fn new(rect: mq::Rect) -> Self {
        Self {
            rect,
            touch_id: None,
        }
    }

    pub fn touched(&mut self, touches: Vec<mq::Touch>) -> bool {
        for touch in touches {
            if let Some(id) = self.touch_id {
                if id == touch.id && touch.phase == mq::TouchPhase::Ended {
                    self.touch_id = None;
                    return self.rect.contains(touch.position);
                }
            } else if touch.phase == mq::TouchPhase::Started && self.rect.contains(touch.position) {
                self.touch_id = Some(touch.id);
            }
        }
        false
    }
    pub fn draw(&self) {
        mq::draw_rectangle(
            self.rect.x,
            self.rect.y,
            self.rect.w,
            self.rect.h,
            mq::Color::new(0.0, 0.0, 0.0, 0.5),
        );
    }
}
