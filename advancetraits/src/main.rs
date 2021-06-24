
use std::ops::Add;

pub trait Iterator {
    type Item;  //placeholder type. Implementors will specify the return type

    fn next(&mut self) -> Option<Self::Item>;
}

#[derive(Debug, PartialEq)]
struct Point {
    x: i32,
    y: i32,
}

struct Millimeters(u32);
struct Meters(u32);

impl Add<Meters> for Millimeters {
    type Output = Millimeters;

    fn add(self, other: Meters) -> Millimeters {
        Millimeters(self.0 + (other.0 * 1000))
    }
}

impl Add for Point {
    type Output = Point;  //associative type returned

    fn add(self, other: Point) -> Point {
        Point {
            x: self.x + other.x,
            y: self.y + other.y,
        }
    }
}


trait Pilot {
    fn fly(&self);
}

trait Wizard {
    fn fly(&self);
}

struct Human;

impl Pilot for Human {
    fn fly(&self) {
        println!("This is your captain speaking.");
    }
}

impl Wizard for Human {
    fn fly(&self) {
        println!("Up!");
    }
}

impl Human {
    fn fly(&self) {
        println!("*waving arms furiously*");
    }
}

trait Animal {
    fn baby_name() -> String;  //associative function, not a method since it doesnt have a self
}

struct Dog;

impl Dog {
    fn baby_name() -> String {
        String::from("Spot")
    }
}

impl Animal for Dog {
    fn baby_name() -> String {
        String::from("puppy")
    }
}


fn main() {
    assert_eq!(
        Point { x: 1, y: 0 } + Point { x: 2, y: 3 },
        Point { x: 3, y: 3 }
    );


    let person = Human;
    person.fly();  // prints println!("*waving arms furiously*");
    Pilot::fly(&person);
    Wizard::fly(&person);

    println!("A baby dog is called a {}", Dog::baby_name());
    // println!("A baby dog is called a {}", Animal::baby_name()); error
    println!("A baby dog is called a {}", <Dog as Animal>::baby_name()); //return puppy

    // syntax <Type as Trait>::function(receiver_if_method, next_arg, ...);




}
