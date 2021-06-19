fn main() {

    let mut v = Vec::new();

    v.push(5);
    v.push(6);
    v.push(7);
    v.push(8);

    println!("Vector v is {:?}",v);

    reading_vector();

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