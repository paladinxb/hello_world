use std::thread;
use std::sync::{Mutex, Arc};

struct Philosopher {
    name: String,
    left: usize,
    right: usize,
}

impl Philosopher {
    fn new(name: &str, left: usize, right: usize) -> Philosopher {
        Philosopher{
            name: name.to_string(),
            left: left,
            right: right,
        }
    }

    fn eat(&self, table: &Table){
        let _left = table.forks[self.left].lock().unwrap();
        let _right = table.forks[self.right].lock().unwrap();
        println!("{} начала есть.", self.name);

        thread::sleep_ms(1000);

        println!("{} закончила есть.", self.name);
    }
}

struct Table {
    forks: Vec<Mutex<()>>,
}

fn main(){
    let table =Arc::new(Table { forks: vec! [
        Mutex::new(()),
        Mutex::new(()),
        Mutex::new(()),
        Mutex::new(()),
        Mutex::new(()),
    ]});

    let philosofers = vec![
        Philosopher::new("Джудит Батлер", 0, 1),
        Philosopher::new("Рая Дунаевская", 1, 2),
        Philosopher::new("Зарубина Наталья", 2, 3),
        Philosopher::new("Эмма Гольдман", 3, 4),
        Philosopher::new("Анна Шмидт", 0, 4),
    ];

    let hadnles: Vec<_> = philosofers.into_iter().map(|p| {

        let table = table.clone();

        thread::spawn(move || {
            p.eat(&table);
        })
    }).collect();

    for h in hadnles {
        h.join().unwrap();
    }
}
