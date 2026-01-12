use fltk::{prelude::*, *};
use std::rc::Rc;
use std::cell::RefCell;

fn main() {
    let app = app::App::default();
    let mut main_win = window::Window::new(100, 100, 400, 300, "Моя програма");
    
    // Створюємо стан програми
    let app_state = Rc::new(RefCell::new(AppState::new(&mut main_win)));
    
    // Створюємо колбек для обробки дій
    let on_action: Rc<RefCell<dyn FnMut(Action)>> = Rc::new(RefCell::new({
        let app_state = app_state.clone();
        move |action| {
            app_state.borrow_mut().handle_action(action);
        }
    }));
    
    // Створюємо контейнер для поточного вікна
    let mut current_group: Option<Group> = None;
    
    // Функція для зміни вікна
    let show_window = move |window_type: WindowType| {
        // Видаляємо поточне вікно
        if let Some(mut group) = current_group.take() {
            group.clear();
        }
        
        // Створюємо нове вікно
        let group = match window_type {
            WindowType::MainMenu => build_main_menu(on_action.clone()),
            WindowType::Settings => build_settings(on_action.clone()),
            WindowType::Mash => build_mash(on_action.clone()),
            WindowType::Typing => build_typing(on_action.clone()),
            WindowType::Language => build_language(on_action.clone()),
        };
        
        current_group = Some(group);
        main_win.redraw();
    };
    
    // Показуємо головне меню
    show_window(WindowType::MainMenu);
    
    main_win.end();
    main_win.show();
    
    app.run().unwrap();
}