#![allow(dead_code)]

use crate::paddle::Paddle;
use macroquad::prelude::*;

pub struct Ball {
    pub pos: Vec2,
    pub vel: Vec2,
    r: f32,
    speed: f32,
}

impl Ball {
    pub fn new(pos: Vec2, vel: Vec2) -> Ball {
        let r = 8.;
        let speed = 200.;

        Ball { pos, vel, r, speed }
    }

    pub fn update_pos(&mut self, scores: &mut[usize; 2]) -> [usize; 2] {
        let d_time = get_frame_time();
        self.pos += self.vel * d_time * self.speed;
        if self.pos.y >= screen_height() || self.pos.y <= 0. {
            self.vel.y *= -1.;
        }
        // TODO: Replace with new round logic
        if self.pos.x < -75. {
            self.pos = Vec2::new(screen_width() / 2., screen_height() / 2.);
            scores[0] += 1;
            *scores
        } else if self.pos.x > screen_width() + 75. {
            self.pos = Vec2::new(screen_width() / 2., screen_height() / 2.);
            scores[1] += 1;
            *scores
        } else {
            *scores
        }
    }

    pub fn paddle_check(&mut self, paddles: [&Paddle; 2]) {
        for paddle in paddles {
            if paddle.is_player()
                && self.pos.y > paddle.pos.y
                && self.pos.y < (paddle.pos.y + paddle.get_height())
                && self.pos.x - self.r < paddle.pos.x + paddle.get_width()
            {
                self.vel.x *= -1.;
            }
            if !paddle.is_player()
                && self.pos.y > paddle.pos.y
                && self.pos.y < (paddle.pos.y + paddle.get_height())
                && self.pos.x + self.r > paddle.pos.x
            {
                self.vel.x *= -1.;
            }
        }
    }

    pub fn draw(&self) {
        draw_circle(self.pos.x, self.pos.y, self.r, WHITE);
    }
}
