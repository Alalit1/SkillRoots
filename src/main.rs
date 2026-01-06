mod screenmanager;
use screenmanager::ScreenManager;
mod graphics;
mod typingtrainer;
mod vidgets;
mod ui;

fn main() {
    let scr = ScreenManager::new(400, 300);
    scr.draw();
}


