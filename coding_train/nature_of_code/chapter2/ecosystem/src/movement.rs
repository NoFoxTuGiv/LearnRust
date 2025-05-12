use macroquad::prelude::*;

pub trait Vec2Behaviors {
    fn seek_food(self, foods: &[Vec2]) -> Vec2;
    fn avoid_predators(self, predators: &[Vec2]) -> Vec2;
    fn apply_speed(self, max_speed: f32) -> Vec2;
}

impl Vec2Behaviors for Vec2 {

    /// Takes in a positional vector and adds a weighted vector towards the nearest piece of food.
    fn seek_food(self,foods: &[Vec2]) -> Vec2 {
        // Select closest food and create weighted vector towards that food.
        // Add this vector to the incoming positional vector.
        for food in foods {
            let shortest_distance: usize = usize::MAX;
            let distance = self.pos - food.pos;
            if distance 
        }
        todo!();
    }

    /// Takes in a positional vector and adds a weighted vector away from nearby predators.
    fn avoid_predators(self, predators: &[Vec2]) -> Vec2 {
        // Create a weighted vector that avoids nearby predators
        // Add this vector to the incoming positional vector.
        todo!();
    }

    /// Takes a vector, usually the sum of other behaviors, and normalizes then scales this vector
    /// according to max_speed.
    fn apply_speed(self, max_speed: f32) -> Vec2 {
        if self.length() > 0. {
            self.normalize() * max_speed
        } else {
            Vec2::ZERO
        }
    }
}

