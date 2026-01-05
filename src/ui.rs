use fltk::{button::Button, frame::Frame, group::Group, prelude::*, window::Window};
use fltk::enums::Color;
use crate::vidgets::{ButtonConfig, create_button};

//use vidgets::{ButtonConfig, create_button};
pub fn manu(win: &mut Window) {
    let stely = "black";

    // робимо змінну mut, щоб міняти
    let mut my_color = Color::Black;

    match stely {
        "black" => my_color = Color::from_rgb(120, 200, 80),
        "w"     => my_color = Color::from_rgb(120, 20, 80),
        _       => my_color = Color::Gray0, // дефолт
    }

    let buttons = vec![
    ButtonDef {
        cfg: ButtonConfig { x: 140, y: 130, w: 120, h: 35, label: "Start" },
        action: Action::Start,
    },
    ButtonDef {
        cfg: ButtonConfig { x: 140, y: 175, w: 120, h: 35, label: "Settings" },
        action: Action::Settings,
    },
];


    let mut screen1 = Group::new(0, 0, 400, 300, "");
    Frame::new(0, 0, 400, 200, "Це екран 1");

enum Action {
    Start,
    Settings,
    Exit,
}

struct ButtonDef<'a> {
    cfg: ButtonConfig<'a>,
    action: Action,
}


    for b in buttons {
    let action = b.action;
    create_button(b.cfg, move || {
        match action {
            Action::Start => println!("start"),
            Action::Settings => println!("settings"),
            Action::Exit => println!("exit"),
        }
    });
}
}