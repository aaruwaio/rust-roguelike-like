use crate::item::Item;
use crate::point::Point;

#[derive(Copy, Clone)]
pub struct Grid {
    pos: Point,
    item: Option<Item>,
}

impl Grid {
    pub fn new(x: u32, y: u32) -> Self {
        Self {
            pos: Point { x, y },
            item: None,
        }
    }

    pub fn get_pos(&self) -> Point {
        self.pos
    }

    pub fn get_item(&self) -> Option<Item> {
        self.item
    }

    pub fn set_item(&mut self, item: Option<Item>) {
        self.item = item;
    }

    pub fn create_item(&mut self, item: Option<Item>) -> Result<(), String> {
        match self.get_item() {
            Some(_) => Err(format!("conflict items")),
            None => {
                self.set_item(item);
                Ok(())
            }
        }
    }
}
