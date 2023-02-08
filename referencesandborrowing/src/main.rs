fn main() {
    // To not give up ownership of the value that does not have the 'Copy' 'trait' we can
    
    // 1. Reference the value
    let s1 = String::from("Hello");
    val_ref(&s1);
    println!("{}", s1);

    // 2. Borrow the value
    let mut s2 = String::from("Hello");
    val_borrow(&mut s2);
    println!("{}", s2);
}

fn val_ref(word_ref: &String) {
    println!("{}", word_ref);
    // word_ref.push_str(" World"); // This will not work because we are not borrowing the value mutably
}

fn val_borrow(word_ref: &mut String) {
    word_ref.push_str(" World");
}