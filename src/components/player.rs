use crate::wave_game::*;

pub struct Player {
    pub spellbook: Vec<Spell>,
    pub spell_cursor: usize,
}