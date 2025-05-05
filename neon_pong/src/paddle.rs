use macroquad::prelude::*;

const MOVESPD: f32 = 500.;

pub struct Paddle {
    pub pos: Vec2,
    height: f32,
    width: f32,
    player: bool,
}

impl Paddle {
    pub fn new(pos: Vec2) -> Paddle {
        let player = pos.x < screen_width() / 2.;
        let height: f32 = 80.;
        let width: f32 = 10.;
        Paddle {
            pos,
            height,
            width,
            player,
        }
    }

    pub fn get_height(&self) -> f32 {
        self.height
    }

    pub fn get_width(&self) -> f32 {
        self.width
    }

    pub fn is_player(&self) -> bool {
        self.player
    }

    pub fn update(&mut self) {
        let delta_time = get_frame_time();
        let move_speed = MOVESPD * delta_time;

        if self.player & is_key_down(KeyCode::W) {
            self.pos -= Vec2::new(0., move_speed);
        }
        if self.player & is_key_down(KeyCode::S) {
            self.pos += Vec2::new(0., move_speed);
        }

        //TODO: AI movement

        self.edges();
    }

    fn edges(&mut self) {
        if self.pos.y > screen_height() - self.height {
            self.pos.y = screen_height() - self.height;
        }
        if self.pos.y < 0. {
            self.pos.y = 0.;
        }
    }

    pub fn show(&self) {
        draw_rectangle(self.pos.x, self.pos.y, self.width, self.height, WHITE);
    }
}
