use duku::Vec2;
use specs::Join;
use specs::ReadStorage;
use specs::System;
use specs::WriteStorage;

use crate::components::Animations;
use crate::components::Direction;
use crate::components::Player;
use crate::components::Position;
use crate::components::Sprite;

pub struct AnimateSystem {
    pub delta_time: f32,
}

impl<'s> System<'s> for AnimateSystem {
    type SystemData = (
        WriteStorage<'s, Sprite>,
        WriteStorage<'s, Animations>,
        ReadStorage<'s, Position>,
        ReadStorage<'s, Player>,
    );

    fn run(&mut self, data: Self::SystemData) {
        let (mut sprites, mut animations, positions, players) = data;

        // do player animation changes
        for (pos, ani, _) in (&positions, &mut animations, &players).join() {
            let prev_animation = ani.current_animation.clone();
            if pos.offset != Vec2::default() {
                ani.current_animation = match pos.direction {
                    Direction::Right => "walk-right".to_string(),
                    Direction::Left => "walk-left".to_string(),
                    Direction::Up => "walk-up".to_string(),
                    Direction::Down => "walk-down".to_string(),
                };
            } else {
                ani.current_animation = match pos.direction {
                    Direction::Right => "idle-right".to_string(),
                    Direction::Left => "idle-left".to_string(),
                    Direction::Up => "idle-up".to_string(),
                    Direction::Down => "idle-down".to_string(),
                };
            }
            if prev_animation != ani.current_animation {
                ani.time = 0.0;
            }
        }

        // calculate animated entities
        for (spr, anis) in (&mut sprites, &mut animations).join() {
            // get current animation
            let ani = &anis
                .animations
                .get(&anis.current_animation)
                .expect("bad animation");

            // validate animation duration
            if ani.duration != 0 {
                // calculate frame index
                let duration = ani.duration as f32 / 1000.0;
                anis.time += self.delta_time;

                let frame_index = ani.frames[((anis.time / duration) as usize) % ani.frames.len()];

                // calculate texture part
                let x = frame_index as u32 % anis.size.x as u32;
                let y = frame_index as u32 / anis.size.x as u32;
                let w = spr.texture.read().width() / anis.size.x as u32;
                let h = spr.texture.read().height() / anis.size.y as u32;

                spr.part_pos = Vec2::new((x * w) as f32, (y * h) as f32);
                spr.part_size = Vec2::new(w as f32, h as f32);
            }
        }
    }
}
