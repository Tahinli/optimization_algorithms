use std::io;

use rand::Rng;

fn main() {
    println!("Hello, world!");

    println!("Decision Variable Count");
    let decision_variable_count = get_float_input() as usize;

    println!("Population Number");
    let population_number = get_float_input() as u64;

    println!("Crossover Rate");
    let crossover_rate = get_float_input();

    println!("Scale Factor");
    let scale_factor = get_float_input();

    println!("Upper Bound");
    let upper_bound = get_float_input();

    println!("Lower Bound");
    let lower_bound = get_float_input();

    println!("Iteration");
    let iteration = get_float_input() as usize;

    let mut population = create_population(
        population_number,
        decision_variable_count,
        upper_bound,
        lower_bound,
    );

    for _ in 0..iteration {
        for (current_location, element) in population.clone().iter().enumerate() {
            let new_volunteer = mutate_and_recombine(
                current_location,
                crossover_rate,
                scale_factor,
                decision_variable_count,
                population.clone(),
            );

            let current_calculation = calculate(element.clone());
            let new_calculation = calculate(new_volunteer.clone());
            if new_calculation < current_calculation {
                println!(
                    "Changed | Old = {} | New {}",
                    current_calculation, new_calculation
                );
                population[current_location] = new_volunteer;
            }
        }
    }

    println!("{:#?}", population);
}

fn get_float_input() -> f64 {
    let mut input = String::new();
    io::stdin().read_line(&mut input).unwrap();
    let input: f64 = input.trim().parse().unwrap();
    input
}

fn create_population(
    population_number: u64,
    decision_variable_count: usize,
    upper_bound: f64,
    lower_bound: f64,
) -> Vec<Vec<f64>> {
    let mut population = vec![vec![decision_variable_count as f64]; population_number as usize];

    for i in 0..population_number {
        let mut randomized_single_dimension = vec![];
        for _ in 0..decision_variable_count {
            randomized_single_dimension
                .push(rand::thread_rng().gen_range(lower_bound..=upper_bound));
        }
        population[i as usize] = randomized_single_dimension;
    }
    population
}

fn calculate(decision_variables: Vec<f64>) -> f64 {
    let mut result = 0.0;
    for element in decision_variables {
        result += element * element;
    }
    result
}

fn mutate_and_recombine(
    current_location: usize,
    crossover_rate: f64,
    scale_factor: f64,
    decision_variable_count: usize,
    population: Vec<Vec<f64>>,
) -> Vec<f64> {
    let definite_random_decision_index: usize =
        rand::thread_rng().gen_range(0..=decision_variable_count);
    let mut definite_random = rand::thread_rng().gen_range(0..population.len());

    while definite_random == current_location {
        definite_random = rand::thread_rng().gen_range(0..population.len());
    }

    let mut chosen_indices = vec![];
    for _ in 0..3 {
        chosen_indices.sort();

        let mut maybe = rand::thread_rng().gen_range(0..population.len());
        while maybe == current_location || chosen_indices.binary_search(&maybe).is_ok() {
            maybe = rand::thread_rng().gen_range(0..population.len());
        }
        chosen_indices.push(maybe);
    }

    let ingredients: Vec<Vec<f64>> = vec![
        population[chosen_indices[0]].clone(),
        population[chosen_indices[1]].clone(),
        population[chosen_indices[2]].clone(),
    ];

    let mut new_volunteer = vec![];

    for i in 0..decision_variable_count {
        if rand::thread_rng().gen_range(0.0..=1.0) < crossover_rate
            || i == definite_random_decision_index
        {
            new_volunteer
                .push(ingredients[0][i] + scale_factor * (ingredients[1][i] - ingredients[2][i]));
        } else {
            new_volunteer.push(population[definite_random][i]);
        }
    }
    
    new_volunteer
}
