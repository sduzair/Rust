fn main() {
    let a = divide_by_two(4);
    // println!("{}", a.);
}

fn divide_by_two(num: i32) -> Option<i32> {
    let item2 = Item {
        width: 100,
        height: 100,
        tag: String::from("div"),
        content: String::from("Hello World"),
        style: Style {
            color: String::from("red"),
            background: String::from("blue"),
            border: String::from("1px solid black"),
        },
    };

    // let item = newItem(Item { ..item2 });
    
    // let item3 = newItem(Item { ..item2 });
    let item4 = newItem(Item { ..item2 });
    
    let item = newItem(Item {
        width: 100,
        height: 100,
        tag: String::from("div"),
        content: String::from("Hello World"),
        style: Style {
            color: String::from("red"),
            background: String::from("blue"),
            border: String::from("1px solid black"),
        },
    });
    if num == 0 {
        return None;
    }
    Some(num / 2)
}

// fn some_funcition(arg: usize) -> Result<some::long::type> {

// }

// newItem is a function that returns a new item
fn newItem(item: Item) -> Item {
    item
}

// Item is a struct
struct Item {
    width: i32,
    height: i32,
    tag: String,
    content: String,
    style: Style,
}

// Style is a struct
struct Style {
    color: String,
    background: String,
    border: String,
}
