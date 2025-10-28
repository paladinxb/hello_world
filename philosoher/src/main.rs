struct Philosopher {
    name: String,
}

impl Philosopher {
    fn new(name: &str) -> Philosopher {
        Philosopher{
            name: name.to_string(),
        }
    }

    fn eat(&self){
        println!("{} закончил есть.", self.name);
    }
}

fn main(){
    let philosofers = vec![
        Philosopher::new("Джудит Батлер"),
        Philosopher::new("Рая Дунаевская"),
        Philosopher::new("Зарубина Наталья"),
        Philosopher::new("Эмма Гольдман"),
        Philosopher::new("Анна Шмидт"),
    ];

    for p in &philosofers{
        p.eat();
    }
    
}
