fn invert_the_case(s: String) -> String {
    s.chars()
        .map(|c| {
            if c.is_lowercase() {
                c.to_uppercase().to_string()
            } else {
                c.to_lowercase().to_string()
            }
        })
        .collect()
}

fn main() {
    let input = "World".to_string();
    let output = invert_the_case(input.clone());
    println!("Input: {}, Output: {}", input, output);
}
