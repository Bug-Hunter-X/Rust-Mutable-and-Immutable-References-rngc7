fn main() {
    let mut x = 5;
    let y = &mut x; // y is a mutable reference to x
    *y += 1;      // Modify x through y
    let z = &x;    // z is an immutable reference to x
    println!("x = {}", x); //This line will print 6
    println!("z = {}", *z); //This line will also print 6, because z points to the same memory location as x, and x's value is already updated to 6

    let mut a = 10;
    let b = &mut a;
    *b = 20; // Modify a through b
    let c = &a;
    println!("a = {}", a); //This line will print 20
    println!("c = {}", *c); // This line will also print 20
}
