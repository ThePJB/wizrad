use crate::spell::*;

#[derive(Clone)]
pub struct Player {
    pub spellbook: Vec<Spell>,
    pub spell_cursor: i32,
    pub speed: f32,
}