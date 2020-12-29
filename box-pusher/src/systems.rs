use duku::window::Events;
use duku::window::Key;
use duku::Filter;
use duku::ShapeMode;
use duku::Target;
use duku::Vec2;
use gilrs::Button;
use gilrs::Gamepad;
use specs::world::EntitiesRes;
use specs::Component;
use specs::Entity;
use specs::Join;
use specs::World;
use specs::WorldExt;

use crate::components::Animations;
use crate::components::Movable;
use crate::components::Player;
use crate::components::Position;
use crate::components::Pushable;
use crate::components::Solid;
use crate::components::Sprite;

pub fn draw_system(world: &World, t: &mut Target, view_w: u32, view_h: u32) {
    let positions = world.read_storage::<Position>();
    let sprites = world.read_storage::<Sprite>();

    t.filter(Filter::Nearest);
    t.shape_mode(ShapeMode::TopLeft);
    t.translate_y(view_h as f32 / -2.0);
    t.translate_x(view_w as f32 / -2.0);

    // render sprites with positions
    for (pos, sprite) in (&positions, &sprites).join() {
        t.push();
        t.translate_z(pos.layer);
        t.texture_part(
            &sprite.texture,
            pos.pos,
            sprite.part_size,
            sprite.part_pos,
            sprite.part_size,
        );
        t.pop();
    }
}

pub fn animate_system(world: &World, dt: f32) {
    let mut sprites = world.write_storage::<Sprite>();
    let mut animations = world.write_storage::<Animations>();

    for (spr, anis) in (&mut sprites, &mut animations).join() {
        // get current animation
        let ani = &anis.animations[anis.current_animation];

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
    let mut positions = world.write_storage::<Position>();
    let movables = world.read_storage::<Movable>();

    for (pos, mov) in (&mut positions, &movables).join() {
        // check if player is moving
        let is_moving = pos.pos != mov.target;

        if is_moving {
            // move player to target position
            let dir = (mov.target - pos.pos).unit();
            pos.pos += dir * 1.0;
        }
    }
}

pub fn collision_system(world: &World) {
    let positions = world.read_storage::<Position>();
    let solids = world.read_storage::<Solid>();
    let mut movables = world.write_storage::<Movable>();

    for (pos, mov) in (&positions, &mut movables).join() {
        // check solids
        for (solid, _) in (&positions, &solids).join() {
            if solid.pos == mov.target {
                mov.target = pos.pos;
            }
        }
    }
}

pub fn player_move_system(
    world: &World,
    events: &Events,
    gamepad: &Option<Gamepad>,
    tile_size: u32,
) {
    let players = world.read_storage::<Player>();
    let positions = world.read_storage::<Position>();
    let mut movables = world.write_storage::<Movable>();
    let mut animations = world.write_storage::<Animations>();

    let mut pushable = None;
    let mut push_dir = Vec2::new(0.0, 0.0);

    for (pos, mov, ani, _) in (&positions, &mut movables, &mut animations, &players).join() {
        // check if player is moving
        let is_moving = pos.pos != mov.target;

        if !is_moving {
            // move target if player pressed key
            // and change animation
            if events.is_key_pressed(Key::W) || gamepad_pressed(gamepad, Button::DPadUp) {
                mov.target.y += tile_size as f32;
                ani.current_animation = 1;
            } else if events.is_key_pressed(Key::S) || gamepad_pressed(gamepad, Button::DPadDown) {
                mov.target.y -= tile_size as f32;
                ani.current_animation = 0;
            } else if events.is_key_pressed(Key::A) || gamepad_pressed(gamepad, Button::DPadLeft) {
                mov.target.x -= tile_size as f32;
                ani.current_animation = 2;
            } else if events.is_key_pressed(Key::D) || gamepad_pressed(gamepad, Button::DPadRight) {
                mov.target.x += tile_size as f32;
                ani.current_animation = 3;
            }

            // get move direction and check for pushables
            push_dir = mov.target - pos.pos;
            pushable = check_position::<Pushable>(world, mov.target);

            // check the next tile to cancel movement
            if pushable.is_some()
                && (check_position::<Solid>(world, mov.target + push_dir).is_some()
                    || check_position::<Pushable>(world, mov.target + push_dir).is_some())
            {
                pushable = None;
                mov.target = pos.pos;
            }
        }
    }

    // if pushable ahead, move it
    if let Some(push) = pushable {
        let mov = movables.get_mut(push).expect("bad entity");
        mov.target += push_dir;
    }
}

fn check_position<T: Component>(world: &World, pos: Vec2) -> Option<Entity> {
    let positions = world.read_storage::<Position>();
    let components = world.read_storage::<T>();
    let entities = world.fetch::<EntitiesRes>();

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
