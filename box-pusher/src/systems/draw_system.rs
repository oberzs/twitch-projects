use duku::Filter;
use duku::ShapeMode;
use duku::Target;
use duku::Vec2;
use specs::Join;
use specs::ReadStorage;
use specs::System;

use crate::components::Position;
use crate::components::Sprite;

pub struct DrawSystem<'t> {
    pub target: &'t mut Target,
    pub view_width: u32,
    pub view_height: u32,
    pub tile_size: u32,
}

impl<'t> System<'t> for DrawSystem<'t> {
    type SystemData = (ReadStorage<'t, Position>, ReadStorage<'t, Sprite>);

    fn run(&mut self, data: Self::SystemData) {
        let (positions, sprites) = data;

        self.target.filter(Filter::Nearest);
        self.target.shape_mode(ShapeMode::TopLeft);
        self.target.translate_y(self.view_height as f32 / -2.0);
        self.target.translate_x(self.view_width as f32 / -2.0);

        let mut draw_data = (&positions, &sprites).join().collect::<Vec<_>>();
        draw_data.sort_by_key(|k| -(k.0.z as i32));

        for (pos, spr) in draw_data {
            let x = (pos.x as f32 + pos.offset.x) * self.tile_size as f32;
            let y = (pos.y as f32 + pos.offset.y) * self.tile_size as f32;

            self.target.push();
            self.target.translate_z(pos.z as f32);
            self.target.texture_part(
                &spr.texture,
                Vec2::new(x, y),
                spr.part_size,
                spr.part_pos,
                spr.part_size,
            );
            self.target.pop();
        }
    }
}
