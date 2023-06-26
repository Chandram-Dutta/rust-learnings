fn main() {
    println!("Hello, world!");
    print_my_name();
    print_my_age(19);
    println!("1 + 1 = {}", one_plus_one());
}

fn print_my_name() {
    println!("Hello, Chandram!");
}

fn print_my_age(age: usize) {
    println!("You are {}", age);
}

fn one_plus_one() -> usize {
    1 + 1
}
