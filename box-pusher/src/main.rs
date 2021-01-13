mod macros;

mod components;
mod error;
mod level;
mod systems;
mod world;

use duku::Camera;
use duku::Duku;
use gilrs::Event;
use gilrs::Gilrs;

use error::Result;
use systems::animate_system;
use systems::collision_system;
use systems::draw_system;
use systems::movable_system;
use systems::player_animate_system;
use systems::player_move_system;
use world::World;

fn main() -> Result<()> {
    let tile_size = 16;
    let view_w = tile_size * 10;
    let view_h = tile_size * 9;
    let window_w = view_w * 4;
    let window_h = view_h * 4;

    let (mut duku, window) = Duku::windowed(window_w, window_h)?;
    let mut gilrs = Gilrs::new().expect("bad gilrs");

    let mut gamepad_id = None;

    let camera = Camera::orthographic_sized(view_w as f32, view_h as f32);

    let mut world = World::new()?;

    // load sprites
    world.add_sprite(&mut duku, "assets/player.png")?;
    world.add_sprite(&mut duku, "assets/floor.png")?;
    world.add_sprite(&mut duku, "assets/wall.png")?;
    world.add_sprite(&mut duku, "assets/box.png")?;

    // load level
    level::load(&mut world, "assets/world.ldtk", "Test")?;

    window.while_open(move |events| {
        // check for new gamepad events
        while let Some(Event { id, .. }) = gilrs.next_event() {
            gamepad_id = Some(id);
        }
        let gamepad = gamepad_id.map(|g| gilrs.gamepad(g));

        movable_system(&world);
        player_move_system(&world, events, &gamepad, tile_size);
        collision_system(&world);
        player_animate_system(&world);
        animate_system(&world, duku.delta_time());

        duku.draw(Some(&camera), |t| {
            draw_system(&world, t, view_w, view_h);
        });
    });

    Ok(())
}
