use macroquad::prelude::*;

pub struct Ball {
    pub pos: Vec2,
    pub vel: Vec2,
    w: f32,
    half_w: f32
}

impl Ball {

    pub fn new(pos: Vec2, vel: Vec2) -> Ball {
        Ball {
            pos: pos,
            vel: vel,
            w: 4.,
            half_w: 4. / 2.,
        }
    }

    pub fn update_pos(&mut self) {
        self.pos += self.vel;
        if self.pos.y >= screen_height() || self.pos.y <= 0. {
            self.vel.y *= -1.;
        }
        // Remove this later
        if self.pos.x >= screen_width() || self.pos.x <= 0. {
            self.vel.x *= -1.
        }
    }

    pub fn draw(&self) {
        draw_circle(self.pos.x, self.pos.y, self.w, WHITE);
    }
}

