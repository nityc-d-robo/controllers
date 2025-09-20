use super::*;
use p9n_interface::GamepadLayout;
use std::collections::HashMap;
use strum::IntoEnumIterator;

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub enum State {
    Pressed,
    PressedEdge,
    Ignored,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub struct ButtonState(Button, State);

impl Button {
    pub fn state(self, state: State) -> ButtonState {
        ButtonState(self, state)
    }
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct ButtonCombination {
    buttons: HashMap<Button, State>,
}

impl ButtonCombination {
    pub fn new() -> Self {
        ButtonCombination {
            buttons: HashMap::new(),
        }
    }

    pub fn add(&self, button_state: ButtonState) -> Self {
        let mut next = self.clone();
        next.buttons.insert(button_state.0, button_state.1);
        next
    }

    pub fn ignores(&self, buttons: &[Button]) -> Self {
        let mut next = self.clone();
        for &button in buttons {
            next.buttons.insert(button, State::Ignored);
        }
        next
    }

    pub fn evalute<L: GamepadLayout>(&self, gamepad: &mut Gamepad<L>, msg: &Joy) -> bool {
        Button::iter().all(|button| {
            self.buttons
                .get(&button)
                .map_or(!gamepad.pressed(msg, button), |state| match state {
                    State::Pressed => gamepad.pressed(msg, button),
                    State::PressedEdge => gamepad.pressed_edge(msg, button),
                    State::Ignored => true,
                })
        })
    }
}
