fn main() {
    let mut x = 5;
    { // Create a scope for the first mutable borrow
        let y = &mut x;
        *y += 1;
    }
    { // Create a scope for the second mutable borrow
        let z = &mut x;
        *z += 1;
    }
    println!("x = {}", x); //x will be 7
}