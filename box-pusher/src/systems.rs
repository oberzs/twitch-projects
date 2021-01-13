use duku::window::Events;
use duku::window::Key;
use duku::Filter;
use duku::ShapeMode;
use duku::Target;
use duku::Vec2;
use gilrs::Button;
use gilrs::Gamepad;
use specs::Component;
use specs::Entity;
use specs::Join;

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
use crate::world::World;

pub fn draw_system(world: &World, t: &mut Target, view_w: u32, view_h: u32) {
    let positions = world.read::<Position>();
    let sprites = world.read::<Sprite>();
    let floors = world.read::<FloorLayer>();
    let tiles = world.read::<TileLayer>();
    let entities = world.read::<EntityLayer>();

    t.filter(Filter::Nearest);
    t.shape_mode(ShapeMode::TopLeft);
    t.translate_y(view_h as f32 / -2.0);
    t.translate_x(view_w as f32 / -2.0);

    // render floors
    for (pos, sprite, _) in (&positions, &sprites, &floors).join() {
        t.push();
        t.translate_z(2.0);
        t.texture_part(
            &sprite.texture,
            pos.pos,
            sprite.part_size,
            sprite.part_pos,
            sprite.part_size,
        );
        t.pop();
    }

    // render tiles
    for (pos, sprite, _) in (&positions, &sprites, &tiles).join() {
        t.push();
        t.translate_z(1.0);
        t.texture_part(
            &sprite.texture,
            pos.pos,
            sprite.part_size,
            sprite.part_pos,
            sprite.part_size,
        );
        t.pop();
    }

    // render entities
    for (pos, sprite, _) in (&positions, &sprites, &entities).join() {
        t.texture_part(
            &sprite.texture,
            pos.pos,
            sprite.part_size,
            sprite.part_pos,
            sprite.part_size,
        );
    }
}

pub fn animate_system(world: &World, dt: f32) {
    let mut sprites = world.write::<Sprite>();
    let mut animations = world.write::<Animations>();

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
            anis.time += dt;
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

pub fn movable_system(world: &World) {
    let mut positions = world.write::<Position>();
    let movables = world.read::<Movable>();

    for (pos, mov) in (&mut positions, &movables).join() {
        // check if movable is moving
        if pos.pos != mov.target {
            // move player to target position
            let dir = (mov.target - pos.pos).unit();
            pos.pos += dir * mov.speed;
        }
    }
}

pub fn collision_system(world: &mut World) {
    let mut bump = false;

    {
        let solids = world.read::<Solid>();
        let positions = world.read::<Position>();
        let mut movables = world.write::<Movable>();

        for (pos, mov) in (&positions, &mut movables).join() {
            // check solids
            for (solid, _) in (&positions, &solids).join() {
                if solid.pos == mov.target {
                    mov.target = pos.pos;
                    bump = true;
                }
            }
        }
    }

    if bump {
        world.play_sound("slurp.mp3");
    }
}

pub fn player_animate_system(world: &World) {
    let players = world.read::<Player>();
    let positions = world.read::<Position>();
    let movables = world.read::<Movable>();
    let mut animations = world.write::<Animations>();

    for (pos, mov, ani, _) in (&positions, &movables, &mut animations, &players).join() {
        let prev_animation = ani.current_animation.clone();

        if mov.target != pos.pos {
            ani.current_animation = match mov.direction {
                Vec2 { x, .. } if x as i32 == 1 => "walk-right".to_string(),
                Vec2 { x, .. } if x as i32 == -1 => "walk-left".to_string(),
                Vec2 { y, .. } if y as i32 == 1 => "walk-up".to_string(),
                Vec2 { y, .. } if y as i32 == -1 => "walk-down".to_string(),
                _ => unreachable!(),
            };
        } else {
            ani.current_animation = match mov.direction {
                Vec2 { x, .. } if x as i32 == 1 => "idle-right".to_string(),
                Vec2 { x, .. } if x as i32 == -1 => "idle-left".to_string(),
                Vec2 { y, .. } if y as i32 == 1 => "idle-up".to_string(),
                Vec2 { y, .. } if y as i32 == -1 => "idle-down".to_string(),
                _ => unreachable!(),
            };
        }

        if prev_animation != ani.current_animation {
            ani.time = 0.0;
        }
    }
}

pub fn player_move_system(
    world: &mut World,
    events: &Events,
    gamepad: &Option<Gamepad>,
    tile_size: u32,
) {
    let mut bump = false;

    {
        let players = world.read::<Player>();
        let positions = world.read::<Position>();
        let mut movables = world.write::<Movable>();
        let mut pushable = None;
        let mut push_dir = Vec2::new(0.0, 0.0);
        for (pos, mov, _) in (&positions, &mut movables, &players).join() {
            if pos.pos == mov.target {
                // reset player movement speed
                mov.speed = 1.0;
                // move target if player pressed key
                // and change animation
                if events.is_key_pressed(Key::W) || gamepad_pressed(gamepad, Button::DPadUp) {
                    mov.target.y += tile_size as f32;
                    mov.direction = Vec2::up();
                } else if events.is_key_pressed(Key::S)
                    || gamepad_pressed(gamepad, Button::DPadDown)
                {
                    mov.target.y -= tile_size as f32;
                    mov.direction = Vec2::down();
                } else if events.is_key_pressed(Key::A)
                    || gamepad_pressed(gamepad, Button::DPadLeft)
                {
                    mov.target.x -= tile_size as f32;
                    mov.direction = Vec2::left();
                } else if events.is_key_pressed(Key::D)
                    || gamepad_pressed(gamepad, Button::DPadRight)
                {
                    mov.target.x += tile_size as f32;
                    mov.direction = Vec2::right();
                }
                // get move direction and check for pushables
                push_dir = mov.target - pos.pos;
                pushable = check_position::<Pushable>(world, mov.target);
                // change state to push if pushable ahead
                if pushable.is_some() {
                    mov.speed = 0.5;
                }
                // check the next tile to cancel movement
                if pushable.is_some()
                    && (check_position::<Solid>(world, mov.target + push_dir).is_some()
                        || check_position::<Pushable>(world, mov.target + push_dir).is_some())
                {
                    pushable = None;
                    mov.target = pos.pos;
                    bump = true;
                }
            }
        }
        // if pushable ahead, move it
        if let Some(push) = pushable {
            let mov = movables.get_mut(push).expect("bad entity");
            mov.target += push_dir;
        }
    }

    if bump {
        world.play_sound("slurp.mp3");
    }
}

fn check_position<T: Component>(world: &World, pos: Vec2) -> Option<Entity> {
    let positions = world.read::<Position>();
    let components = world.read::<T>();
    let entities = world.entities();

    // loop through objects with component
    for (e, comp, _) in (&entities, &positions, &components).join() {
        if comp.pos == pos {
            return Some(e);
        }
    }
    None
}

fn gamepad_pressed(gamepad: &Option<Gamepad>, button: Button) -> bool {
    gamepad.map(|g| g.is_pressed(button)).unwrap_or(false)
}
