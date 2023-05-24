use requestty::*;

fn make_eqn(coefficient_list: &Vec<f64>) -> String {
    let mut eqn_string: String = "f(x) = ".to_owned();
    let mut iterator: usize = coefficient_list.len() - 1;
    for number in coefficient_list {
        let sign: String;
        if *number < f64::from(0) {
            sign = "-".to_owned();
        } else {
            sign = "+".to_owned();
        }
        eqn_string.push_str(&format!("{} {}x^{} ", sign, number.abs(), iterator));
        if iterator != 0 {
            iterator -= 1;
        }
    }
    eqn_string
}

fn gather_undefined_number_of_float_inputs(
    initial_message: &str,
    message: &str,
    continue_message: &str,
) -> Vec<f64> {
    let mut result: Vec<f64> = Vec::new();

    let question_initial = Question::float(*&initial_message)
        .message(*&initial_message)
        .build();
    let initial = prompt_one(question_initial)
        .expect("Could not parse question")
        .as_float()
        .expect("Could not convert to number");

    result.append(&mut vec![initial]);

    loop {
        let question_wants_to_continue = Question::confirm("wants to continue")
            .message(*&continue_message)
            .build();
        let wants_to_continue: bool = prompt_one(question_wants_to_continue)
            .expect("Could not process answer")
            .as_bool()
            .expect("Could not convert answer to bool");
        if !wants_to_continue {
            break;
        }
        let question_nth = Question::float("coefficient").message(message).build();
        let nth: f64 = prompt_one(question_nth)
            .expect("Could not parse answer")
            .as_float()
            .expect("Could not convert to bool");
        result.append(&mut vec![nth]);
    }

    result
}

fn calculate_input(input: f64, coefficients: &Vec<f64>) -> f64 {
    let mut result: f64 = 0.0;
    let mut iterator: i32 = (coefficients.len() as i32) - 1;
    for coefficient in coefficients {
        result += coefficient * input.powi(*&iterator);
        if iterator != 0 {
            iterator -= 1;
        }
    }
    result
}

fn main() {
    let coefficients: Vec<f64> = gather_undefined_number_of_float_inputs(
        "Leading Coefficient",
        "Coefficient",
        "Do you want to contine giving coefficients or stop?",
    );
    let inputs: Vec<f64> = gather_undefined_number_of_float_inputs(
        "Input",
        "Input",
        "Do you want to continue giving coefficients or stop?",
    );
    println!("This is your equation: {}", make_eqn(&coefficients));
    for input in inputs {
        println!("f({}) = {}", input, calculate_input(input, &coefficients));
    }
}
