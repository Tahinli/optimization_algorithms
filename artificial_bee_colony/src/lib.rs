use std::{
    env,
    fs::File,
    io::{self, BufRead, BufReader},
};

pub mod bee;
pub mod food;

pub struct Input {
    pub decision_variable_count: usize,
    pub food_source_number: u128,
    pub food_source_try_limit: u128,
    pub upper_bound: f64,
    pub lower_bound: f64,
    pub iteration: usize,
    pub run: usize,
}
impl Input {
    pub fn get() -> Self {
        let args = env::args().collect::<Vec<String>>();
        if args.len() > 1 {
            if args[1] == "-f" || args[1] == "--file" {
                Input::read_config_file_input()
            } else {
                Input::user_interactive_input()
            }
        } else {
            Input::user_interactive_input()
        }
    }

    fn user_interactive_input() -> Self {
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

        println!("Run");
        let run = Self::get_input().parse().unwrap();
        Input {
            decision_variable_count,
            food_source_number,
            food_source_try_limit,
            upper_bound,
            lower_bound,
            iteration,
            run,
        }
    }

    fn read_config_file_input() -> Input {
        let mut config_file_input = Self {
            decision_variable_count: 0,
            food_source_number: 0,
            food_source_try_limit: 0,
            upper_bound: 0.0,
            lower_bound: 0.0,
            iteration: 0,
            run: 0,
        };

        let config_file = File::open("abc.toml").unwrap();
        let reader = BufReader::new(config_file);
        let mut words = vec![];
        for line in reader.lines().map(|x| x.unwrap()).collect::<Vec<String>>() {
            let mut parsed = line
                .split('=')
                .map(|x| x.trim().to_string())
                .collect::<Vec<String>>();
            words.append(&mut parsed);
        }

        for i in 0..words.len() {
            if i % 2 == 0 {
                continue;
            }
            match words[i].as_str() {
                "decision_variable_count" => {
                    config_file_input.decision_variable_count = words[i + 1].parse().unwrap()
                }
                "food_source_number" => {
                    config_file_input.food_source_number = words[i + 1].parse().unwrap()
                }
                "food_source_try_limit" => {
                    config_file_input.food_source_try_limit = words[i + 1].parse().unwrap()
                }
                "upper_bound" => config_file_input.upper_bound = words[i + 1].parse().unwrap(),
                "lower_bound" => config_file_input.lower_bound = words[i + 1].parse().unwrap(),
                "iteration" => config_file_input.iteration = words[i + 1].parse().unwrap(),
                "run" => config_file_input.run = words[i + 1].parse().unwrap(),
                _ => {}
            }
        }
        config_file_input
    }

    fn get_input() -> String {
        let mut input = String::new();
        io::stdin().read_line(&mut input).unwrap();
        input.trim().to_string()
    }
}
