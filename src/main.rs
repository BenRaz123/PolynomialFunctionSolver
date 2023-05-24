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
        iterator -= 1;
    }
    eqn_string
}

fn main() {
    let mut coefficients: Vec<f64> = Vec::new();

    // Getting the coefficients
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
    println!("Your equation is: {}", make_eqn(&coefficients))

}
