use fltk::prelude::*; // містить WidgetBase, який потрібен для new()
use fltk::{app::App, window::Window, image::PngImage};
use crate::ui;
use fltk::group::Group;



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
        
        ui::manu(&mut wind);

        wind.end();
        wind.show();
        app.run().unwrap();
    }
    pub fn update(corect_screen: &mut Group, new_screen: &mut Group) {
        corect_screen.hide();
        new_screen.show();
    }
}