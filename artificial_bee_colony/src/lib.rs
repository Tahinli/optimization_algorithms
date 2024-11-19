use std::io;

pub mod bee;
pub mod food;

pub struct Input {
    pub decision_variable_count: usize,
    pub food_source_number: u128,
    pub food_source_try_limit: u128,
    pub upper_bound: f64,
    pub lower_bound: f64,
    pub iteration: usize,
}
impl Input {
    pub fn get() -> Self {
        println!("Decision Variable Count");
        let decision_variable_count = Self::get_input().parse().unwrap();

        println!("Food Source Number");
        let food_source_number = Self::get_input().parse().unwrap();

        println!("Food Source Try Limit");
        let food_source_try_limit = Self::get_input().parse().unwrap();

        println!("Upper Bound");
        let upper_bound = Self::get_input().parse().unwrap();

        println!("Lower Bound");
        let lower_bound = Self::get_input().parse().unwrap();

        println!("Iteration");
        let iteration = Self::get_input().parse().unwrap();

        Input {
            decision_variable_count,
            food_source_number,
            food_source_try_limit,
            upper_bound,
            lower_bound,
            iteration,
        }
    }
    fn get_input() -> String {
        let mut input = String::new();
        io::stdin().read_line(&mut input).unwrap();
        input.trim().to_string()
    }
}
