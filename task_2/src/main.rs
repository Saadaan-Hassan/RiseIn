use std::io;

fn main() {
    println!("----------Calculator----------");
    let mut num1 = String::new();
    let mut num2 = String::new();
    let mut oper = String::new();

    println!("Enter a operand-1: ");
    io::stdin()
        .read_line(&mut num1)
        .expect("Failed to read input");
    num1 = num1.trim().to_string();

    println!("Enter a operand-2: ");
    io::stdin()
        .read_line(&mut num2)
        .expect("Failed to read input");
    num2 = num2.trim().to_string();

    println!(
        "Select the operator: \n1-Add \n2-Subtract \n3- Multiply \n4- Divide\nEnter the operator: "
    );
    io::stdin()
        .read_line(&mut oper)
        .expect("Failed to read input");
    
    let oper: u32 = oper.trim().parse().expect("Invalid input");


    let operation = match oper {
        1 => Operation::Add( num1.parse().expect("Invalid Input"),
        num2.parse().expect("Invalid Input")),
        2 => Operation::Subtract(num1.parse().expect("Invalid Input"),
        num2.parse().expect("Invalid Input")),
        3 => Operation::Multiply(num1.parse().expect("Invalid Input"),
        num2.parse().expect("Invalid Input")),
        4 => Operation::Divide(num1.parse().expect("Invalid Input"),
        num2.parse().expect("Invalid Input")),
        _ => panic!("Invalid operation"),
    };
    let result = calculate(operation);
    println!("Result: {}", result);
}

enum Operation {
    Add(f64, f64),
    Subtract(f64, f64),
    Multiply(f64, f64),
    Divide(f64, f64),
}

fn calculate(operation: Operation) -> f64 {
    match operation {
        Operation::Add(n1, n2) => n1 + n2,
        Operation::Subtract(n1, n2) => n1 - n2,
        Operation::Multiply(n1, n2) => n1 * n2,
        Operation::Divide(n1, n2) => n1 / n2,
    }
}
