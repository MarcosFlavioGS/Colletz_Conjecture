use std::io;

fn main() {
    let mut _counter: u64 = 0;
    let mut input = String::new();

    println!("Type a number for the test: ");

    io::stdin()
        .read_line(&mut input)
        .expect("Failed to read line");

    let mut input: u64 = input.trim().parse().expect("Please pass a number");

    println!("Number chosen: {}", input);

    while input > 1 {
        if input % 2 == 0 {
            println!("Even. Dividing {} by 2...", input);
            input = input / 2;
            println!("{}", input);
            _counter += 1;
        } else {
            println!("Odd. {} * 3 + 1...", input);
            input = input * 3 + 1;
            println!("{}", input);
            _counter += 1;
        }
    }
    println!("Total of tests made: {}", _counter);
}
