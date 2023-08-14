use core::panic;
use std::io;

// Please get in touch with this newbie if anything is wrong :)
fn main() {

    // Welcome to simple calculator
    println!("Welcome to my simple calculator :)");
    // Operator choosing
        println!("Please enter the calculation operator.(+ , - , * , / )");

        let mut user_op_input = String::new();
        io::stdin().read_line(&mut user_op_input).expect("Failed to read your message! Please try again.");
        let user_op_input  = user_op_input.trim();
        // Enum instance
    let operator_menu = match user_op_input{
        "+" => Operation::Add ,
        "-" => Operation::Subtract,
        "*" => Operation::Multiply,
        "/" => Operation::Divide,
        _=>{
            println!("Please enter a valid operator !");
            return;
        }
    };
    
    // First number input

    println!("Please enter the first number .");

    let mut num1 = String::new();
    io::stdin().read_line(&mut num1).expect("Failed to read the first number ! Please try again.");
    let num1:f64= num1.trim().parse().expect("Please enter a valid number !");

    // Second number input 

    println!("Please enter the second number .");

    let mut num2 =String::new();
    io::stdin().read_line(&mut num2).expect("Failed to read the second number ! Please 
    try again.");
    let num2:f64 = num2.trim().parse().expect("Please enter a valid number !");

    // Calculation
    let calculation_result = calculate(operator_menu, num1, num2);

    // Result
    println!("Calculation result is {}",calculation_result);
}
enum Operation{
    Add,
    Subtract,
    Multiply,
    Divide,
}

fn calculate(operator_menu:Operation, num1:f64, num2:f64)->f64{

    match operator_menu{
        Operation::Add=> num1 + num2,
        Operation::Subtract => num1 - num2,
        Operation::Multiply=> num1 * num2,
        Operation::Divide=> num1 / num2,
        _=> panic!("There is no such calculation method! Please enter a valid method."),
    }

}
