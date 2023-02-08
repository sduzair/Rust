fn main() {
    println!("Loops return example");

    let mut counter = 0;
    let result = loop {
        counter += 1;
        if counter == 10 {
            break counter
        }
    };

    println!("{}", result);
}
