fn is_even(n: i32) -> bool {
    n % 2 == 0
}

fn main() {
    let numbers: [i32; 10] = [12, 25, 30, 7, 9, 14, 45, 22, 33, 50];
    
    println!("Analyzing numbers:");
    for &num in &numbers {
        if num % 3 == 0 && num % 5 == 0 {
            println!("{}: FizzBuzz", num);
        } else if num % 3 == 0 {
            println!("{}: Fizz", num);
        } else if num % 5 == 0 {
            println!("{}: Buzz", num);
        } else if is_even(num) {
            println!("{}: Even", num);
        } else {
            println!("{}: Odd", num);
        }
    }
    
    let mut sum = 0;
    let mut i = 0;
    while i < numbers.len() {
        sum += numbers[i];
        i += 1;
    }
    println!("Sum of all numbers: {}", sum);
    
    let mut max = numbers[0];
    for &num in &numbers {
        if num > max {
            max = num;
        }
    }
    println!("Largest number: {}", max);
}
