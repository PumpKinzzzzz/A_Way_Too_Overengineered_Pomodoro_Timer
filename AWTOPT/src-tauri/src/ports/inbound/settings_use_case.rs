pub trait SettingsPort {
    fn update_durations(&mut self, work_duration: u64, short_break_duration: u64, long_break_duration: u64);
    fn toggle_auto_start_breaks(&mut self);
    fn update_sequence_list(&mut self, sequence_list: Vec<Sequence>);
}