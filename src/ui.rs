use fltk::{app, button::Button, frame::Frame, group::Group, prelude::*, window::Window};

pub fn manu() {
    let _frame1 = Frame::new(0, 0, 400, 200, "Це екран 1");
    let _btn_next = Button::new(150, 220, 100, 40, "Start");
    let _btn_exit = Button::new(100, 150, 100, 40, "Exit");
}

pub fn manu_lason() {
    let _frame2 = Frame::new(0, 0, 400, 200, "Це екран 2");
}
