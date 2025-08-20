use safe_drive::msg::interfaces::lifecycle_msgs::msg::State;

use super::*;
// 共通のインターフェース

pub enum Button {
    PS,
    L1,
    R1,
    L2,
    R2,
    Cross,
    Circle,
    Triangle,
    Square,
    DpadLeft,
    DpadRight,
    DpadUp,
    DpadDown,
}

pub type ButtonState = [bool; 12];

pub trait GamepadLayout {
    fn ps(&self, msg: &Joy) -> bool;
    fn l1(&self, msg: &Joy) -> bool;
    fn r1(&self, msg: &Joy) -> bool;
    fn l2(&self, msg: &Joy) -> bool;
    fn r2(&self, msg: &Joy) -> bool;
    fn cross(&self, msg: &Joy) -> bool;
    fn circle(&self, msg: &Joy) -> bool;
    fn triangle(&self, msg: &Joy) -> bool;
    fn square(&self, msg: &Joy) -> bool;
    fn dpad_left(&self, msg: &Joy) -> bool;
    fn dpad_right(&self, msg: &Joy) -> bool;
    fn dpad_up(&self, msg: &Joy) -> bool;
    fn dpad_down(&self, msg: &Joy) -> bool;

    // 共通のヘルパー
    fn is_valid_button(&self, msg: &Joy, button: usize) -> bool {
        button < msg.buttons.len()
    }
    fn is_valid_axis(&self, msg: &Joy, axis: usize) -> bool {
        axis < msg.axes.len()
    }
}

pub struct Gamepad<'a, L: GamepadLayout> {
    msg: &'a Joy,
    layout: L,
}

impl<'a, L: GamepadLayout> Gamepad<'a, L> {
    pub fn new(msg: &'a Joy, layout: L) -> Self {
        Self { msg, layout }
    }

    pub fn pressed_ps(&self) -> bool {
        self.layout.ps(&self.msg)
    }

    pub fn pressed_l1(&self) -> bool {
        self.layout.l1(&self.msg)
    }

    pub fn pressed_r1(&self) -> bool {
        self.layout.r1(&self.msg)
    }

    pub fn pressed_l2(&self) -> bool {
        self.layout.l2(&self.msg)
    }

    pub fn pressed_r2(&self) -> bool {
        self.layout.r2(&self.msg)
    }

    pub fn pressed_cross(&self) -> bool {
        self.layout.cross(&self.msg)
    }

    pub fn pressed_circle(&self) -> bool {
        self.layout.circle(&self.msg)
    }

    pub fn pressed_triangle(&self) -> bool {
        self.layout.triangle(&self.msg)
    }

    pub fn pressed_square(&self) -> bool {
        self.layout.square(&self.msg)
    }

    pub fn pressed_dpad_left(&self) -> bool {
        self.layout.dpad_left(&self.msg)
    }

    pub fn pressed_dpad_right(&self) -> bool {
        self.layout.dpad_right(&self.msg)
    }

    pub fn pressed_dpad_up(&self) -> bool {
        self.layout.dpad_up(&self.msg)
    }

    pub fn pressed_dpad_down(&self) -> bool {
        self.layout.dpad_down(&self.msg)
    }

    pub fn pressed_ps_edge(&mut self, prev_buttons: &mut ButtonState) -> bool {
        let idx = Button::PS as usize;
        std::mem::replace(&mut prev_buttons[idx], self.pressed_ps()) == false && prev_buttons[idx]
    }

    pub fn pressed_l1_edge(&mut self, prev_buttons: &mut ButtonState) -> bool {
        let idx = Button::L1 as usize;
        std::mem::replace(&mut prev_buttons[idx], self.pressed_l1()) == false && prev_buttons[idx]
    }

    pub fn pressed_r1_edge(&mut self, prev_buttons: &mut ButtonState) -> bool {
        let idx = Button::R1 as usize;
        std::mem::replace(&mut prev_buttons[idx], self.pressed_r1()) == false && prev_buttons[idx]
    }

    pub fn pressed_l2_edge(&mut self, prev_buttons: &mut ButtonState) -> bool {
        let idx = Button::L2 as usize;
        std::mem::replace(&mut prev_buttons[idx], self.pressed_l2()) == false && prev_buttons[idx]
    }

    pub fn pressed_r2_edge(&mut self, prev_buttons: &mut ButtonState) -> bool {
        let idx = Button::R2 as usize;
        std::mem::replace(&mut prev_buttons[idx], self.pressed_r2()) == false && prev_buttons[idx]
    }

    pub fn pressed_cross_edge(&mut self, prev_buttons: &mut ButtonState) -> bool {
        let idx = Button::Cross as usize;
        std::mem::replace(&mut prev_buttons[idx], self.pressed_cross()) == false
            && prev_buttons[idx]
    }

    pub fn pressed_circle_edge(&mut self, prev_buttons: &mut ButtonState) -> bool {
        let idx = Button::Circle as usize;
        std::mem::replace(&mut prev_buttons[idx], self.pressed_circle()) == false
            && prev_buttons[idx]
    }

    pub fn pressed_triangle_edge(&mut self, prev_buttons: &mut ButtonState) -> bool {
        let idx = Button::Triangle as usize;
        std::mem::replace(&mut prev_buttons[idx], self.pressed_triangle()) == false
            && prev_buttons[idx]
    }

    pub fn pressed_square_edge(&mut self, prev_buttons: &mut ButtonState) -> bool {
        let idx = Button::Square as usize;
        std::mem::replace(&mut prev_buttons[idx], self.pressed_square()) == false
            && prev_buttons[idx]
    }

    pub fn pressed_dpad_left_edge(&mut self, prev_buttons: &mut ButtonState) -> bool {
        let idx = Button::DpadLeft as usize;
        std::mem::replace(&mut prev_buttons[idx], self.pressed_dpad_left()) == false
            && prev_buttons[idx]
    }

    pub fn pressed_dpad_right_edge(&mut self, prev_buttons: &mut ButtonState) -> bool {
        let idx = Button::DpadRight as usize;
        std::mem::replace(&mut prev_buttons[idx], self.pressed_dpad_right()) == false
            && prev_buttons[idx]
    }

    pub fn pressed_dpad_up_edge(&mut self, prev_buttons: &mut ButtonState) -> bool {
        let idx = Button::DpadUp as usize;
        std::mem::replace(&mut prev_buttons[idx], self.pressed_dpad_up()) == false
            && prev_buttons[idx]
    }

    pub fn pressed_dpad_down_edge(&mut self, prev_buttons: &mut ButtonState) -> bool {
        let idx = Button::DpadDown as usize;
        std::mem::replace(&mut prev_buttons[idx], self.pressed_dpad_down()) == false
            && prev_buttons[idx]
    }
}
