use duku::Vec2;
use specs::Entities;
use specs::Entity;
use specs::Join;
use specs::Read;
use specs::ReadStorage;
use specs::System;
use specs::WriteStorage;
use std::collections::HashMap;

use crate::components::Direction;
use crate::components::Immovable;
use crate::components::Movable;
use crate::components::Player;
use crate::components::Position;
use crate::resources::Button;
use crate::resources::Inputs;

pub struct MoveSystem {}

impl<'s> System<'s> for MoveSystem {
    #[allow(clippy::type_complexity)]
    type SystemData = (
        WriteStorage<'s, Position>,
        ReadStorage<'s, Player>,
        ReadStorage<'s, Immovable>,
        ReadStorage<'s, Movable>,
        Read<'s, Inputs>,
        Entities<'s>,
    );

    fn run(&mut self, data: Self::SystemData) {
        let (mut positions, players, immovables, movables, inputs, entities) = data;

        // build a tile reference map
        let immov: HashMap<_, _> = (&positions, &immovables, &entities)
            .join()
            .map(|(pos, _, i)| ((pos.x, pos.y), i))
            .collect();

        let mov: HashMap<_, _> = (&positions, &movables, &entities)
            .join()
            .map(|(pos, _, i)| ((pos.x, pos.y), i))
            .collect();

        // do player position changes
        let mut moving_entities = vec![];
        for (pos, _, player) in (&positions, &players, &entities).join() {
            if pos.offset.length() < 0.1 {
                let no_x = pos.offset.x.abs() == 0.0;
                let no_y = pos.offset.y.abs() == 0.0;

                if inputs.keys_pressed.contains(&Button::Up) && no_x {
                    if can_move(&immov, &mov, Direction::Up, pos.x, pos.y) {
                        moving_entities.push((player, Direction::Up));

                        // check if should push a thing
                        if let Some(entity) = mov.get(&(pos.x, pos.y + 1)) {
                            moving_entities.push((*entity, Direction::Up));
                        }
                    } else {
                        // bump sound
                    }
                }
                if inputs.keys_pressed.contains(&Button::Down) && no_x {
                    if can_move(&immov, &mov, Direction::Down, pos.x, pos.y) {
                        moving_entities.push((player, Direction::Down));

                        // check if should push a thing
                        if let Some(entity) = mov.get(&(pos.x, pos.y - 1)) {
                            moving_entities.push((*entity, Direction::Down));
                        }
                    } else {
                        // bump sound
                    }
                }
                if inputs.keys_pressed.contains(&Button::Left) && no_y {
                    if can_move(&immov, &mov, Direction::Left, pos.x, pos.y) {
                        moving_entities.push((player, Direction::Left));

                        // check if should push a thing
                        if let Some(entity) = mov.get(&(pos.x - 1, pos.y)) {
                            moving_entities.push((*entity, Direction::Left));
                        }
                    } else {
                        // bump sound
                    }
                }
                if inputs.keys_pressed.contains(&Button::Right) && no_y {
                    if can_move(&immov, &mov, Direction::Right, pos.x, pos.y) {
                        moving_entities.push((player, Direction::Right));

                        // check if should push a thing
                        if let Some(entity) = mov.get(&(pos.x + 1, pos.y)) {
                            moving_entities.push((*entity, Direction::Right));
                        }
                    } else {
                        // bump sound
                    }
                }
            }
        }

        // move all entities that should be moved
        for (entity, direction) in moving_entities {
            let mut pos = positions.get_mut(entity).expect("bad entity");
            pos.direction = direction;
            match direction {
                Direction::Up => {
                    pos.y += 1;
                    pos.offset += Vec2::down();
                }
                Direction::Down => {
                    pos.y -= 1;
                    pos.offset += Vec2::up();
                }
                Direction::Left => {
                    pos.x -= 1;
                    pos.offset += Vec2::right();
                }
                Direction::Right => {
                    pos.x += 1;
                    pos.offset += Vec2::left();
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

fn can_move(
    immov: &HashMap<(i32, i32), Entity>,
    mov: &HashMap<(i32, i32), Entity>,
    direction: Direction,
    x: i32,
    y: i32,
) -> bool {
    let (xo, yo) = match direction {
        Direction::Up => (0, 1),
        Direction::Down => (0, -1),
        Direction::Right => (1, 0),
        Direction::Left => (-1, 0),
    };

    !(immov.contains_key(&(x + xo, y + yo))
        || (mov.contains_key(&(x + xo, y + yo))
            && (immov.contains_key(&(x + xo * 2, y + yo * 2))
                || mov.contains_key(&(x + xo * 2, y + yo * 2)))))
}
