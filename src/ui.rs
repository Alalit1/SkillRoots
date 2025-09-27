use fltk::{button::Button, frame::Frame, group::Group, prelude::*, window::Window};
use fltk::enums::Color;

pub fn manu(win: &mut Window) {
    let stely = "black";

    // робимо змінну mut, щоб міняти
    let mut my_color = Color::Black;

    match stely {
        "black" => my_color = Color::from_rgb(120, 200, 80),
        "w"     => my_color = Color::from_rgb(120, 20, 80),
        _       => my_color = Color::Gray0, // дефолт
    }

    
    let mut screen1 = Group::new(0, 0, 400, 300, "");
    Frame::new(0, 0, 400, 200, "Це екран 1");
    let mut btn_next = Button::new(150, 130, 100, 40, "start");
    let mut btn_setings = Button::new(150, 175, 100, 40, "Setings");
    let mut btn_exit = Button::new(150, 220, 100, 40, "exit");
    btn_next.set_color(Color::from_rgb(50, 150, 200));

    // Задаємо колір тексту
    btn_next.set_label_color(Color::from_rgb(255, 255, 255));
    screen1.end();

    let mut screen2 = Group::new(0, 0, 400, 300, "");
    Frame::new(0, 0, 400, 200, "Це екран 2");
    let mut btn_back = Button::new(150, 220, 100, 40, "Back");
    let mut btn_mash = Button::new(150, 175, 100, 40, "Матиматика");
    //let mut btn_englesh = Button::new(150, 220, 100, 40, "Англська");
    //let mut btn_back = Button::new(150, 220, 100, 40, "Back");
    //let mut btn_back = Button::new(150, 220, 100, 40, "Back");
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
    btn_exit.set_callback(|_| {
    std::process::exit(0); // завершує програму
    });
    btn_setings.set_callback({
        let mut screen1 = screen1.clone();
        let mut screen2 = screen2.clone();
        move |_| {
            screen2.hide();
            screen1.show();
        }
    });
    
}
