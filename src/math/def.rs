pub fn calculate_char_base_def(base: u32, cone: u32) -> u32 {
    base + cone
}

pub fn calculate_enemy_base_def(level: u8) -> u32 {
    200 + 10 * (level as u32)
}

pub fn calculate_char_def_multiplier(def: f32, level: u8) -> f32 {
    1.0 - (def / (def + calculate_enemy_base_def(level) as f32))
}
