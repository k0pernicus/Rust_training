use std::thread;
use std::sync::Mutex;
use std::sync::Arc;

struct Table {
    forks: Vec<Mutex<()>>,
}

//A philosopher is a person which have a name
struct Philosopher {
    name: String,
    left_hand: usize,
    right_hand: usize,
}

impl Philosopher {

    fn new(name: &str, left: usize, right: usize) -> Philosopher {
        Philosopher {
            name: name.to_string(),
            left_hand: left,
            right_hand: right,
        }
    }

    fn eat(&self, table: &Table) {

        let _left = table.forks[self.left_hand].lock().unwrap();
        let _right = table.forks[self.right_hand].lock().unwrap();

        println!("{} is going to eat", self.name);

        thread::sleep_ms(1000);

        println!("{} is done eating.", self.name);
    }

}

fn main() {

    let table = Arc::new(Table { forks: vec![
        Mutex::new(()),
        Mutex::new(()),
        Mutex::new(()),
        Mutex::new(()),
        Mutex::new(()),
    ]});

    let philosophers: Vec<Philosopher> = vec![
        Philosopher::new("Alexandre", 0, 1),
        Philosopher::new("Antonin", 1, 2),
        Philosopher::new("Elliot", 2, 3),
        Philosopher::new("Quentin", 3, 4),
        Philosopher::new("Gaetan", 4, 0),
    ];

    let handles: Vec<_> = philosophers.into_iter().map(|p| {
        let table = table.clone();

        thread::spawn(move || {
            p.eat(&table);
            })
        }).collect();

    for h in handles {
        h.join().unwrap();
    }

}
