use std::collections::HashSet;

#[derive(Default)]
pub struct Inputs {
    pub keys_pressed: HashSet<Button>,
}

#[derive(Copy, Clone, PartialEq, Eq, Hash)]
pub enum Button {
    Up,
    Down,
    Left,
    Right,
}
