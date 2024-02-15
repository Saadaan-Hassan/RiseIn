fn main() {
    let string1: String = String::from("Saadaan");
    let string2: String = String::from("Hassan");

    let concatenated_string: String = concatenate_strings(&string1, &string2);

    println!("String 1: {}", string1);
    println!("String 2: {}", string2);
    println!("Concatenated String: {}", concatenated_string);
}

fn concatenate_strings(inp1: &str, inp2: &str) -> String {
    let mut result: String = String::from(inp1);
    result.push_str(inp2);

    result
}
