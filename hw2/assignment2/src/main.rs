fn clone_and_modify(s: &String) -> String {
    let mut cloned = s.clone(); // Clone the original string
    cloned.push_str("World!"); // Modify the cloned string by appending "World!"
    cloned // Return the modified clone
}

fn main() {
    let s = String::from("Hello, ");
    let modified = clone_and_modify(&s);
    println!("Original: {}", s); // Should print: "Original: Hello, "
    println!("Modified: {}", modified); // Should print: "Modified: Hello, World!"
}
