use crate::wave_game::*;

pub struct PlayerController {
    pub spellbook: Vec<Spell>,
    pub spell_cursor: usize,
}