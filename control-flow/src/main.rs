fn main() {
    let age_limit = 18;
    let age = 19;

    if age < age_limit {
        println!("Underage")
    } else if age == age_limit {
        println!("Congrats on maturing")
    } else {
        println!("You are allowed")
    }

    let mut num = if true { 5 } else { 3 };
    println!("{}", num);

    loop {
        num += 1;
        if num > 10 {
            num = 5;
            break;
        }
        println!("{}", num);
    }

    while num < 10 {
        num += 1;
        println!("{}", num);
    }

    let a = [100, 16, 20, 46, 53];

    for element in a {
        println!("{}", element);
    }
    for element in 1..=5 {
        println!("{}", element)
    }
}
