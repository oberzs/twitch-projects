use duku::Handle;
use duku::Texture;
use duku::Vec2;
use specs::Component;
use specs::DenseVecStorage;
use specs_derive::Component;
use std::collections::HashMap;

#[derive(Component)]
pub struct Sprite {
    pub texture: Handle<Texture>,
    pub part_pos: Vec2,
    pub part_size: Vec2,
}

#[derive(Component)]
pub struct Animations {
    pub size: Vec2,
    pub time: f32,
    pub animations: HashMap<String, Animation>,
    pub current_animation: String,
}

pub struct Animation {
    pub duration: u32,
    pub frames: Vec<usize>,
}

#[derive(Component)]
pub struct Position {
    pub x: u32,
    pub y: u32,
    pub z: u32,
    pub offset: Vec2,
    pub direction: Vec2,
}

#[derive(Component)]
pub struct Player;

#[derive(Component)]
pub struct Movable;

#[derive(Component)]
pub struct Immovable;
