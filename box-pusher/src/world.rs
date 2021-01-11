use duku::Duku;
use duku::Handle;
use duku::Texture;
use duku::Vec2;
use specs::world::EntitiesRes;
use specs::Builder;
use specs::Component;
use specs::Read;
use specs::ReadStorage;
use specs::World as SpecsWorld;
use specs::WorldExt;
use specs::WriteStorage;
use std::collections::HashMap;
use std::path::Path;

use super::Result;
use crate::components::Animation;
use crate::components::Animations;
use crate::components::EntityLayer;
use crate::components::FloorLayer;
use crate::components::Movable;
use crate::components::Player;
use crate::components::Position;
use crate::components::Pushable;
use crate::components::Solid;
use crate::components::Sprite;
use crate::components::TileLayer;

pub struct World {
    specs: SpecsWorld,
    sprites: HashMap<String, Handle<Texture>>,
}

impl World {
    pub fn new() -> Result<Self> {
        let mut specs = SpecsWorld::new();
        specs.register::<Sprite>();
        specs.register::<Position>();
        specs.register::<Player>();
        specs.register::<Solid>();
        specs.register::<Movable>();
        specs.register::<Animations>();
        specs.register::<Pushable>();
        specs.register::<EntityLayer>();
        specs.register::<FloorLayer>();
        specs.register::<TileLayer>();

        let sprites = HashMap::new();

        Ok(Self { specs, sprites })
    }

    pub fn add_sprite(&mut self, duku: &mut Duku, path: impl AsRef<Path>) -> Result<()> {
        let p = path.as_ref();
        let name = p
            .file_name()
            .map(|n| n.to_str().unwrap_or(""))
            .unwrap_or("");
        let sprite = duku.create_texture_png(p, None)?;
        self.sprites.insert(name.to_string(), sprite);
        Ok(())
    }

    pub fn read<T: Component>(&self) -> ReadStorage<T> {
        self.specs.read_storage()
    }

    pub fn write<T: Component>(&self) -> WriteStorage<T> {
        self.specs.write_storage()
    }

    pub fn entities(&self) -> Read<EntitiesRes> {
        self.specs.entities()
    }

    pub fn spawn_wall(&mut self, sprite: &str, pos: Vec2, part_pos: Vec2, part_size: Vec2) {
        let texture = self.get_sprite(sprite);

        self.specs
            .create_entity()
            .with(Position { pos })
            .with(Sprite {
                texture,
                part_pos,
                part_size,
            })
            .with(TileLayer)
            .with(Solid)
            .build();
    }

    pub fn spawn_floor(&mut self, sprite: &str, pos: Vec2, part_pos: Vec2, part_size: Vec2) {
        let texture = self.get_sprite(sprite);

        self.specs
            .create_entity()
            .with(Position { pos })
            .with(Sprite {
                texture,
                part_pos,
                part_size,
            })
            .with(FloorLayer)
            .build();
    }

    pub fn spawn_box(&mut self, pos: Vec2) {
        let texture = self.get_sprite("box.png");

        self.specs
            .create_entity()
            .with(Position { pos })
            .with(Movable {
                target: pos,
                speed: 2.0,
            })
            .with(Sprite {
                texture,
                part_pos: Vec2::new(0.0, 0.0),
                part_size: Vec2::new(16.0, 16.0),
            })
            .with(Pushable)
            .with(EntityLayer)
            .build();
    }

    pub fn spawn_player(&mut self, pos: Vec2) {
        let texture = self.get_sprite("player.png");

        self.specs
            .create_entity()
            .with(Position { pos })
            .with(Movable {
                target: pos,
                speed: 2.0,
            })
            .with(Sprite {
                texture,
                part_pos: Vec2::new(0.0, 0.0),
                part_size: Vec2::new(0.0, 0.0),
            })
            .with(Animations {
                size: Vec2::new(6.0, 6.0),
                time: 0.0,
                current_animation: "idle-down".to_string(),
                animations: map! (
                    "idle-down" => Animation {
                        duration: 500,
                        frames: vec![0, 1]
                    },
                    "idle-up" => Animation {
                        duration: 500,
                        frames: vec![2, 3]
                    },
                    "idle-right" => Animation {
                        duration: 500,
                        frames: vec![4, 5]
                    },
                    "idle-left" => Animation {
                        duration: 500,
                        frames: vec![6, 7]
                    },
                    "walk-down" => Animation {
                        duration: 250,
                        frames: vec![8, 9, 10, 11],
                    },
                    "walk-up" => Animation {
                        duration: 250,
                        frames: vec![12, 13, 14, 15]
                    },
                    "walk-right" => Animation {
                        duration: 250,
                        frames: vec![16, 17, 18, 19]
                    },
                    "walk-left" => Animation {
                        duration: 250,
                        frames: vec![20, 21, 22, 23]
                    },
                    "special" => Animation {
                        duration: 500,
                        frames: vec![24, 25, 26, 27, 28, 29]
                    }
                ),
            })
            .with(Player)
            .with(EntityLayer)
            .build();
    }

    fn get_sprite(&self, name: &str) -> Handle<Texture> {
        self.sprites.get(name).expect("bad sprite").clone()
    }
}
