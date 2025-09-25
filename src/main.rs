mod screenmanager;
mod ui; 
use crate::screenmanager::ScreenManager; // імпортуємо структуру

fn main() {
    
    let scr = ScreenManager::new(400, 300);
    scr.draw();
    
}
