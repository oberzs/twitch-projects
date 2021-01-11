use duku::Vec2;
use serde_json::Map;
use serde_json::Value;
use std::fs;
use std::path::Path;

use crate::error::Result;
use crate::world::World;

pub fn load(world: &mut World, path: impl AsRef<Path>, level_name: impl AsRef<str>) -> Result<()> {
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
    for layer_val in layers {
        let layer = as_map(layer_val)?;
        let layer_type = as_str(&layer["__type"])?;

        // check if is tile or entity layer
        match layer_type {
            "Entities" => {
                // extract instances
                let instances = as_vec(&layer["entityInstances"])?;

                for instance_val in instances {
                    let instance = as_map(instance_val)?;
                    let identifier = as_str(&instance["__identifier"])?;

                    // get position
                    let xy = as_vec(&instance["px"])?;
                    let x = as_i32(xy.get(0).ok_or("no x")?)?;
                    let y = level_height - as_i32(xy.get(1).ok_or("no y")?)?;
                    let pos = Vec2::new(x as f32, y as f32);

                    // spawn entity
                    match identifier {
                        "Player" => world.spawn_player(pos),
                        "Box" => world.spawn_box(pos),
                        _ => {}
                    }
                }
            }
            "Tiles" => {
                // get identifier
                let identifier = as_str(&layer["__identifier"])?;

                // get texture name
                let tex_name = as_str(&layer["__tilesetRelPath"])?;

                // extract tiles
                let tiles = as_vec(&layer["gridTiles"])?;

                // get grid size
                let grid_size = as_i32(&layer["__gridSize"])?;

                for tile_val in tiles {
                    let tile = as_map(tile_val)?;

                    // get coordinates
                    let xy = as_vec(&tile["px"])?;
                    let uv = as_vec(&tile["src"])?;
                    let x = as_i32(xy.get(0).ok_or("no x")?)?;
                    let y = level_height - as_i32(xy.get(1).ok_or("no y")?)?;
                    let u = as_i32(uv.get(0).ok_or("no u")?)?;
                    let v = as_i32(uv.get(1).ok_or("no v")?)?;

                    let pos = Vec2::new(x as f32, y as f32);
                    let part_pos = Vec2::new(u as f32, v as f32);
                    let part_size = Vec2::new(grid_size as f32, grid_size as f32);

                    match identifier {
                        "Collisions" => world.spawn_wall(tex_name, pos, part_pos, part_size),
                        "Background" => world.spawn_floor(tex_name, pos, part_pos, part_size),
                        _ => {}
                    }
                }
            }
            _ => return Err("invalid layer type".into()),
        }
    }

    Ok(())
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

fn as_str(value: &Value) -> Result<&str> {
    match value {
        Value::String(s) => Ok(s),
        _ => Err("invalid str".into()),
    }
}

fn as_vec(value: &Value) -> Result<&[Value]> {
    match value {
        Value::Array(vec) => Ok(vec),
        _ => Err("invalid vec".into()),
    }
}

fn as_map(value: &Value) -> Result<&Map<String, Value>> {
    match value {
        Value::Object(map) => Ok(map),
        _ => Err("invalid map".into()),
    }
}
