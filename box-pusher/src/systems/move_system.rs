use duku::Vec2;
use specs::Join;
use specs::Read;
use specs::ReadStorage;
use specs::System;
use specs::WriteStorage;

use crate::components::Player;
use crate::components::Position;
use crate::resources::Button;
use crate::resources::Inputs;

pub struct MoveSystem {}

impl<'s> System<'s> for MoveSystem {
    type SystemData = (
        WriteStorage<'s, Position>,
        ReadStorage<'s, Player>,
        Read<'s, Inputs>,
    );

    fn run(&mut self, data: Self::SystemData) {
        let (mut positions, players, inputs) = data;

        // do player position changes
        for (mut pos, _) in (&mut positions, &players).join() {
            if pos.offset.length() < 0.1 {
                if inputs.keys_pressed.contains(&Button::Up) && pos.offset.x.abs() == 0.0 {
                    pos.y += 1;
                    pos.offset += Vec2::down();
                    pos.direction = Vec2::up();
                }
                if inputs.keys_pressed.contains(&Button::Down) && pos.offset.x.abs() == 0.0 {
                    pos.y -= 1;
                    pos.offset += Vec2::up();
                    pos.direction = Vec2::down();
                }
                if inputs.keys_pressed.contains(&Button::Left) && pos.offset.y.abs() == 0.0 {
                    pos.x -= 1;
                    pos.offset += Vec2::right();
                    pos.direction = Vec2::left();
                }
                if inputs.keys_pressed.contains(&Button::Right) && pos.offset.y.abs() == 0.0 {
                    pos.x += 1;
                    pos.offset += Vec2::left();
                    pos.direction = Vec2::right();
                }
            }
        }

        // move objects that have an offset
        for pos in (&mut positions).join() {
            if pos.offset != Vec2::default() {
                let dir = -pos.offset.unit();
                let dist = dir * 0.1;

                if pos.offset.length() >= dist.length() {
                    pos.offset += dist;
                } else {
                    pos.offset = Vec2::default();
                }
            }
        }
    }
}
