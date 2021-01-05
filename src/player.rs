use crate::direction::Dir;
use crate::field::Field;
use crate::grid::Grid;
use crate::item::Item;
use crate::point::Point;

const FIELD_SIZE_X: usize = 12;
const FIELD_SIZE_Y: usize = 5;

#[derive(Debug)]
pub struct Player {
    pos: Point,
    items: Vec<Item>,
}

impl<'a> Player {
    pub fn new(x: u32, y: u32) -> Self {
        Self {
            pos: Point { x, y },
            items: Vec::new(),
        }
    }

    pub fn get_pos(&self) -> Point {
        self.pos
    }

    pub fn get_grid(&self, field: &'a mut Field) -> &'a mut Grid {
        field.get_grid(self.get_pos().x as usize, self.get_pos().y as usize)
    }

    pub fn set_pos(&mut self, dir: Dir) {
        match dir {
            Dir::Up if self.pos.y > 0 => {
                self.pos.y -= 1;
            }
            Dir::Down if self.pos.y < FIELD_SIZE_Y as u32 - 1 => {
                self.pos.y += 1;
            }
            Dir::Left if self.pos.x > 0 => {
                self.pos.x -= 1;
            }
            Dir::Right if self.pos.x < FIELD_SIZE_X as u32 - 1 => {
                self.pos.x += 1;
            }
            _ => {}
        }
    }

    pub fn take_item(&mut self, field: &mut Field) -> Option<String> {
        if let Some(item) = self.get_grid(field).get_item() {
            self.get_grid(field).set_item(None);
            self.items.push(item);
            println!("{:?}", self.items);
            match item {
                Item::Apple => return Some(String::from("りんご")),
                Item::Orange => return Some(String::from("みかん")),
                Item::Lemon => return Some(String::from("れもん")),
            }
        }
        None
    }
}
