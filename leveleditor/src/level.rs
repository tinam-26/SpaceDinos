use std::path::Path;
use serde::{Deserialize, Serialize};

#[derive(Serialize, Deserialize, Debug)]
pub struct LevelObject {
    pub name: String,
    pub x: isize,
    pub y: isize,
}

#[allow(non_snake_case)]
#[derive(Serialize, Deserialize, Debug)]
pub struct Level {
    pub levelObjects: Vec<LevelObject>,
}

const OBJECT_TYPES: &'static [&'static str] = &[
    "block",
    "platform",
    "coin",
    "spikes",
    "invisible_block",
];

fn find_obj_type_index(search_name: &str) -> Option<usize> {
    OBJECT_TYPES
        .iter()
        .enumerate()
        .find_map(|(i, name)|
            if search_name == *name {
                Some(i)
            } else {
                None
            }
        )
}

impl Level {
    pub fn open<P: AsRef<Path>>(path: P) -> Option<Self> {
        serde_json::from_str(
            &std::fs::read_to_string(path.as_ref()).ok()?
        ).ok()
    }

    pub fn save<P: AsRef<Path>>(&self, path: P) -> Result<(), std::io::Error> {
        std::fs::write(path, 
            serde_json::to_string(&self)?
        )?;
        Ok(())
    }

    pub fn get_at_pos(&mut self, x: isize, y: isize) -> Option<LevelObject> {
        for i in 0..self.levelObjects.len() {
            if self.levelObjects[i].x == x && self.levelObjects[i].y == y {
                return Some(self.levelObjects.remove(i));
            }
        }
        None
    }

    pub fn toggle_position(&mut self, x: isize, y: isize) {
        if let Some(mut object) = self.get_at_pos(x, y) {
            if let Some(pos) = find_obj_type_index(&object.name[..]) {
                if pos != OBJECT_TYPES.len() - 1 {
                    object.name = String::from(OBJECT_TYPES[pos + 1]);
                    self.levelObjects.push(object);
                }
            }
        } else {
            self.levelObjects.push(LevelObject {
                name: String::from(OBJECT_TYPES[0]),
                x,
                y
            });
        }
    }
}

impl Default for Level {
    fn default() -> Self {
        Level {
            levelObjects: vec![]
        }
    }
}