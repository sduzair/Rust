fn main() {
    let a = divide_by_two(0);
    // println!("{}", a.);

}

fn divide_by_two(num: i32) -> Option<i32> {
    if num == 0 {
        return None;
    }
    Some(num / 2)
}
