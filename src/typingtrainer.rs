use crossterm::event::{read, Event, KeyCode, KeyEventKind};
use crossterm::terminal::enable_raw_mode;

//1 генерацихї 21 соберая наш вод 3 проек=рука
//ganerate task
pub struct TypingTrainer{
    pub connectivity: bool,
    pub difficulty: u8, //difficulty level
    pub punctuationmarks: bool, //punctuation marks
}

impl TypingTrainer{
    pub fn new(connectivity: bool, difficulty: u8, punctuationmarks:bool) -> Self {
        Self{
            connectivity,
            difficulty,
            punctuationmarks
        }
    }

    //ganarate
    pub fn ganarate(&mut self){
        // TODO: implemen
    }
    //chaked
    pub fn cheked(&mut self,input_char: char){
        let task_cahr = 'y';
        if input_char == task_cahr{
            println!("hello");
        }
    }
    
    //klicked 
    pub fn klicked(&mut self){
    enable_raw_mode().unwrap();
    println!("Пиши що завгодно (Esc для виходу)");

    loop {
        if let Event::Key(event) = read().unwrap() {
            // 🔴 ФІЛЬТР
            if event.kind != KeyEventKind::Press {
                continue;
            }

            match event.code {
                KeyCode::Char(c) => {
                    self.cheked(c);
                    println!("Символ: '{}'", c);
                }
                KeyCode::Enter => println!("ENTER"),
                KeyCode::Esc => break,
                _ => {}
            }
        }
    }
}
}
