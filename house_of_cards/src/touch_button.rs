use macroquad::prelude as mq;

pub struct TouchButton {
    pub rect: mq::Rect,
    touch_id: Option<u64>,
}

impl TouchButton {
    pub fn new(rect: mq::Rect) -> Self {
        Self {
            rect,
            touch_id: None,
        }
    }

    pub fn touched_selected_not_used(&mut self, touches: &[mq::Touch], used_id: Option<u64>) -> bool {
        for touch in touches {
            if let Some(id) = self.touch_id {
                if id == touch.id && touch.phase == mq::TouchPhase::Ended {
                    self.touch_id = None;
                    return self.rect.contains(touch.position);
                }
            } else if touch.phase == mq::TouchPhase::Started && self.rect.contains(touch.position) && used_id != Some(touch.id) {
                self.touch_id = Some(touch.id);
            }
        }
        false
    }

    pub fn touched_selected(&mut self, touches: &[mq::Touch]) -> bool {
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

    pub fn touched_down(&mut self, touches: &[mq::Touch]) -> Option<u64> {
        for touch in touches {
            if touch.phase == mq::TouchPhase::Started && self.rect.contains(touch.position) {
                return Some(touch.id);
            }
        }
        None
    }
}
