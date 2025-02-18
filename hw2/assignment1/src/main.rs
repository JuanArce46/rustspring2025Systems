fn concat_strings(s1: &String, s2: &String) -> String {
    let mut result = s1.clone(); // Clone the first string
    result.push_str(s2); // Append the second string to the result
    result // Return the result
}

fn main() {
    let s1 = String::from("Hello, ");
    let s2 = String::from("World!");
    let result = concat_strings(&s1, &s2);
    println!("{}", result); // Should print: "Hello, World!"
}
