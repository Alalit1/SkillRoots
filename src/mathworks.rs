use rand::prelude::*;
use std::{thread, time};

struct Math{
    taimer: bool;
    difficultylevels: u8;
    tema: str;
    // qqq
}
impl Math{
    pub fn new(self){
        /*функція*/
    }

    pub fn time(self){
        /* функція з відліком часу*/
        let start = time::Instant::now();
        println!("Початок...");
        thread::sleep(time::Duration::from_secs(2)); // пауза 2 сек
        println!("Минуло {:?}", start.elapsed());
    }
    pub fn ganerate_tasks(self){
        /* Функція створення прикладів*/
        // генеруеться складність з таких  увмов як складність завдань та теми завдань
        // це робити по шаблонам які будуть створувати приклади на основі тем

         // створюємо генератор
        let mut rng = rand::thread_rng();

        let actions = match self.difficultylevels {
            1 => rng.gen_range(1..5),
            2 => rng.gen_range(2..7),
            3 => rng.gen_range(4..9),
            _ => 0, // запасний варіант
        };
        let n: i32 = rng.gen_range(0..10);
        println!("Випадкове число: {}", n);

        // випадкове число з плаваючою крапкою [0,1)
        let x: f64 = rng.gen();
        println!("Випадкове число (float): {}", x);
        let 

    }

}