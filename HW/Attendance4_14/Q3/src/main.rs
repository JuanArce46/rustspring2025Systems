fn process_vector<F>(vec: Vec<i32>, f: F) -> Vec<i32>
where
    F: Fn(i32) -> i32,
{
    let mut result = Vec::new();
    for val in vec {
        result.push(f(val));
    }
    result
}


fn main() {
    let numbers = vec![1, 2, 3];

    let doubled = process_vector(numbers.clone(), |x| {
        // Multiply each number by 2
        x * 2
    });

    let replaced = process_vector(numbers, |x| {
        // Replace if number > 2 with 0, else keep the number
        if x > 2 { 0 } else { x }
    });

    println!("Doubled: {:?}", doubled);
    println!("Replaced: {:?}", replaced);
}