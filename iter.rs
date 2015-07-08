fn main() {
    let mut iter = 1..10;

    loop {
        match iter.next() {
            Some(x) => println!("{}", x),
            None => break
        }
    }

    println!("The End!");

}
