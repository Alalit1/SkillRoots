use fltk::{app, button::Button, frame::Frame, group::Group, prelude::*, window::Window};

fn main() {
    let app = app::App::default();
    let mut wind = Window::new(100, 100, 400, 300, "Screens example");

    // --- Перший "екран"
    let mut screen1 = Group::new(0, 0, 400, 300, "");
    let mut frame1 = Frame::new(0, 0, 400, 200, "Це екран 1");
    let mut btn_next = Button::new(150, 220, 100, 40, "Start");
    let mut btn_next = Button::new(150, 220, 100, 40, "Setings");
    let mut btn_next = Button::new(150, 220, 100, 40, "Ekit");
    screen1.end();

    // --- Другий "екран"
    let mut screen2 = Group::new(0, 0, 400, 300, "");
    let mut frame2 = Frame::new(0, 0, 400, 200, "Це екран 2");
    let mut btn_next = Button::new(150, 220, 100, 40, "Матиматика");
    let mut btn_next = Button::new(150, 220, 100, 40, "Украінська мова");
    let mut btn_next = Button::new(150, 220, 100, 40, "Англіська мова");
    let mut btn_back = Button::new(150, 220, 100, 40, "Назад");
    screen2.end();

    // Спочатку ховаємо другий екран
    screen2.hide();

    wind.end();
    wind.show();

    // --- Логіка перемикання ---
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

    app.run().unwrap();
}
