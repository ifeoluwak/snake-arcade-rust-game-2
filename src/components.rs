use std::default;

use bevy::prelude::{Component, Entity, Timer};

use crate::constants::GAME_AREA_STEP;



#[derive(Component, Debug)]
pub struct Board;

#[derive(Component, Debug)]
pub struct Snake;

#[derive(Component, Debug)]
pub struct SnakeHead;

#[derive(Component, Debug)]
pub struct Block;

pub struct LastEntity {
    pub block: Entity,
}

pub struct BlockCount(pub usize);

#[derive(Debug)]
pub enum SnakeDirection {
    Left,
    Right,
    Up,
    Down
}

pub struct MotionTimer {
    pub timer: Timer
}

#[derive(Component, Debug)]
pub struct PreviousEntity(pub Entity);

pub struct GameStep(pub f32);

impl Default for GameStep {
    fn default() -> Self {
        GameStep(GAME_AREA_STEP)
    }
}