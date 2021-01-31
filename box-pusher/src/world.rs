use duku::Duku;
use duku::Handle;
use duku::Texture;
use duku::Vec2;
use kira::manager::AudioManager;
use kira::sound::SoundId;
use specs::Builder;
use specs::RunNow;
use specs::System;
use specs::World as SpecsWorld;
use specs::WorldExt;
use std::collections::HashMap;
use std::path::Path;
use std::time::Instant;

use super::Result;
use crate::components::Animation;
use crate::components::Animations;
use crate::components::Direction;
use crate::components::Immovable;
use crate::components::Movable;
use crate::components::Player;
use crate::components::Position;
use crate::components::Sprite;
use crate::resources::Inputs;

pub struct World {
    specs: SpecsWorld,
    audio: AudioManager,
    sprites: HashMap<String, Handle<Texture>>,
    sounds: HashMap<String, SoundId>,
    sound_start: Instant,
    sound_cooldown: f64,
}

impl World {
    pub fn new() -> Result<Self> {
        let mut specs = SpecsWorld::new();

        // register components
        specs.register::<Sprite>();
        specs.register::<Position>();
        specs.register::<Player>();
        specs.register::<Movable>();
        specs.register::<Animations>();
        specs.register::<Movable>();
        specs.register::<Immovable>();

        // insert resources
        specs.insert(Inputs::default());

        let audio = AudioManager::new(Default::default()).expect("bad kira");

        let sprites = HashMap::new();
        let sounds = HashMap::new();

        Ok(Self {
            sound_start: Instant::now(),
            sound_cooldown: 0.0,
            specs,
            audio,
            sprites,
            sounds,
        })
    }

    pub fn run_system<'a>(&'a self, mut system: impl System<'a>) {
        system.run_now(&self.specs);
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

    pub fn add_sound(&mut self, path: impl AsRef<Path>) {
        let p = path.as_ref();
        let name = p
            .file_name()
            .map(|n| n.to_str().unwrap_or(""))
            .unwrap_or("");
        let sound = self
            .audio
            .load_sound(p, Default::default())
            .expect("bad sound");
        self.sounds.insert(name.to_string(), sound);
    }

    pub fn play_sound(&mut self, name: &str) {
        if self.sound_start.elapsed().as_secs_f64() > self.sound_cooldown {
            let sound = self
                .sounds
                .get(name)
                .expect("bad oliver, you did bad sound");

            // set cooldown
            self.sound_start = Instant::now();
            self.sound_cooldown = sound.duration();

            self.audio
                .play(*sound, Default::default())
                .expect("cannot play");
        }
    }

    pub fn spawn_wall(&mut self, sprite: &str, x: i32, y: i32, part_pos: Vec2, part_size: Vec2) {
        let texture = self.get_sprite(sprite);

        self.specs
            .create_entity()
            .with(Position {
                x,
                y,
                z: 2,
                offset: Vec2::default(),
                direction: Direction::Right,
            })
            .with(Sprite {
                texture,
                part_pos,
                part_size,
            })
            .with(Immovable)
            .build();
    }

    pub fn spawn_floor(&mut self, sprite: &str, x: i32, y: i32, part_pos: Vec2, part_size: Vec2) {
        let texture = self.get_sprite(sprite);

        self.specs
            .create_entity()
            .with(Position {
                x,
                y,
                z: 3,
                offset: Vec2::default(),
                direction: Direction::Right,
            })
            .with(Sprite {
                texture,
                part_pos,
                part_size,
            })
            .build();
    }

    pub fn spawn_box(&mut self, x: i32, y: i32) {
        let texture = self.get_sprite("box.png");

        self.specs
            .create_entity()
            .with(Position {
                x,
                y,
                z: 1,
                offset: Vec2::default(),
                direction: Direction::Right,
            })
            .with(Sprite {
                texture,
                part_pos: Vec2::new(0.0, 0.0),
                part_size: Vec2::new(16.0, 16.0),
            })
            .with(Movable)
            .build();
    }

    pub fn spawn_player(&mut self, x: i32, y: i32) {
        let texture = self.get_sprite("player.png");

        self.specs
            .create_entity()
            .with(Position {
                x,
                y,
                z: 1,
                offset: Vec2::default(),
                direction: Direction::Down,
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
                        duration: 150,
                        frames: vec![8, 9, 10, 11],
                    },
                    "walk-up" => Animation {
                        duration: 150,
                        frames: vec![12, 13, 14, 15]
                    },
                    "walk-right" => Animation {
                        duration: 150,
                        frames: vec![16, 17, 18, 19]
                    },
                    "walk-left" => Animation {
                        duration: 150,
                        frames: vec![20, 21, 22, 23]
                    },
                    "special" => Animation {
                        duration: 500,
                        frames: vec![24, 25, 26, 27, 28, 29]
                    }
                ),
            })
            .with(Player)
            .build();
    }

    fn get_sprite(&self, name: &str) -> Handle<Texture> {
        self.sprites.get(name).expect("bad sprite").clone()
    }
}
