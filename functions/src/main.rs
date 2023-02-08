fn main() {
    let result = double(2);
    println!("{}", result);
}

fn double(num: i32) -> i32{
    num * 2
}