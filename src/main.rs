//use screenmanager::ScreenManager;
//mod mathworks;
//mod ui;
mod typingtrainer;

fn main() {
    // роботает 
    let mut test = typingtrainer::TypingTrainer::new(true, 3, true);
    //test::ganerate_tasks();
    test.klicked();
    test.cheked('y');
}
/*let test = typingtrainer::TypingTrainer();
    test::ganerate_tasks();
    test::klicked();
    test::cheked();*/
    //let scr = ScreenManager::new(400, 300);
    //scr.draw();

