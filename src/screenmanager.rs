#[derive(Copy, Clone, Debug)]
pub enum Action {
    Start,
    Settings,
    Exit,
    Back,
    Mash,
    Typing,
    Language,
}

// Структура для керування станом програми
pub struct AppState {
    current_window: WindowType,
    main_window: *mut fltk::window::Window,
}

#[derive(Clone, Copy, PartialEq)]
pub enum WindowType {
    MainMenu,
    Settings,
    Mash,
    Typing,
    Language,
}

impl AppState {
    pub fn new(main_window: *mut fltk::window::Window) -> Self {
        Self {
            current_window: WindowType::MainMenu,
            main_window,
        }
    }
    
    pub fn handle_action(&mut self, action: Action) {
        match action {
            Action::Start => {
                println!("Запуск гри...");
            },
            Action::Settings => {
                self.show_window(WindowType::Settings);
            },
            Action::Mash => {
                self.show_window(WindowType::Mash);
            },
            Action::Typing => {
                self.show_window(WindowType::Typing);
            },
            Action::Language => {
                self.show_window(WindowType::Language);
            },
            Action::Back => {
                self.show_window(WindowType::MainMenu);
            },
            Action::Exit => {
                println!("Вихід з програми");
                // Закриваємо головне вікно
                unsafe {
                    if !self.main_window.is_null() {
                        (*self.main_window).hide();
                    }
                }
            },
        }
    }
    
    fn show_window(&mut self, window_type: WindowType) {
        self.current_window = window_type;
        self.redraw_current_window();
    }
    
    pub fn redraw_current_window(&mut self) {
        // 1. Очищаємо старий вміст
        self.content_group.clear();
        
        // 2. Створюємо новий вміст залежно від поточного вікна
        match self.current_window {
            WindowType::MainMenu => {
                self.build_main_menu();
            }
            WindowType::Settings => {
                self.build_settings();
            }
            WindowType::Mash => {
                self.build_mash();
            }
            WindowType::Typing => {
                self.build_typing();
            }
            WindowType::Language => {
                self.build_language();
            }
        }
    }
}