fn simple_iter() {
    let mut iter = 1..10;

    loop {
        match iter.next() {
            Some(x) => println!("{}", x),
            None => break
        }
    }

    println!("The End!");

}

fn simple_consumer() {
    let cons = (1..10).collect::<Vec<u32>>();

    for i in cons {
        println!("{}", i);
    }
}

fn find_in_consumers() {
    let cons = (1..1001).find(|x| *x > 22);

    for i in cons {
        println!("{}", i);
    }
}

fn filter_in_consumers() {
    let cons = (1..1001).filter(|x| *x % 2 == 0);

    for i in cons {
        println!("{}", i);
    }
}

fn main() {
    filter_in_consumers();
}
