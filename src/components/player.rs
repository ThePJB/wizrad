use crate::spell::*;

pub struct Player {
    pub spellbook: Vec<Spell>,
    pub spell_cursor: usize,
    pub speed: f32,
}