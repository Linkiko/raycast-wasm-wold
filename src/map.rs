use serde::{Deserialize, Serialize};
pub const SCREEN_WIDTH: u32 = 1280;
pub const SCREEN_HEIGHT: u32 = 720;

#[derive(Serialize, Deserialize)]
pub struct MapBuilder {
    pub height: usize,
    pub width: usize,
    pub map: Vec<Vec<u8>>,
}

pub struct MapContainer {
    pub map: Vec<u8>,
    pub width: usize,
    pub height: usize,
}

impl MapContainer {
    pub fn load(map_builder: MapBuilder) -> MapContainer {
        let mut map_container = MapContainer {
            map: vec![0; map_builder.height * map_builder.width],
            height: map_builder.height,
            width: map_builder.width,
        };
        for i in 0..map_builder.height {
            for j in 0..map_builder.width {
                map_container.set(i, j, map_builder.map[i][j]).unwrap();
            }
        }
        map_container
    }

    pub fn get(&self, x: usize, y: usize) -> u8 {
        self.map[self.width * y + x]
    }

    pub fn get_mut(&mut self, x: usize, y: usize) -> u8 {
        self.map[self.width * y + x]
    }

    pub fn set(&mut self, x: usize, y: usize, val: u8) -> Result<(), String> {
        self.map[self.width * y + x] = val;
        Ok(())
    }
}
