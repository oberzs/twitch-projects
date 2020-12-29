use duku::Duku;
use duku::Handle;
use duku::Texture;
use std::collections::HashMap;
use std::path::Path;

use super::Result;

#[derive(Default)]
pub struct Sprites {
    sprites: HashMap<String, Handle<Texture>>,
}

impl Sprites {
    pub fn add(&mut self, duku: &mut Duku, path: impl AsRef<Path>) -> Result<()> {
        let p = path.as_ref();
        let name = p
            .file_name()
            .map(|n| n.to_str().unwrap_or(""))
            .unwrap_or("");
        let sprite = duku.create_texture_png(p, None)?;
        self.sprites.insert(name.to_string(), sprite);
        Ok(())
    }

    pub fn get(&self, name: impl AsRef<str>) -> Option<Handle<Texture>> {
        self.sprites.get(name.as_ref()).cloned()
    }
}
