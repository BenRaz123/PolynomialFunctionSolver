use requestty::*;

fn make_eqn(coefficient_list: &Vec<f64>) -> String {
    let mut eqn_string: String = "f(x) = ".to_owned();
    let mut iterator: usize = coefficient_list.len()-1;
    for number in coefficient_list {
        let sign: String;
        if *number < f64::from(0) {
            sign = "-".to_owned();
        } else {
            sign = "+".to_owned();
        }
        eqn_string.push_str(&format!("{} {}x^{} ", sign, number, iterator));
        if iterator != 0 {
            iterator -=1;
        }
    }
    eqn_string
}

fn calculate_input(input: f64, coefficients: Vec<f64>) -> f64 {
    let mut result: f64 = 0.0;
    let mut iterator: i32 = ( coefficients.len() as i32 ) -1;
    for coefficient in coefficients {
        result += coefficient * input.powi(*&iterator);
        if iterator != 0 {
            iterator -= 1;
        }
    }
    result
}

fn main() {
    let mut coefficients: Vec<f64> = Vec::new();
    let mut inputs: Vec<f64> = Vec::new();
    
    // Getting the coefficients
    
    let question_leading_coefficient = Question::float("leading coefficient").message("Enter your leading coefficient").build();
    let leading_coefficient = prompt_one(question_leading_coefficient).expect("Could not parse question").as_float().expect("Could not convert to number");

    coefficients.append(&mut vec![leading_coefficient]);

    loop {
        let question_wants_to_continue = Question::confirm("wants to continue").message("Do you want to give another coefficient?").build();
        let wants_to_continue: bool = prompt_one(question_wants_to_continue).expect("Could not process answer").as_bool().expect("Could not convert answer to bool");
        if !wants_to_continue {
            break;
        }
        let question_coefficient = Question::float("coefficient").message("Please enter a coefficient").build();
        let coefficient: f64 = prompt_one(question_coefficient).expect("Could not parse answer").as_float().expect("Could not convert to bool"); 
        coefficients.append(&mut vec![coefficient]);
    }
    println!("Your equation is: {}", make_eqn(&coefficients));
    
    // Getting the inputs
    
    let question_first_input = Question::float("first input").message("Please enter a numerical input").build();
    let first_input: f64 = prompt_one(question_first_input).expect("Could not parse question").as_float().expect("Could not convert to number");

    inputs.append(&mut vec![first_input]);
    
    loop {
        let question_wants_to_continue = Question::confirm("wants to continue").message("Do you want to give another input?").build();
        let wants_to_continue = prompt_one(question_wants_to_continue).expect("Could not process answer").as_bool().expect("Could not convert answer to bool");
        if !wants_to_continue {
            break;
        }

        let question_input = Question::float("input").message("Please enter a numerical input").build();
        let input: f64 = prompt_one(question_input).expect("Could not parse answer").as_float().expect("could not convert to number");
        inputs.append(&mut vec![input]);
    }
}
