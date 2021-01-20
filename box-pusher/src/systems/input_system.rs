use duku::window::Events;
use duku::window::Key;
use gilrs::Button as GButton;
use gilrs::Event;
use gilrs::Gamepad;
use gilrs::Gilrs;
use specs::System;
use specs::Write;

use crate::resources::Button;
use crate::resources::Inputs;

pub struct InputSystem<'a> {
    pub events: &'a Events,
    pub gilrs: &'a mut Gilrs,
}

impl<'a> System<'a> for InputSystem<'a> {
    type SystemData = Write<'a, Inputs>;

    fn run(&mut self, data: Self::SystemData) {
        let mut inputs = data;

        // get the active gamepad if exists
        let mut gamepad_id = None;
        while let Some(Event { id, .. }) = self.gilrs.next_event() {
            gamepad_id = Some(id);
        }
        let gamepad = gamepad_id.map(|g| self.gilrs.gamepad(g));

        // handle inputs
        if gamepad_pressed(&gamepad, GButton::DPadUp) || self.events.is_key_pressed(Key::W) {
            inputs.keys_pressed.insert(Button::Up);
        } else {
            inputs.keys_pressed.remove(&Button::Up);
        }
        if gamepad_pressed(&gamepad, GButton::DPadDown) || self.events.is_key_pressed(Key::S) {
            inputs.keys_pressed.insert(Button::Down);
        } else {
            inputs.keys_pressed.remove(&Button::Down);
        }
        if gamepad_pressed(&gamepad, GButton::DPadLeft) || self.events.is_key_pressed(Key::A) {
            inputs.keys_pressed.insert(Button::Left);
        } else {
            inputs.keys_pressed.remove(&Button::Left);
        }
        if gamepad_pressed(&gamepad, GButton::DPadRight) || self.events.is_key_pressed(Key::D) {
            inputs.keys_pressed.insert(Button::Right);
        } else {
            inputs.keys_pressed.remove(&Button::Right);
        }
    }
}

fn gamepad_pressed(gamepad: &Option<Gamepad>, button: GButton) -> bool {
    gamepad.map(|g| g.is_pressed(button)).unwrap_or(false)
}
