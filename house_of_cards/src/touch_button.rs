use macroquad::prelude as mq;

pub enum SlotTouchButtonResult {
    Touched(usize),
    None,
}

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

    pub fn touched_selected(&mut self, touches: Vec<mq::Touch>) -> bool {
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

    pub fn touched_down(&mut self, touches: Vec<mq::Touch>) -> bool {
        for touch in touches {
            if touch.phase == mq::TouchPhase::Started && self.rect.contains(touch.position) {
                return true;
            }
        }
        false
    }
}
