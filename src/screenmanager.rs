use fltk::prelude::*; // містить WidgetBase, який потрібен для new()
use fltk::{app::App, window::Window, image::PngImage};
use crate::ui::build_main_menu;
use crate::ui::Action;
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

        // Виклик функції з ui.rs
        let menu = build_main_menu(|action| {
            match action {
                Action::Start => {
                    println!("Start game");
                    // screen_manager.show(Game)
                }
                Action::Settings => {
                    println!("Open settings");
                    // screen_manager.show(Settings)
                }
                Action::Exit => {
                    std::process::exit(0);
                }
            }
        });
        wind.add(&menu);

        //ui::build_main_menu(&mut wind);

        wind.end();
        wind.show();
        app.run().unwrap();
    }
}