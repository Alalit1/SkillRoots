use fltk::prelude::*; // містить WidgetBase, який потрібен для new()
use fltk::{app::App, window::Window, image::PngImage};
use crate::ui;



pub struct ScreenManager {
    pub width: u16,
    pub height: u16,
}

impl ScreenManager {
    pub fn new(width: u16, height: u16) -> Self {
        Self { width, height }
    }

    pub fn draw(&self) {
        let app = App::default();
        let mut wind = Window::new(100, 100, self.width.into(), self.height.into(), "SkillRoots");

        // Завантажуемо іконку для вікна
        let icon = PngImage::load("Icon_SkillRoots.png").unwrap();

        // Встановлюємо іконку вікна
        wind.set_icon(Some(icon));
        //wind.set_color(Color::Black);

        // Виклик функції з ui.rs
        ui::manu();


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