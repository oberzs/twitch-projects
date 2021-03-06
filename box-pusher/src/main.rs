mod macros;

mod components;
mod error;
mod level;
mod resources;
mod systems;
mod world;

use duku::glsl::Metadata;
use duku::Duku;
use duku::Filter;
use duku::Rgb;
use duku::Wrap;
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
    let window_width = view_width * 5;
    let window_height = view_height * 5;

    let (mut duku, window) = Duku::windowed(window_width, window_height);
    let mut gilrs = Gilrs::new().expect("bad gilrs");

    let shader_path = "assets/crt.glsl";
    let mut crt_shader = duku.create_shader_glsl(shader_path)?;
    let mut meta = Metadata::new(shader_path)?;
    let canvas = duku.create_canvas(view_width, view_height);
    let material = duku.create_material();

    // set up material
    {
        let mut m = material.write();
        m.a.x = canvas.read().shader_index(0).expect("no texture") as f32;
        m.b.x = 6.0;
        m.b.y = 6.0;
        m.c.x = 0.5;
        m.d.x = view_width as f32;
        m.d.y = view_height as f32;
    }

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
        if meta.is_modified() {
            match duku.create_shader_glsl(shader_path) {
                Ok(shader) => crt_shader = shader,
                Err(err) => println!("{}", err),
            }
        }

        world.run_system(InputSystem {
            gilrs: &mut gilrs,
            events,
        });

        world.run_system(MoveSystem {});

        world.run_system(AnimateSystem {
            delta_time: duku.delta_time(),
        });

        duku.begin();

        duku.draw_on_canvas(&canvas, None, |t| {
            t.background(Rgb::clear());
            world.run_system(DrawSystem {
                target: t,
                view_width,
                view_height,
                tile_size,
            });
        });

        duku.draw(None, |t| {
            t.background("#000000");
            t.filter(Filter::Nearest);
            t.wrap(Wrap::ClampBorder);
            t.material(&material);
            t.surface(&crt_shader);
        });

        duku.end();
    });

    Ok(())
}
