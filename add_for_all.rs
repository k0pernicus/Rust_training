use std::env;

fn add(x: &i32, y: &i32) -> i32 {
    *x + *y
}

fn main() {
    let table: Vec<i32> = vec![1,2,3,4,5];
    let args: Vec<_> = env::args().collect();
    let ref arg = args[1].parse::<i32>().unwrap();
    for acc in &table{
        println!("{} became {}", acc, add(&acc, &arg));
    }
}
