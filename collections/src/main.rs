#[derive(Debug)]

enum SpreadsheetCell {
    Int(i32),
    Float(f64),
    Text(String),
}


fn main() {

    let mut v = Vec::new();

    v.push(5);
    v.push(6);
    v.push(7);
    v.push(8);

    println!("Vector v is {:?}",v);

    let row = vec![
    SpreadsheetCell::Int(3),
    SpreadsheetCell::Text(String::from("blue")),
    SpreadsheetCell::Float(10.12),
    ];  

    println!("Row of Enums: {:?}", row);

    reading_vector();
    push_into_vector();
    itterate();
    itterate_add();
    intro_strings();

}

fn reading_vector() {
    let v = vec![1, 2, 3, 4, 5];

    let third: &i32 = &v[2];
    println!("The third element is {}", third);
    println!("The third element is {}", &v[2]);

    // let does_not_exist = &v[100];
    let does_not_exist = v.get(100);



    match v.get(2) {
        Some(third) => println!("The third element is {}", third),
        None => println!("There is no third element."),
    }
}

fn push_into_vector(){
    let mut v = vec![1, 2, 3, 4, 5];

    //let first = &v[0];
    let first = v[0]; //this fixes the reference borrow. Now it compiles


    v.push(6);  //error because it borrows the reference &v at first as immutable

    println!("The first element is: {}", first);
}

fn itterate(){
    let v = vec![100, 32, 57];
    for i in &v {
        println!("{}", i);
    }
}

fn itterate_add(){
    let mut v = vec![11,22,33];
    for i in &mut v {
        *i += 50;
        println!("{}", i);
    }
}

fn intro_strings(){
    let mut s = String::new();

    let data = "initial contents";

    let s = data.to_string();

    // the method also works on a literal directly:
    let s = "initial contents".to_string();

    let s = String::from("initial contents");

    println!("String s:{}",s);

    let hello = String::from("Dobrý den");
    let hello = String::from("Hello");
    let hello = String::from("שָׁלוֹם");
    let hello = String::from("नमस्ते");
    let hello = String::from("こんにちは");
    let hello = String::from("안녕하세요");
    let hello = String::from("你好");
    let hello = String::from("Olá");
    let hello = String::from("Здравствуйте");
    let hello = String::from("Hola");
    let hello = String::from("السلام عليكم");

    println!("String hello:{}",hello);

    append_strings();
}

fn append_strings(){
    let mut s = String::from("foo");
    s.push_str("bar");

    println!("Appended string :{}",s);

    let mut s1 = String::from("foo");
    let s2 = "bar";
    s1.push_str(s2);
    println!("s2 is {}", s2);

    let mut s = String::from("lo");
    s.push('l');

    let s1 = String::from("Hello, ");
    let s2 = String::from("world!");
    let s3 = s1 + &s2; //s1 no longer exists after addition because its moved into s3

    println!("s3 is {}", s3);

    let s1 = String::from("tic");
    let s2 = String::from("tac");
    let s3 = String::from("toe");

    let s = format!("{}-{}-{}", s1, s2, s3);

    println!("s is {}", s);

    //UTF-8 vaules are not always 1 byte
    let hello = "Здравствуйте";
    let s = &hello[0..4];

    println!("s is {}", s);

    //Return chars
    for c in "नमस्ते".chars() {
        println!("{}", c);
    }

    //Return bytes
    for b in "नमस्ते".bytes() {
        println!("{}", b);
    }

}