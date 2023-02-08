fn main() {
    println!("Hello, world!");

    let array = [10 ; 5];

    for num in array.iter() {
        println!("{}",num);
    }

    // print numbers in the range exclusive of the last number
    for num in 1..6 {
        println!("{}", num);
    }
}
