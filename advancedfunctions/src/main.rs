fn add_one(x: i32) -> i32 {
    x + 1
}

fn do_twice(f: fn(i32) -> i32, arg: i32) -> i32 {  //use fn to pass a function as arg
    f(arg) + f(arg)
}

enum Status {
    Value(u32),
    Stop,
}

fn main() {
    let answer = do_twice(add_one, 5);

    println!("The answer is: {}", answer);

    let list_of_numbers = vec![1, 2, 3];
    let list_of_strings: Vec<String> =
        list_of_numbers.iter().map(|i| i.to_string()).collect();

    let list_of_statuses: Vec<Status> = (0u32..20).map(Status::Value).collect();

}

fn returns_closure() -> Box<dyn Fn(i32) -> i32> {  //cant return a closure directly
    Box::new(|x| x + 1)  //must be wrapped in a Box
}
