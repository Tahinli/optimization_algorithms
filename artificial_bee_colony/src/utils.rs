use core::str;
use std::{
    env::args,
    fs::{File, OpenOptions},
    io::{self, BufRead, BufReader, Write},
    process::exit,
};

use crate::food::FoodSource;

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
        let mut read_file = false;
        for arg in args().collect::<Vec<String>>() {
            match arg.as_str() {
                "-r" | "--read_file" => read_file = true,
                "-h" | "--help" => {
                    show_help();
                    exit(0);
                }
                _ => {}
            }
        }

        if read_file {
            Input::read_config_file_input()
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

        let config_file = File::open("abc_config.toml").unwrap();
        let reader = BufReader::new(config_file);
        let mut words = vec![];
        for line in reader.lines().map(|x| x.unwrap()).collect::<Vec<String>>() {
            let mut parsed = line
                .split('=')
                .map(|x| x.trim().to_string())
                .collect::<Vec<String>>();
            words.append(&mut parsed);
        }
        if words[0].as_str() == "[start_parameters]" {
            for i in (1..words.len()).step_by(2) {
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
        }
        config_file_input
    }

    fn get_input() -> String {
        let mut input = String::new();
        io::stdin().read_line(&mut input).unwrap();
        input.trim().to_string()
    }
}

pub fn give_output(
    best_food_source: &FoodSource,
    function_results: &[f64],
    fitness_results: &[f64],
    input_run: usize,
    run_counter: usize,
) {
    let mut write_file = false;
    for arg in args().collect::<Vec<String>>() {
        match arg.as_str() {
            "-w" | "--write_file" => write_file = true,
            _ => {}
        }
    }

    let mut print_buffer = vec![];
    write!(print_buffer, "[{}]\n{}\n", run_counter, best_food_source).unwrap();
    if run_counter == input_run - 1 {
        let function_results_arithmetic_mean = arithmetic_mean(function_results);
        let function_results_standard_deviation =
            standard_deviation(function_results, function_results_arithmetic_mean);
        let fitness_results_arithmetic_mean = arithmetic_mean(fitness_results);
        let fitness_results_standard_deviation =
            standard_deviation(fitness_results, fitness_results_arithmetic_mean);
        write!(
            print_buffer,
            "[function_calculations_results]\narithmetic_mean = {:e}\nstandard_deviation = {:e}\n\n[fitness_calculations_results]\narithmetic_mean = {:e}\nstandard_deviation = {:e}",
            function_results_arithmetic_mean, function_results_standard_deviation, fitness_results_arithmetic_mean, fitness_results_standard_deviation
        )
        .unwrap();
    }
    if write_file {
        write_output_file(&print_buffer[..], run_counter)
    } else {
        give_terminal_output(&print_buffer[..])
    }
}

fn give_terminal_output(print_buffer: &[u8]) {
    println!("{}", str::from_utf8(print_buffer).unwrap());
}

fn write_output_file(print_buffer: &[u8], run_counter: usize) {
    let mut file_try_counter = 0;
    let file_name = "abc_result";
    let file_extension = "toml";
    let mut file_path = format!("{}.{}", file_name, file_extension);

    let mut file;
    if run_counter == 0 {
        while File::open(file_path.clone()).is_ok() {
            file_try_counter += 1;
            file_path = format!("{}{}.{}", file_name, file_try_counter, file_extension);
        }
        file = File::create_new(file_path).unwrap();
    } else {
        while File::open(file_path.clone()).is_ok() {
            file_try_counter += 1;
            file_path = format!("{}{}.{}", file_name, file_try_counter, file_extension);
        }
        if file_try_counter > 1 {
            file_path = format!("{}{}.{}", file_name, file_try_counter - 1, file_extension);
        } else {
            file_path = format!("{}.{}", file_name, file_extension);
        }
        file = OpenOptions::new().append(true).open(file_path).unwrap();
    }

    file.write_all(print_buffer).unwrap();
    file.flush().unwrap();
}

fn arithmetic_mean(results: &[f64]) -> f64 {
    let mut total_results = 0.0;
    for function_result in results {
        total_results += function_result
    }
    total_results / results.len() as f64
}

fn standard_deviation(results: &[f64], arithmetic_mean: f64) -> f64 {
    let mut total_difference_square = 0.0;
    for function_result in results {
        total_difference_square += (function_result - arithmetic_mean).powi(2)
    }
    f64::sqrt(total_difference_square / (results.len() - 1) as f64)
}

fn show_help() {
    println!("\n\n\n");
    println!("   Arguments           |  Details                    ");
    println!("----------------------------------------------------------------------");
    println!("   -r  -> --read_file  |  Reads Config from abc_config.toml File");
    println!("   -w  -> --write_file |  Writes Results to abc_result.toml File");
    println!("   -h  -> --help       |  Shows Help                 ");
    println!("\n\n\n");
}