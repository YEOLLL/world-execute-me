use super::lovable::Lovable;

pub struct World<'a> {
    pub things: Vec<&'a Lovable>,
}

pub struct WorldBuilder {}

impl WorldBuilder {
    pub fn new<'a>() -> World<'a> {
        World {
            things: vec![],
        }
    }
}

impl<'a> World<'a> {

    pub fn initialisation(&self) -> &World { self }
    pub fn start_simulation(&self) {}
    pub fn add_thing(&mut self, thing: &'a Lovable) -> &mut World<'a> {
        self.things.push(thing);
        self
    }
    pub fn time_travel_for_us(&self, _: &str, _: i32, _: &Lovable, _: &Lovable) {}
    pub fn unite(&self,_: &Lovable, _: &Lovable) {}
    pub fn lock_thing(&self, _: &Lovable) {}
    pub fn get_god(&self) -> Lovable {
        let god = Lovable {
            name: String::from("me"),
        };
        god
    }
    pub fn do_whatever(&self, _: &Lovable, _: &Lovable, _: &str, _: &str) {}
    pub fn make_high(&self, _: &Lovable) {}
    pub fn announce(&self, _: &str) {}
    pub fn execution(&self) {}
    pub fn is_execution_by(&self, _: &Lovable) -> bool { true }
    pub fn get_thing(&self, thing: &Lovable) -> bool {
        match thing.name.as_str() {
            "you" => false,
            _ => true,
        }
    }
    pub fn execute(&self, _: &Lovable) {}
}