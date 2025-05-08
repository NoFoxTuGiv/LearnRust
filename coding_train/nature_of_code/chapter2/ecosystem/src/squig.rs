#![allow(unused)]

use macroquad::prelude::*;
use crate::movement::Vec2Behaviors;

pub enum SquigDiet {
    Carnivore,
    Herbivore,
    Omnivore,
}

pub struct BaseSquig {
    diet: SquigDiet,
    pos: Vec2,
}

impl BaseSquig {
    pub fn new(diet: SquigDiet, pos: Vec2) -> BaseSquig {
        BaseSquig { diet, pos }
    }
}

pub trait SquigBehavior {
    fn update(&mut self);
    fn draw(&self);
}

pub struct RunnerSquig {
    base: BaseSquig,
    max_speed: f32,
}

impl RunnerSquig {
    fn find_food() -> Vec<Vec2> {
        todo!();
    }
    fn take_step(&self) -> Vec2 {
        // Placeholder food vector, will create via function later
        let food = vec![Vec2::new(50., 50.), Vec2::new(25., 25.)];
        // Placeholder predators vector, will create via function later
        let predators = vec![Vec2::new(100., 100.), Vec2::new(150., 75.)];
        Vec2::ZERO
            .seek_food(&food)
            .avoid_predators(&predators)
            .apply_speed(self.max_speed)
    }
}

impl SquigBehavior for RunnerSquig {
    fn update(&mut self) {
        self.take_step();
    }
    
    fn draw(&self) {
        todo!();
    }
}
