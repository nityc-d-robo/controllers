use crate::ps4_dualshock4::AXES_DUALSHOCK4;
use crate::ps4_dualshock4::BUTTONS_DUALSHOCK4;
use safe_drive::msg::common_interfaces::sensor_msgs;

pub struct DualShock4Interface<'a> {
    msg: &'a sensor_msgs::msg::Joy,
}

impl<'a> DualShock4Interface<'a> {
    pub fn new(_msg: &'a sensor_msgs::msg::Joy) -> Self {
        DualShock4Interface { msg: _msg }
    }
    pub fn set_joy_msg(&mut self, _msg: &'a sensor_msgs::msg::Joy) {
        self.msg = _msg;
    }
    fn is_vaild_button(&self, button: usize) -> bool {
        button < self.msg.buttons.as_slice().len()
    }
    fn is_vaild_axes(&self, axes: usize) -> bool {
        axes < self.msg.buttons.as_slice().len()
    }
    pub fn pressed_ps(&self) -> bool {
        if self.is_vaild_button(BUTTONS_DUALSHOCK4::PS) {
            self.msg.buttons.as_slice()[BUTTONS_DUALSHOCK4::PS] == 1
        } else {
            false
        }
    }
    pub fn pressed_l1(&self) -> bool {
        if self.is_vaild_button(BUTTONS_DUALSHOCK4::L1) {
            self.msg.buttons.as_slice()[BUTTONS_DUALSHOCK4::L1] == 1
        } else {
            false
        }
    }
    pub fn pressed_r1(&self) -> bool {
        if self.is_vaild_button(BUTTONS_DUALSHOCK4::R1) {
            self.msg.buttons.as_slice()[BUTTONS_DUALSHOCK4::R1] == 1
        } else {
            false
        }
    }
    pub fn pressed_r2(&self) -> bool {
        if self.is_vaild_button(BUTTONS_DUALSHOCK4::R2) {
            self.msg.buttons.as_slice()[BUTTONS_DUALSHOCK4::R2] == 1
        } else {
            false
        }
    }
    pub fn pressed_l2(&self) -> bool {
        if self.is_vaild_button(BUTTONS_DUALSHOCK4::L2) {
            self.msg.buttons.as_slice()[BUTTONS_DUALSHOCK4::L2] == 1
        } else {
            false
        }
    }
    pub fn pressed_cross(&self) -> bool {
        if self.is_vaild_button(BUTTONS_DUALSHOCK4::CROSS) {
            self.msg.buttons.as_slice()[BUTTONS_DUALSHOCK4::CROSS] == 1
        } else {
            false
        }
    }
    pub fn pressed_circle(&self) -> bool {
        if self.is_vaild_button(BUTTONS_DUALSHOCK4::CIRCLE) {
            self.msg.buttons.as_slice()[BUTTONS_DUALSHOCK4::CIRCLE] == 1
        } else {
            false
        }
    }
    pub fn pressed_triangle(&self) -> bool {
        if self.is_vaild_button(BUTTONS_DUALSHOCK4::TRIANGLE) {
            self.msg.buttons.as_slice()[BUTTONS_DUALSHOCK4::TRIANGLE] == 1
        } else {
            false
        }
    }
    pub fn pressed_square(&self) -> bool {
        if self.is_vaild_button(BUTTONS_DUALSHOCK4::SQUARE) {
            self.msg.buttons.as_slice()[BUTTONS_DUALSHOCK4::SQUARE] == 1
        } else {
            false
        }
    }
    pub fn pressed_dpad_left(&self) -> bool {
        if self.is_vaild_axes(AXES_DUALSHOCK4::DPAD_X) {
            self.msg.axes.as_slice()[AXES_DUALSHOCK4::DPAD_X] > 0.0
        } else {
            false
        }
    }
    pub fn pressed_dpad_right(&self) -> bool {
        if self.is_vaild_axes(AXES_DUALSHOCK4::DPAD_X) {
            self.msg.axes.as_slice()[AXES_DUALSHOCK4::DPAD_X] < 0.0
        } else {
            false
        }
    }
    pub fn pressed_dpad_up(&self) -> bool {
        if self.is_vaild_axes(AXES_DUALSHOCK4::DPAD_Y) {
            self.msg.axes.as_slice()[AXES_DUALSHOCK4::DPAD_Y] > 0.0
        } else {
            false
        }
    }
    pub fn pressed_dpad_down(&self) -> bool {
        if self.is_vaild_axes(AXES_DUALSHOCK4::DPAD_Y) {
            self.msg.axes.as_slice()[AXES_DUALSHOCK4::DPAD_Y] < 0.0
        } else {
            false
        }
    }
}
