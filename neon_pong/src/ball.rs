use macroquad::prelude::*;
use crate::paddle::Paddle;

pub struct Ball {
    pub pos: Vec2,
    pub vel: Vec2,
    d: f32,
    r: f32
}

impl Ball {
    pub fn new(pos: Vec2, vel: Vec2) -> Ball {
        let d = 8.;
        let r = d / 2.;

        Ball {
            pos,
            vel,
            d,
            r,
        }
    }

    pub fn update_pos(&mut self) {
        self.pos += self.vel;
        if self.pos.y >= screen_height() || self.pos.y <= 0. {
            self.vel.y *= -1.;
        }
        // Remove this later
        if self.pos.x >= screen_width() || self.pos.x <= 0. {
            self.vel.x *= -1.;
        }
    }

    pub fn paddle_check(&mut self, paddles: [&Paddle; 2]) {
        for paddle in paddles {
            if paddle.is_player()
            && self.pos.y > paddle.pos.y 
            && self.pos.y < (paddle.pos.y + paddle.get_height()) 
            && self.pos.x < paddle.pos.x + paddle.get_width() {
                self.vel.x *= -1.;
            }
        }
    }

    pub fn draw(&self) {
        draw_circle(self.pos.x, self.pos.y, self.r, WHITE);
    }
}

