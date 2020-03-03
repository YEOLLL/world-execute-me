use super::lovable;

pub struct World {
    pub things: Vec<String>,
}


impl World {
    pub fn new(&self) -> &World {
        self
    }

    fn is_thing_exist(self, thing: String) -> bool {
        match self.things.binary_search(&thing) {
            Ok(_) => true,
            Err(_) => false,
        }
    }

    pub fn add_thing(&mut self, thing: lovable::Lovable) -> &mut World {
        self.things.push(thing.name);
        self
    }

    pub fn start_simulation(&self) -> bool {
        true
    }
}