// Internal types shared across workshop workers
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum Sequence {
    Work,
    ShortBreak,
    LongBreak,
}
