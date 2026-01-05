mod screenmanager;
use screenmanager::ScreenManager;

//mod mathworks;
mod graphics;
mod typingtrainer;
mod vidgets;
mod ui;

fn main() {
    
    let scr = ScreenManager::new(400, 300);
    scr.draw();
    // роботает 
    let mut test = typingtrainer::TypingTrainer::new(true, 3, true);
    //test::ganerate_tasks();
    test.klicked();
}


