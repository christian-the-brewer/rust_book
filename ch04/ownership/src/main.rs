fn main() {
    let mut s = String::from("Hello");

    s.push_str(", world!");//appends literal to a String

    println!("{s}"); // prints contents of s
}
