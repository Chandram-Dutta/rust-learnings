fn main() {
    let tup1 = (12, "Hello World", 34.56);
    println!("{} {} {}", tup1.0, tup1.1, tup1.2);
    let (x, y, z) = tup1;
    println!("{} {} {}", x, y, z);
}
