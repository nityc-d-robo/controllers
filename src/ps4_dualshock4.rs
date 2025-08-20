use super::p9n_interface::GamepadLayout;
use super::Joy;

#[allow(non_snake_case, non_upper_case_globals)]
pub mod AXES_DUALSHOCK4 {
    pub const STICK_LX: usize = 0;
    pub const STICK_LY: usize = 1;
    pub const L2: usize = 2;
    pub const STICK_RX: usize = 3;
    pub const STICK_RY: usize = 4;
    pub const R2: usize = 5;
    pub const DPAD_X: usize = 6;
    pub const DPAD_Y: usize = 7;
}

#[allow(non_snake_case, non_upper_case_globals)]
pub mod BUTTONS_DUALSHOCK4 {
    pub const CROSS: usize = 0;
    pub const CIRCLE: usize = 1;
    pub const TRIANGLE: usize = 2;
    pub const SQUARE: usize = 3;
    pub const L1: usize = 4;
    pub const R1: usize = 5;
    pub const L2: usize = 6;
    pub const R2: usize = 7;
    pub const SELECT: usize = 8;
    pub const START: usize = 9;
    pub const PS: usize = 10;
}

pub struct DualShock4Layout;

impl GamepadLayout for DualShock4Layout {
    fn ps(&self, msg: &Joy) -> bool {
        self.is_valid_button(msg, BUTTONS_DUALSHOCK4::PS)
            && msg.buttons.as_slice()[BUTTONS_DUALSHOCK4::PS] == 1
    }
    fn l1(&self, msg: &Joy) -> bool {
        self.is_valid_button(msg, BUTTONS_DUALSHOCK4::L1)
            && msg.buttons.as_slice()[BUTTONS_DUALSHOCK4::L1] == 1
    }
    fn r1(&self, msg: &Joy) -> bool {
        self.is_valid_button(msg, BUTTONS_DUALSHOCK4::R1)
            && msg.buttons.as_slice()[BUTTONS_DUALSHOCK4::R1] == 1
    }
    fn l2(&self, msg: &Joy) -> bool {
        self.is_valid_button(msg, BUTTONS_DUALSHOCK4::L2)
            && msg.buttons.as_slice()[BUTTONS_DUALSHOCK4::L2] == 1
    }
    fn r2(&self, msg: &Joy) -> bool {
        self.is_valid_button(msg, BUTTONS_DUALSHOCK4::R2)
            && msg.buttons.as_slice()[BUTTONS_DUALSHOCK4::R2] == 1
    }
    fn cross(&self, msg: &Joy) -> bool {
        self.is_valid_button(msg, BUTTONS_DUALSHOCK4::CROSS)
            && msg.buttons.as_slice()[BUTTONS_DUALSHOCK4::CROSS] == 1
    }
    fn circle(&self, msg: &Joy) -> bool {
        self.is_valid_button(msg, BUTTONS_DUALSHOCK4::CIRCLE)
            && msg.buttons.as_slice()[BUTTONS_DUALSHOCK4::CIRCLE] == 1
    }
    fn triangle(&self, msg: &Joy) -> bool {
        self.is_valid_button(msg, BUTTONS_DUALSHOCK4::TRIANGLE)
            && msg.buttons.as_slice()[BUTTONS_DUALSHOCK4::TRIANGLE] == 1
    }
    fn square(&self, msg: &Joy) -> bool {
        self.is_valid_button(msg, BUTTONS_DUALSHOCK4::SQUARE)
            && msg.buttons.as_slice()[BUTTONS_DUALSHOCK4::SQUARE] == 1
    }
    fn dpad_left(&self, msg: &Joy) -> bool {
        self.is_valid_axis(msg, AXES_DUALSHOCK4::DPAD_X)
            && msg.axes.as_slice()[AXES_DUALSHOCK4::DPAD_X] > 0.0
    }
    fn dpad_right(&self, msg: &Joy) -> bool {
        self.is_valid_axis(msg, AXES_DUALSHOCK4::DPAD_X)
            && msg.axes.as_slice()[AXES_DUALSHOCK4::DPAD_X] < 0.0
    }
    fn dpad_up(&self, msg: &Joy) -> bool {
        self.is_valid_axis(msg, AXES_DUALSHOCK4::DPAD_Y)
            && msg.axes.as_slice()[AXES_DUALSHOCK4::DPAD_Y] > 0.0
    }
    fn dpad_down(&self, msg: &Joy) -> bool {
        self.is_valid_axis(msg, AXES_DUALSHOCK4::DPAD_Y)
            && msg.axes.as_slice()[AXES_DUALSHOCK4::DPAD_Y] < 0.0
    }
}
