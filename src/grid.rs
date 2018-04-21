use std::collections::HashSet;

use embla::math::Vec2;

pub struct Grid {
    width: i32,
    height: i32,
    cell_width: i32,
    cell_height: i32,
    occupied: HashSet<(i32, i32)>,
}

impl Grid {
    pub fn new(width: i32, height: i32, cell_width: i32, cell_height: i32) -> Grid {
        Grid {
            width,
            height,
            cell_width,
            cell_height,
            occupied: HashSet::new(),
        }
    }

    pub fn width(&self) -> i32 {
        self.width
    }

    pub fn height(&self) -> i32 {
        self.height
    }

    pub fn cell_width(&self) -> i32 {
        self.cell_width
    }

    pub fn cell_height(&self) -> i32 {
        self.cell_height
    }

    pub fn cell_rect(&self, x: i32, y: i32) -> (i32, i32, i32, i32) {
        let (x, y) = (x * self.cell_width, y * self.cell_height);
        (x, y, x + self.cell_width, y + self.cell_height)
    }

    pub fn tile_at(&self, pos: Vec2) -> Option<(i32, i32)> {
        let mouse_tile = (
            (pos.0 / self.cell_width as f32).floor() as i32,
            (pos.1 / self.cell_height as f32).floor() as i32,
        );
        if mouse_tile.0 >= 0 && mouse_tile.0 < self.width && mouse_tile.1 >= 0
            && mouse_tile.1 < self.height
        {
            Some(mouse_tile)
        } else {
            None
        }
    }

    pub fn set_occupied(&mut self, tile: (i32, i32), occupied: bool) {
        if occupied {
            self.occupied.insert(tile);
        } else {
            self.occupied.remove(&tile);
        }
    }

    pub fn occupied(&self, tile: &(i32, i32)) -> bool {
        self.occupied.contains(tile)
    }
}
