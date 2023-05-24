use requestty::*;

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

    println!("Your coefficients are: {:#?}", coefficients);
}
