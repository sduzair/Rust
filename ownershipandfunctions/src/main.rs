fn main() {
    // *Takes ownership of the variable - Example 1 of 2
    let my_string = String::from("hello");
    takes_ownership(my_string);
    // String type is allocated on the heap so it does not deep copy instead the pointer, length and capacity are copied
    // once copied the original variable is 'invalidated' so the value is 'moved' and the new variable is the 'owner'
    // !println!("{}", my_string);

    // *Takes ownership of the variable - Example 2 of 2
    let my_string1 = String::from("hello");
    let my_string2 = my_string1;
    // String type is allocated on the heap so it does not deep copy instead the pointer, length and capacity are copied from my_string1 to my_string2
    // once copied the original variable is 'invalidated' so the value is 'moved' and the new variable is the 'owner'
    // !println!("{}", my_string1);

    // *Why is the previous variable 'invalidated'?
    // When a variable goes out of scope the 'drop' method is called automatically which frees the memory that the variable was using on the heap. If two variables point to the same memory and both of them go out of scope then the memory will be freed twice which is a problem known as 'double free error'. To prevent this Rust invalidates the first variable so that it can no longer be used.

    // *Makes a copy of the variable
    let my_int = 5;
    makes_copy(my_int);
    // Int implements 'Copy' 'trait', that we can place on types that are stored on the stack, so the value is 'copied'
    // new 'owner' of my_int is makes_copy function so we can use it
    println!("{}", my_int);

    // *Returns a value and takes ownership of it
    let returned_string = gives_ownership();
    println!("{}", returned_string);

    // *Takes ownership of a variable and returns it
    let returned_string = takes_and_gives_back(returned_string);
    println!("{}", returned_string);

}

fn takes_ownership(some_string: String) {
    println!("{}", some_string);
    // some_string goes out of scope and 'drop' is called automatically
}

fn makes_copy(some_int: i32) {
    println!("{}", some_int);
    // some_int goes out of scope but nothing special happens
}

fn gives_ownership() -> String {
    let some_string = String::from("hello");
    some_string
    // some_string goes out of scope and 'drop' is called automatically
}

fn takes_and_gives_back(a_string: String) -> String {
    a_string
    // a_string goes out of scope and 'drop' is called automatically
}
