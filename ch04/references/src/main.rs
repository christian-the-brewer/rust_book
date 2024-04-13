fn main() {
    let mut v: Vec<i32> = vec![1,2,3];

    let num: &mut i32 = &mut v[2];

    *num += 1; 

    println!("Third element in v is {num}");

    println!("Vector is now {:?}", v);
}
