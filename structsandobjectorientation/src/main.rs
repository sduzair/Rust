fn main() {
    let person1 = Person::create("name", 32);
    if person1.can_have_icecream() == true {
        println!("Can have icecream!");
    } else {
        println!("Cannot have icecream!");
    }

    println!("Hello, world!");
}

// *struct similar to class fields
struct Person {
    name: String,
    age: i32,
    is_adult: bool,
}

// *impl of a struct/type similar to class behaviour/methods
impl Person {
    fn create(name: &str, age: i32) -> Self{
        Self { name: String::from(name), age: age, is_adult: false }
    }

    //* no overloading/polymorphism?? (ans is yes see below)
    // fn create(name: &str, age: i32, is_adult: bool) -> Self{
    //     Self { name: String::from(name), age: age, is_adult: is_adult }
    // }

    fn can_have_icecream(&self) -> bool {
        self.is_adult
    }
}

// *trait similar to Interface
trait SeniorSpec {
    fn is_senior(&self) -> bool;
}

impl SeniorSpec for Person {
    fn is_senior(&self) -> bool {
        self.age > 60
    }
}

// *all properties and methods can be accessed
fn person_details(person: Person) {
    println!("Name: {}", person.name);
    println!("Age: {}", person.age);
    println!("Is Adult?: {}", person.is_adult);


    if person.is_senior() {
        println!("Person is senior")
    }
    
}

// *only the methods declared in 'trait' can be accessed
fn person_senior(person: &impl SeniorSpec) {
    let a = person.is_senior();
}

// *OBSERVATIONS 1/N
// *rust uses composition of types(structs) not inheritance as in OOPS
// *rust uses traits to share functionality across types

struct SpecialPerson {
    name: String,
    age: i32,
    is_adult: bool,
    special_prop: String
}

impl SeniorSpec for SpecialPerson {
    fn is_senior(&self) -> bool {
        true
    }
}

// *OBSERVATIONS 2/N
// *rust allows polymorphism to be implemented through traits

fn polymorphism() {
    let person1: Person = Person::create("name", 34);
    let person2: SpecialPerson = SpecialPerson { name: "special".to_string(), age: 34, is_adult: true, special_prop: "deaf".to_string() };

    // person1.is_senior()

    // person2.is_senior()
}