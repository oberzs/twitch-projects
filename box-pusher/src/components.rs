use duku::Handle;
use duku::Texture;
use duku::Vec2;
use specs::Component;
use specs::DenseVecStorage;
use specs_derive::Component;

#[derive(Component)]
pub struct Sprite {
    pub texture: Handle<Texture>,
    pub part_pos: Vec2,
    pub part_size: Vec2,
}

#[derive(Component)]
pub struct Animations {
    pub size: Vec2,
    pub animations: Vec<Animation>,
    pub current_animation: usize,
    pub time: f32,
}

pub struct Animation {
    pub duration: u32,
    pub frames: Vec<usize>,
}

#[derive(Component)]
pub struct Position {
    pub pos: Vec2,
    pub layer: f32,
}

#[derive(Component)]
pub struct Movable {
    pub target: Vec2,
    pub speed: i32,
}

#[derive(Component)]
pub struct Solid;

#[derive(Component)]
pub struct Pushable;

#[derive(Component)]
pub struct Player;
