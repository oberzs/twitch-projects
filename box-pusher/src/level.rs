use duku::Vec2;
use serde_json::Map;
use serde_json::Value;
use specs::Builder;
use specs::World;
use specs::WorldExt;
use std::fs;
use std::path::Path;

use crate::components::Animation;
use crate::components::Animations;
use crate::components::Movable;
use crate::components::Player;
use crate::components::Position;
use crate::components::Pushable;
use crate::components::Solid;
use crate::components::Sprite;
use crate::error::Result;
use crate::sprites::Sprites;

pub fn load(
    world: &mut World,
    sprites: &Sprites,
    path: impl AsRef<Path>,
    level_name: impl AsRef<str>,
) -> Result<()> {
    let l_name = level_name.as_ref();
    let bytes = fs::read(path)?;
    let json: Value = serde_json::from_slice(&bytes)?;

    // find selected level
    let levels = match &json["levels"] {
        Value::Array(vec) => vec,
        _ => return Err("levels are not present".into()),
    };
    let level = {
        let level_val = levels
            .iter()
            .find(|v| {
                let i = match v {
                    Value::Object(map) => &map["identifier"],
                    _ => &Value::Null,
                };
                matches!(i, Value::String(s) if s == l_name)
            })
            .ok_or("level not found")?;
        as_map(level_val)?
    };

    // get layers out of the level
    let layers = as_vec(&level["layerInstances"])?;

    // get level height
    let level_height = as_i32(&level["pxHei"])?;

    // iterate over layers
    for (i, layer_val) in layers.iter().enumerate() {
        let layer = as_map(layer_val)?;
        let layer_type = as_str(&layer["__type"])?;

        // check if is tile or entity layer
        match layer_type {
            "Entities" => {
                // extract instances
                let instances = as_vec(&layer["entityInstances"])?;

                for instance_val in instances {
                    let instance = as_map(instance_val)?;

                    // get position
                    let xy = as_vec(&instance["px"])?;
                    let x = as_i32(xy.get(0).ok_or("no x")?)?;
                    let y = level_height - as_i32(xy.get(1).ok_or("no y")?)?;
                    let pos = Vec2::new(x as f32, y as f32);

                    // create entity
                    let mut builder = world.create_entity().with(Position {
                        layer: i as f32,
                        pos,
                    });

                    // get fields
                    let fields = as_vec(&instance["fieldInstances"])?;

                    // add sprite
                    if let Some(sprite) = get_field(&fields, "sprite")? {
                        let name = as_str(sprite)?;
                        let texture = sprites.get(name).expect("bad sprite");
                        let width = texture.read().width();
                        let height = texture.read().height();
                        builder = builder.with(Sprite {
                            texture,
                            part_pos: Vec2::new(0.0, 0.0),
                            part_size: Vec2::new(width as f32, height as f32),
                        });
                    }

                    // add animation
                    if let Some(anim) = get_field(&fields, "animations")? {
                        let string = as_str(anim)?;
                        let mut props = string.split("::");

                        // get size
                        let mut size_iter = props.next().expect("bad animation size").split(',');
                        let columns = size_iter
                            .next()
                            .expect("bad animation")
                            .trim()
                            .parse::<u32>()
                            .expect("bad animation");
                        let rows = size_iter
                            .next()
                            .expect("bad animation")
                            .trim()
                            .parse::<u32>()
                            .expect("bad animation");

                        // get animations
                        let mut animations = vec![];
                        for anim_str in props {
                            let mut parts = anim_str.split(',');
                            let duration = parts
                                .next()
                                .expect("bad animation")
                                .trim()
                                .parse::<u32>()
                                .expect("bad number");
                            let frames: Vec<_> = parts
                                .map(|p| p.trim().parse::<usize>().expect("bad number"))
                                .collect();

                            animations.push(Animation { duration, frames });
                        }

                        builder = builder.with(Animations {
                            size: Vec2::new(columns as f32, rows as f32),
                            current_animation: 0,
                            time: 0.0,
                            animations,
                        })
                    }

                    // set markers
                    if let Some(mark) = get_field(&fields, "markers")? {
                        let markers: Vec<_> = as_str(mark)?.split(',').map(|m| m.trim()).collect();

                        // add movable
                        if markers.contains(&"movable") {
                            builder = builder.with(Movable {
                                target: pos,
                                speed: 2,
                            });
                        }

                        // add player
                        if markers.contains(&"player") {
                            builder = builder.with(Player);
                        }

                        // add pushable
                        if markers.contains(&"pushable") {
                            builder = builder.with(Pushable);
                        }
                    }

                    builder.build();
                }
            }
            "Tiles" => {
                // get identifier
                let id = as_str(&layer["__identifier"])?;

                // get texture name
                let tex_name = as_str(&layer["__tilesetRelPath"])?;

                // extract tiles
                let tiles = as_vec(&layer["gridTiles"])?;

                // get grid size
                let grid_size = as_i32(&layer["__gridSize"])?;

                for tile_val in tiles {
                    let tile = as_map(tile_val)?;

                    // get texture
                    let texture = sprites.get(tex_name).ok_or("texture not found")?.clone();

                    // get coordinates
                    let xy = as_vec(&tile["px"])?;
                    let uv = as_vec(&tile["src"])?;
                    let x = as_i32(xy.get(0).ok_or("no x")?)?;
                    let y = level_height - as_i32(xy.get(1).ok_or("no y")?)?;
                    let u = as_i32(uv.get(0).ok_or("no u")?)?;
                    let v = as_i32(uv.get(1).ok_or("no v")?)?;
                    let pos = Vec2::new(x as f32, y as f32);

                    // create tile entity
                    let mut builder = world
                        .create_entity()
                        .with(Sprite {
                            texture,
                            part_pos: Vec2::new(u as f32, v as f32),
                            part_size: Vec2::new(grid_size as f32, grid_size as f32),
                        })
                        .with(Position {
                            pos,
                            layer: i as f32,
                        });

                    // add solid
                    if id == "Collisions" {
                        builder = builder.with(Solid);
                    }

                    builder.build();
                }
            }
            _ => return Err("invalid layer type".into()),
        }
    }

    Ok(())
}

fn get_field<'a>(fields: &'a [Value], name: &str) -> Result<Option<&'a Value>> {
    for field in fields {
        let map = as_map(field)?;
        let id = as_str(&map["__identifier"])?;
        if id == name {
            let value = &map["__value"];
            return Ok(Some(value));
        }
    }

    Ok(None)
}

fn as_i32(value: &Value) -> Result<i32> {
    match value {
        Value::Number(n) => {
            let i = n.as_i64().ok_or("invalid i32")?;
            Ok(i as i32)
        }
        _ => Err("invalid i32".into()),
    }
}

fn as_str<'a>(value: &'a Value) -> Result<&'a str> {
    match value {
        Value::String(s) => Ok(s),
        _ => Err("invalid str".into()),
    }
}

fn as_vec<'a>(value: &'a Value) -> Result<&'a [Value]> {
    match value {
        Value::Array(vec) => Ok(vec),
        _ => Err("invalid vec".into()),
    }
}

fn as_map<'a>(value: &'a Value) -> Result<&'a Map<String, Value>> {
    match value {
        Value::Object(map) => Ok(map),
        _ => Err("invalid map".into()),
    }
}
