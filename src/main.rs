mod screenmanager;
use screenmanager::ScreenManager;

fn main() {
    let scr = ScreenManager::new(400, 300);
    scr.draw();
}
