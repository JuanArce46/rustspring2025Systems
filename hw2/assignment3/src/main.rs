#[allow(unused_variables, unused_mut)]
fn sum(total: &mut i32, low: i32, high: i32) {
    for i in low..=high {  // Iterate from low to high (inclusive)
        *total += i; // Dereference total to modify its value
    }
}

fn main() {
    let mut total = 0; // Create the total variable with initial value 0
    let low = 0;
    let high = 100;
    
    sum(&mut total, low, high); // Call the sum function with mutable reference to total
    
    println!("Total sum from {} to {} is: {}", low, high, total); // Should print: Total sum from 0 to 100 is: 5050
}