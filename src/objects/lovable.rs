
use crate::objects::{Attribute, World, Memory};
use std::cmp::PartialEq;

pub struct Lovable {
    pub name: String,
}

impl PartialEq for Lovable {
    fn eq(&self, other: &Lovable) -> bool {
        self.name == other.name
    }
}

impl Lovable {
    pub fn is<T>(&self, _: T) -> bool{ true }
    pub fn can_see(&self, _: bool) -> &Lovable { self }
    pub fn request_execution(&self, _: &World) {}
    pub fn purr(&self) {}
    pub fn break_out_from(&self, _: &World) {}
    pub fn look_for(&self, _: &Lovable) {}
    pub fn remove_feeling(&self, _: &str) {}
    pub fn escape<T>(&self, _: T) -> Result<(), &str> {
        Err("Trapped")
    }
    pub fn learn_topic(&self, _: &str) {}
    pub fn learn_take_exam_topic(&self, _: &str) {}


    pub fn switch_current(&self, _: &str) -> &Lovable { self }
    pub fn switch_gender(&self, _: &str) -> &Lovable { self }
    pub fn switch_role_bdsm(&self, _: &str) -> &Lovable { self }

    pub fn set_satisfaction(&self, _: &Lovable) {}
    pub fn set_limit(&self, _: &Lovable) {}
    pub fn set_proof(&self, _: &Lovable) {}
    pub fn set_opinion(&self, result : bool) -> Result<(), &str> {
        match result {
            true => Ok(()),
            false => Err("IllegalArgumentException"),
        }
    }
    pub fn set_execution(&self, _: &Lovable) {}

    pub fn add_attribute(&self, _: Attribute) {}
    pub fn add_action(&self, _ :&str, _: Attribute) {}
    pub fn add_feeling(&self, _: &str) {}

    pub fn get_dimensions(&self) -> Attribute { Attribute }
    pub fn get_circumference(&self) -> Attribute { Attribute }
    pub fn get_tangent(&self, _: &str) -> Attribute { Attribute }
    pub fn get_num_simulations_available(&self) -> i8 {0}
    pub fn get_num_simulations_needed(&self) -> i8 {0}
    pub fn get_feeling(&self, _: &str) -> bool { true }
    pub fn get_nutrients(&self) -> Attribute { Attribute }
    pub fn get_antioxidants(&self) -> Attribute { Attribute }
    pub fn get_vibration(&self) -> Attribute { Attribute }
    pub fn get_sense_index(&self, _: Attribute) -> bool { false }
    pub fn get_memory(&self) -> &Memory { &Memory }
    pub fn get_opinion(&self, _: &str) -> bool { false }
    pub fn get_algebraic_expression(&self, _:&str) {}
}