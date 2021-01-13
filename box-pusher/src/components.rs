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
    pub pos: Vec2,
}

#[derive(Component)]
pub struct Movable {
    pub target: Vec2,
    pub direction: Vec2,
    pub speed: f32,
}

#[derive(Component)]
pub struct Player;

#[derive(Component)]
pub struct Solid;

#[derive(Component)]
pub struct Pushable;

#[derive(Component)]
pub struct FloorLayer;

#[derive(Component)]
pub struct TileLayer;

#[derive(Component)]
pub struct EntityLayer;
