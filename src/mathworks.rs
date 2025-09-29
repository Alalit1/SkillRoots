use rand::seq::IndexedRandom; // для choose_multiple
use rand::{rng, Rng};    
use std::time::Instant;
use std::io;

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
        
        let ecs = 5;
        let math_operations: [&str;6] = ["+","-","/","*","**","//"];
        
        let actions = match difficulty.into() {
            1 => 1..=2,
            2 => 1..=3,
            3 => 1..=4,
            _ => 1..=5, // запасний варіант
        };

        println!("{:?}",actions);
        println!("Завдань буде надано {}",ecs);

        let mut rng = rng(); // замість thread_rng()
        
        // 5 випадкових варіантів
        for task_id  in 1..ecs {
            // випадкова кількість операцій із діапазону
            let n_ops: usize = rng.random_range(actions.clone());
            
            // вибрати n різних операцій
            let random_ops: Vec<_> = math_operations
                .choose_multiple(&mut rng, n_ops)
                .cloned()
                .collect();

            // будуємо вираз (і одразу рахуємо його)
            let mut numbers: Vec<i32> = Vec::new();
            let first_num: i32 = rng.random_range(1..=20);
            numbers.push(first_num);

            let mut expr = first_num.to_string();

            for op in &random_ops {
                let num: i32 = rng.random_range(1..=20);
                numbers.push(num);
                expr.push_str(&format!(" {} {}", op, num));
            }

            // обчислюємо вираз (простий калькулятор зліва-направо без пріоритетів!)
            let mut result = numbers[0];
            for (i, op) in random_ops.iter().enumerate() {
                let b = numbers[i + 1];
                result = match *op {
                    "+" => result + b,
                    "-" => result - b,
                    "*" => result * b,
                    "/" => {
                        if b == 0 {
                            println!("Упс, ділення на нуль! Перескочимо це завдання.");
                            continue;
                        }
                        result / b
                    }
                    _ => result,
                };
            }

            println!("\nЗавдання {task_id}: {expr} = ?");

            // чекаємо відповідь від користувача
            let mut input = String::new();
            io::stdin().read_line(&mut input).unwrap();

            if let Ok(answer) = input.trim().parse::<i32>() {
                if answer == result {
                    println!("✅ Правильно!");
                } else {
                    println!("❌ Неправильно. Правильна відповідь: {result}");
                }
            } else {
                println!("⚠️ Ви ввели не число. Правильна відповідь: {result}");
            }
        }
        // генерувати які дії будуть виконуватись  беручи одне число з актіонс і стільки символв ми беем із списка дій
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