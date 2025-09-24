use fltk::{app, button::Button, frame::Frame, group::Group, prelude::*, window::Window};

pub struct ScreenManager {
    pub width: u16,
    pub height: u16,
}

impl ScreenManager {
    pub fn new(width: u16, height: u16) -> Self {
        Self { width, height }
    }

    pub fn draw(&self) {
        let app = app::App::default();
        let mut wind = Window::new(100, 100, self.width.into(), self.height.into(), "Screens example");

        let screen1 = Group::new(0, 0, 400, 300, "");
        let _frame1 = Frame::new(0, 0, 400, 200, "Це екран 1");
        let _btn_next = Button::new(150, 220, 100, 40, "Start");
        screen1.end();

        let mut screen2 = Group::new(0, 0, 400, 300, "");
        let _frame2 = Frame::new(0, 0, 400, 200, "Це екран 2");
        screen2.end();
        screen2.hide();

        wind.end();
        wind.show();
        app.run().unwrap();
    }
    /*pub fn update(){
        /*
        Функція заміни отрисованого екрана
        */
        // --- Логіка перемикання --- код функція кнопки
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
    }*/
}