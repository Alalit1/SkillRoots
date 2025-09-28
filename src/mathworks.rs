//use rand::prelude::*;
use std::time::Instant;

pub fn times(time_end: u8){
    /* функція з відліком часу*/
    let start = Instant::now();
    println!("Початок...");

    loop {
        let elapsed = start.elapsed().as_secs();
        println!("Минуло: {} сек", elapsed);

        if elapsed >= time_end.into() {
            println!("Завершення!");
            break;
        }
    }
}

pub fn ganerate_tasks(difficulty: u8){
        /* Функція створення прикладів*/
        // генеруеться складність з таких  увмов як складність завдань та теми завдань
        // це робити по шаблонам які будуть створувати приклади на основі тем

         // створюємо генератор
        //let mut rng = rand::thread_rng();

        let actions = match difficulty.into() {
            1 => println!("1"),
            2 => println!("2"),
            3 => println!("3"),
            _ => println!("0"), // запасний варіант
        };

    }

/*struct Math{
    taimer: bool;
    difficultylevels: u8;
    tema: str;
    // qqq
}
impl Math{
    pub fn new(self){
        /*функція*/
    }

    
    

}*/