fn main() {
    let mut name1 = String::from("Chandram");
    mutable_ref(&mut name1);

    let name2 = name1;
    let name3 = &name2;
    // println!("{}", name1);
    // Won't work as ownership of name1 is "moved" to name2 since String is stored in Heap.
    println!("{}", name2);
    println!("{}", name3); // works as name3 is referencing to name2. It's read-only.

    // mutable_ref(&mut name2); Won't work as we cannot have a mutable and a non mutable refernce at the same time.
    not_mutable_ref(name3);

    let age1 = 4;
    let age2 = 10;
    println!("{}", age1); //Works as age1 is copied to age2 since int is stored in Stack
    println!("{}", age2);
}

fn mutable_ref(name: &mut String) {
    name.push_str(" You are Awesome");
    println!("{}", name);
}
fn not_mutable_ref(name: &String) {
    // let hello_name = name.push_str(" You are Awesome");
    // won't work as the reference is not mutable
    println!("{}, You are Awesome", name);
}
