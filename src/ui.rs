use fltk::{button::Button, frame::Frame, group::Group, prelude::*, window::Window};
use fltk::enums::Color;
use crate::vidgets::{ButtonConfig, create_button};

#[derive(Copy, Clone, Debug)]
pub enum Action {
    Start,
    Settings,
    Exit,
}


pub struct ButtonDef<'a> {
    cfg: ButtonConfig<'a>,
    action: Action,
}

//use vidgets::{ButtonConfig, create_button};
pub fn build_main_menu(
    on_action: impl Fn(Action) + Clone + 'static,
) -> Group {
    let mut group = Group::new(0, 0, 400, 300, "");
    Frame::new(0, 0, 400, 200, "Це головне меню");

    let buttons = vec![
        ButtonDef {
            cfg: ButtonConfig { x: 50, y: 50, w: 120, h: 35, label: "Start" },
            action: Action::Start,
        },
        ButtonDef {
            cfg: ButtonConfig { x: 50, y: 95, w: 120, h: 35, label: "Settings" },
            action: Action::Settings,
        },
    ];

    for b in buttons {
        let action = b.action;
        let cb = on_action.clone();

        create_button(b.cfg, move || cb(action));
    }

    group.end();
    group
}
pub fn build_settings(
    on_action: impl Fn(Action) + Clone + 'static,
) -> Group {
    let mut group = Group::new(0, 0, 400, 300, "");
    Frame::new(0, 0, 400, 200, "Це головне меню");

    let buttons = vec![
        ButtonDef {
            cfg: ButtonConfig { x: 50, y: 50, w: 120, h: 35, label: "Start" },
            action: Action::Start,
        },
        ButtonDef {
            cfg: ButtonConfig { x: 50, y: 95, w: 120, h: 35, label: "Settings" },
            action: Action::Settings,
        },
    ];

    for b in buttons {
        let action = b.action;
        let cb = on_action.clone();

        create_button(b.cfg, move || cb(action));
    }

    group.end();
    group
}
pub fn build_mash(
    on_action: impl Fn(Action) + Clone + 'static,
) -> Group {
    let mut group = Group::new(0, 0, 400, 300, "");
    Frame::new(0, 0, 400, 200, "Це головне меню");

    let buttons = vec![
        ButtonDef {
            cfg: ButtonConfig { x: 50, y: 50, w: 120, h: 35, label: "Start" },
            action: Action::Start,
        },
        ButtonDef {
            cfg: ButtonConfig { x: 50, y: 95, w: 120, h: 35, label: "Settings" },
            action: Action::Settings,
        },
    ];

    for b in buttons {
        let action = b.action;
        let cb = on_action.clone();

        create_button(b.cfg, move || cb(action));
    }

    group.end();
    group
}
pub fn build_tiping(
    on_action: impl Fn(Action) + Clone + 'static,
) -> Group {
    let mut group = Group::new(0, 0, 400, 300, "");
    Frame::new(0, 0, 400, 200, "Це головне меню");

    let buttons = vec![
        ButtonDef {
            cfg: ButtonConfig { x: 50, y: 50, w: 120, h: 35, label: "Start" },
            action: Action::Start,
        },
        ButtonDef {
            cfg: ButtonConfig { x: 50, y: 95, w: 120, h: 35, label: "Settings" },
            action: Action::Settings,
        },
    ];

    for b in buttons {
        let action = b.action;
        let cb = on_action.clone();

        create_button(b.cfg, move || cb(action));
    }

    group.end();
    group
}
pub fn build_langue(
    on_action: impl Fn(Action) + Clone + 'static,
) -> Group {
    let mut group = Group::new(0, 0, 400, 300, "");
    Frame::new(0, 0, 400, 200, "Це головне меню");

    let buttons = vec![
        ButtonDef {
            cfg: ButtonConfig { x: 50, y: 50, w: 120, h: 35, label: "Start" },
            action: Action::Start,
        },
        ButtonDef {
            cfg: ButtonConfig { x: 50, y: 95, w: 120, h: 35, label: "Settings" },
            action: Action::Settings,
        },
    ];

    for b in buttons {
        let action = b.action;
        let cb = on_action.clone();

        create_button(b.cfg, move || cb(action));
    }

    group.end();
    group
}
