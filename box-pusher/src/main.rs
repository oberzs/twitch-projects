mod macros;

mod components;
mod error;
mod level;
mod resources;
mod systems;
mod world;

use duku::Camera;
use duku::Duku;
use gilrs::Gilrs;

use error::Result;
use systems::AnimateSystem;
use systems::DrawSystem;
use systems::InputSystem;
use systems::MoveSystem;
use world::World;

fn main() -> Result<()> {
    let tile_size = 16;
    let view_width = tile_size * 10;
    let view_height = tile_size * 9;
    let window_width = view_width * 4;
    let window_height = view_height * 4;

    let (mut duku, window) = Duku::windowed(window_width, window_height)?;
    let mut gilrs = Gilrs::new().expect("bad gilrs");

    let camera = Camera::orthographic_sized(view_width as f32, view_height as f32);

    let mut world = World::new()?;

    // load sprites
    world.add_sprite(&mut duku, "assets/player.png")?;
    world.add_sprite(&mut duku, "assets/floor.png")?;
    world.add_sprite(&mut duku, "assets/wall.png")?;
    world.add_sprite(&mut duku, "assets/box.png")?;

    // load sounds
    world.add_sound("assets/slurp.mp3");

    // load level
    level::load(&mut world, "assets/world.ldtk", "Test")?;

    window.while_open(move |events| {
        world.run_system(InputSystem {
            gilrs: &mut gilrs,
            events,
        });

        world.run_system(MoveSystem {});

        world.run_system(AnimateSystem {
            delta_time: duku.delta_time(),
        });

        duku.draw(Some(&camera), |t| {
            world.run_system(DrawSystem {
                target: t,
                view_width,
                view_height,
                tile_size,
            });
        });
    });

    Ok(())
}
