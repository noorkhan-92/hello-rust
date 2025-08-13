use std::collections::HashMap;

fn main() {
    let name = "Noor";
    let mut age = 32;
    const AGE_GROUP: i32 = 1990;
    let gen_z = age >= 30 && age <= 40;
    println!("Hello, I am {} and my age is {}.", name, age);
    // after 1 year
    age = 33;
    println!("Hello, I am {} and my age is {}.", name, age);

    let group = if gen_z {
        format!("{} is genZ", name)
    } else {
        format!("{} is not genZ", name)
    };
    println!("{:?}", group);

    let day = 3;
    match day {
        1 => println!("Monday"),
        2 => println!("Tuesday"),
        3 => println!("Wednesday"),
        4 => println!("Thursday"),
        5 => println!("Friday"),
        6 => println!("Saturday"),
        7 => println!("Sunday"),
        _ => println!("Invalid Day Value"),
    }

    let mut count = 1;
    loop {
        println!("Count is {}", count);
        if count == 3 {
            break;
        }
        count += 1;
    }
    println!("sum is {}", add(2, 3));

    // Array
    let numbers = [1, 2, 3, 33, 22, 11];
    println!("{:?}", numbers);
    for num in numbers {
        println!("{}", num);
    }

    // Vector
    let mut cars = vec!["Honda", "Toyota", "BMW"];
    cars.push("Tesla");
    println!("{}", cars[3]);
    cars.pop();

    // Tuple
    let person = ("Noor", 32, "Male", true);
    println!("{}", person.0);
    let (name, age, gender, active) = person;
    println!("name: {}, age: {}, gender: {}, active: {}", name, age, gender, active);
    println!("{:?}", getTuple());

    // HashMap
    let mut capitalCities = HashMap::new();
    capitalCities.insert("Pakistan", "Islamabad");
    capitalCities.insert("Afghanistan", "Kabul");
    capitalCities.insert("India", "Delhi");
    println!("{:?}", capitalCities);

    // struct
    let p = Person {name: "Noor".to_string(), age: 32, is_married: true};
    println!("{}", p.name);

    // enum
    let south = Direction::SOUTH(String::from("distance in km covered in south"), 100);
    println!("{:?}", south);

    println!("{}", power(3, 3));
    println!("{}", recursiveCall(3));
}

fn add(a: i32, b: i32) -> i32 {
    a + b // also return a + b; work
}

fn getTuple() -> (String, i32) {
    (String::from("Noor"), 32)
}

// struct
struct Person {
    name: String,
    age: i32,
    is_married: bool,
}

// enum
#[derive(Debug)]
enum Direction {
    NORTH(String, i32),
    SOUTH(String, i32),
    EAST(String, i32),
    WEST(String, i32),
}

fn power(x: i32, n: i32) -> i32 {
    let mut y = x;
    for _i in 1..n {
        y *= x;
    }
    y
}

fn recursiveCall(mut a: i32) -> bool {
    if a > 100 {
        return true;
    }
    a *= 2;
    recursiveCall(a)
}
