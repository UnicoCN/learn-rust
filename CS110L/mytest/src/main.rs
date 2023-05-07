fn main() {
    let x = 1;
    let p: *const i32 = &x;
    println!("{:?}", p);
}
