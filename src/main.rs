use std::io;

fn get_list(mut input: usize) -> (Vec<usize>, u64) {
    let mut lst: Vec<usize> = Vec::new();
    let mut _counter: u64 = 0;

    while input > 1 {
        if input % 2 == 0 {
            input = input / 2;
            lst.push(input);
            _counter += 1;
        } else {
            input = input * 3 + 1;
            lst.push(input);
            _counter += 1;
        }
    }

    (lst, _counter)
}

fn main() {
    let mut input = String::new();

    println!("Type a number for the test: ");

    io::stdin()
        .read_line(&mut input)
        .expect("Failed to read line");

    let input: usize = input.trim().parse().expect("Please pass a number");

    println!("Number chosen: {}", input);

    let (lst, _counter) = get_list(input);

    lst.iter().for_each(|num| {
        println!("{}", num);
    })

}
