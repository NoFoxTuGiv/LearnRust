#![allow(dead_code)]

use crate::paddle::Paddle;
use macroquad::prelude::*;

pub struct Ball {
    pub pos: Vec2,
    pub vel: Vec2,
    r: f32,
    speed: f32,
    paddle_col: bool,
    wall_col: bool,
    p_col_time: f64,
    w_col_time: f64,
}

impl Ball {
    pub fn new(pos: Vec2, vel: Vec2) -> Ball {
        let r = 8.;
        let speed = 200.;

        Ball {
            pos,
            vel,
            r,
            speed,
            paddle_col: false,
            wall_col: false,
            p_col_time: 0.,
            w_col_time: 0.,
        }
    }

    pub fn update_pos(&mut self, scores: &mut [usize; 2]) -> [usize; 2] {
        let d_time = get_frame_time();

        self.pos += self.vel * d_time * self.speed;

        if self.wall_col && get_time() > self.w_col_time + 0.2 {
            self.wall_col = false;
        }

        if !self.wall_col && (self.pos.y >= screen_height() || self.pos.y <= 0.) {
            self.vel.y *= -1.;
            self.wall_col = true;
            self.w_col_time = get_time();
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
        if self.paddle_col && get_time() > self.p_col_time + 0.2 {
            self.paddle_col = false;
        }
        if !self.paddle_col && self.is_colliding(paddles) {
            self.vel.x *= -1.;
            self.paddle_col = true;
            self.p_col_time = get_time();
        }
    }

    pub fn draw(&self) {
        draw_circle(self.pos.x, self.pos.y, self.r, WHITE);
    }

    fn is_colliding(&self, paddles: [&Paddle; 2]) -> bool {
        let b_top = self.pos.y - self.r;
        let b_bot = self.pos.y + self.r;
        let b_l = self.pos.x - self.r;
        let b_r = self.pos.x + self.r;

        for paddle in paddles {
            let p_top = paddle.pos.y;
            let p_bot = paddle.pos.y + paddle.get_height();
            let p_l = paddle.pos.x;
            let p_r = paddle.pos.x + paddle.get_width();

            if b_bot >= p_top && b_top <= p_bot && b_r >= p_l && b_l <= p_r {
                return true;
            }
        }

        false
    }
}
