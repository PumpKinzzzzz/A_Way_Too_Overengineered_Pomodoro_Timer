pub trait Save {
    fn save_settings(&self, duration: (u64, u64, u64), sequence_list: Vec<String>) -> Result<(), String>;
    fn save_timer_state(&self, state: String, sequence_index: usize) -> Result<(), String>;
}