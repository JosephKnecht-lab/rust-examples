fn main() {

    const MAX_POINTS: u32 = 100_000;

    println!("The value of MAX_POINTS is: {}", MAX_POINTS);

    let mut x = 5;
    println!("The value of x is: {}", x);
    x = 6;
    println!("The value of x is: {}", x);

    shadowing();
    spaces();
    chars();
    tupple();
    expression();
    five();

    let z = plus_one(4);

    println!("the value of z is: {}", z);
}

fn shadowing(){
    let x = 5;

    let x = x + 1;

    let x = x * 2;

    println!("The value of x is: {}", x); 
}

fn spaces(){
    let spaces = "   ";
    let spaces = spaces.len();

    println!("The value of spaces is: {}", spaces); 

}

fn chars (){
    let c = 'z';
    let z = 'ℤ';
    let heart_eyed_cat = '😻';

    println!("The value of heart_eyed_cat is: {}", heart_eyed_cat); 

}

fn tupple() {
    let tup = (500, 6.4, 1);

    let (x, y, z) = tup;

    println!("The value of y is: {}", y);
}

fn expression(){
    let x = 5;

    let y = {
        let x = 3;
        x + 1
    };

    println!("The value of y is: {}", y);
}

fn five() -> i32 {
    5
}

fn plus_one(x: i32) -> i32 {
    x + 1 
}