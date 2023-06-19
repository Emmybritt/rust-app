#![deny(clippy::all)]

const MY_AGE: u8 = 22;

// fn process_name(name: &str, caller: fn(&str) -> ()) {
//     caller(name)
// }

#[derive(Debug)]
struct Person {
    name: String,
    age: u8,
}

#[derive(Debug)]
struct Point(f64, f64, f64);
struct Size {
    height: f32,
    width: f32,
}

// fn create_person(name: String, age: u8) -> Person {
//     let newPerson: Person = Person { name, age };
//     newPerson
// }
enum AnimalType {
    Dog,
    Goat,
    Cat,
    Rabbit,
}
enum Shapes {
    Circle2 { radius: f64, center: (f64, f64) },
    Circle(f32, f32, f32),
    Rectangle2 { width: f32, height: f32 },
    Rectangle(f32, f32, Size),
}

impl Shapes {
    fn calculate_area(&self) -> f32 {
        match self {
            Shapes::Rectangle(x, y, size) => size.width * size.height,
            Shapes::Circle(x, y, radius) => 3.14 * radius * radius,
            _ => 0.0,
        }
    }
}
impl Point {
    fn make_twice(&mut self) {
        self.0 *= 2.0;
        self.1 *= 2.0;
        self.2 *= 2.0;
    }
    fn twice(&self) -> Point {
        Point(self.0 * 2.0, self.1 * 2.0, self.2 * 2.0)
    }
    fn describe(&self) {
        println!("Point is at ({}, {}, {})", self.0, self.1, self.2)
    }

    fn zero() -> Point {
        Point(0.0, 0.0, 0.0)
    }
}

fn main() {
    let rectangle2: Shapes = Shapes::Rectangle2 {
        width: 3.0,
        height: 4.0,
    };
    if let Shapes::Rectangle2 { width, height } = rectangle2 {
        println!("Width={} height={}", width, height);
    }
    let rectangle = Shapes::Rectangle(
        1.5,
        2.4,
        Size {
            height: 3.2,
            width: 2.9,
        },
    );
    rectangle.calculate_area();
    // let area = Shapes

    match rectangle2 {
        Shapes::Rectangle2 { width, height } => println!("width = {} height = {}", width, height),
        _ => println!("Not a rectangle"),
    }
    let mut point = Point(0.0, 5.9, 3.0);
    point.describe();
    point.twice();
    point.make_twice();
    Point::zero();
    println!("x ={} y = {} z= {}", point.0, point.1, point.2);
    let person = Person {
        age: 30,
        name: "John".to_string(),
    };

    println!(
        "My name is {} and i am {} years old",
        person.name, person.age
    );
    let say_hello_to = |name: &str| format!("Hellotes, {}", name);

    println!("{}", say_hello_to("Deborah"));
    let full_name = |first_name: &str, last_name: &str| format!("{} {}", first_name, last_name);
    let multiply_by_2 = |x: i32| x * 2;
    let ask_for_age = || {
        // Ask the user for age
        // Calculate how old in 10 years
        10
    };
    let ptr = multiply_by_2;
    let result = ptr(10);
    println!("this is the result {}", result);
    println!("{}", ask_for_age());
    println!("{}", multiply_by_2(4));
    println!("{}", full_name("Victory", "Ajiri"));
    let mut name: &str = "Emmy britt";
    let age: i32 = 30;
    let personal_data = (22, "john", true, 33.5);
    let (new_age, new_name, is_studentt, percentage) = personal_data;
    // let rgb: i32 = 0xFF0000;]
    // let distance_in_km = 5.5;
    let distance1: f64 = 5.5;
    let distance2: f64 = 6.2;
    let distance3: f64 = 10.2;
    let total_distance = distance1 + distance2 + distance3;
    {
        let total_distance: String = total_distance.to_string();
        println!(
            "My name is {:?} and i am {} years old the distance is {}",
            name, MY_AGE, total_distance
        )
    }
    println!("Your name is {:?}!", name);
    name = "Daniel";
    println!(
        "My new name is {:?} and i am {} years old the distance is {}",
        name, age, total_distance
    );
    println!(
        "My new name is {:?} and i am {} {} years old the distance is {}",
        new_name, new_age, is_studentt, percentage
    );
    let hello = say_hello_world(name);
    println!("{}", hello);
    owner_ship();
}

fn owner_ship() {
    {
        let mut name = String::from("John");
        let name2: &mut String = &mut name;
        println!("Hello {}", name2);
    }
}

fn say_hello_world(to_person: &str) -> String {
    println!("I am working");
    format!("Hello3, {}!", to_person)
}

fn match_imp() {
    let fluppy: AnimalType = AnimalType::Cat;
    match fluppy {
        AnimalType::Cat => println!("This is a cat"),
        AnimalType::Goat => println!("This is a Goat"),
        AnimalType::Dog => println!("This is a Dog"),
        AnimalType::Rabbit => println!("This is a Rabbit"),
    }
}
