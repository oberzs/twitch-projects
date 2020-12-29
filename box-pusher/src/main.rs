mod components;
mod error;
mod level;
mod sprites;
mod systems;

use duku::Camera;
use duku::Duku;
use gilrs::Event;
use gilrs::Gilrs;
use specs::World;
use specs::WorldExt;

use components::Animations;
use components::Movable;
use components::Player;
use components::Position;
use components::Pushable;
use components::Solid;
use components::Sprite;
use error::Result;
use sprites::Sprites;
use systems::animate_system;
use systems::collision_system;
use systems::draw_system;
use systems::movable_system;
use systems::player_move_system;

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

    // load sprites
    let mut sprites = Sprites::default();
    sprites.add(&mut duku, "assets/player.png")?;
    sprites.add(&mut duku, "assets/floor.png")?;
    sprites.add(&mut duku, "assets/wall.png")?;
    sprites.add(&mut duku, "assets/box.png")?;

    // setup ECS
    let mut world = World::new();
    world.register::<Sprite>();
    world.register::<Position>();
    world.register::<Player>();
    world.register::<Solid>();
    world.register::<Movable>();
    world.register::<Animations>();
    world.register::<Pushable>();

    // load level
    level::load(&mut world, &sprites, "assets/world.ldtk", "Test")?;

    window.while_open(move |events| {
        // check for new gamepad events
        while let Some(Event { id, .. }) = gilrs.next_event() {
            gamepad_id = Some(id);
        }
        let gamepad = gamepad_id.map(|g| gilrs.gamepad(g));

        collision_system(&world);
        movable_system(&world);
        // player_push_system(&world, tile_size);
        player_move_system(&world, events, &gamepad, tile_size);
        animate_system(&world, duku.delta_time());

        duku.draw(Some(&camera), |t| {
            draw_system(&world, t, view_w, view_h);
        });
    });

    Ok(())
}
