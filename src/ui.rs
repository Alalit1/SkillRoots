use fltk::{button::Button, frame::Frame, group::Group, prelude::*, window::Window};

pub fn manu(win: &mut Window) {
    let mut screen1 = Group::new(0, 0, 400, 300, "");
    Frame::new(0, 0, 400, 200, "Це екран 1");
    let mut btn_next = Button::new(150, 220, 100, 40, "Start");
    screen1.end();

    let mut screen2 = Group::new(0, 0, 400, 300, "");
    Frame::new(0, 0, 400, 200, "Це екран 2");
    let mut btn_back = Button::new(150, 220, 100, 40, "Back");
    screen2.end();
    screen2.hide();

    btn_next.set_callback({
        let mut screen1 = screen1.clone();
        let mut screen2 = screen2.clone();
        move |_| {
            screen1.hide();
            screen2.show();
        }
    });

    btn_back.set_callback({
        let mut screen1 = screen1.clone();
        let mut screen2 = screen2.clone();
        move |_| {
            screen2.hide();
            screen1.show();
        }
    });

    win.add(&screen1);
    win.add(&screen2);
}
