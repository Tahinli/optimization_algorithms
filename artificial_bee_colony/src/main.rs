use std::io;

use rand::Rng;

#[derive(Debug, Clone)]
struct FoodSource {
    fitness: f64,
    coordinates: Vec<f64>,
    try_counter: u128,
}

impl FoodSource {
    fn get(coordinates: Vec<f64>) -> Self {
        let mut food_source = FoodSource {
            fitness: 0.0,
            coordinates,
            try_counter: 0,
        };
        if !food_source.coordinates.is_empty() {
            food_source.fitness_function();
        }
        food_source
    }
    fn fitness_function(&mut self) {
        let calculation = Self::calculate(self.coordinates.clone());
        if calculation >= 0.0 {
            self.fitness = 1.0 / (1.0 + calculation);
        } else {
            self.fitness = 1.0 + calculation.abs();
        }
    }

    fn calculate(decision_variables: Vec<f64>) -> f64 {
        let mut result = 0.0;
        for element in decision_variables {
            result += element * element;
        }
        result
    }

    fn create_food_sources(
        food_source_number: u128,
        decision_variable_count: usize,
        upper_bound: f64,
        lower_bound: f64,
    ) -> Vec<FoodSource> {
        let mut food_sources = vec![];

        for _ in 0..food_source_number {
            let mut coordinates = vec![];
            for _ in 0..decision_variable_count {
                let random = rand::thread_rng().gen_range(lower_bound..=upper_bound);
                coordinates.push(random);
            }
            food_sources.push(FoodSource::get(coordinates));
        }
        food_sources
    }
}
enum Bee {}
impl Bee {
    fn worker_bee(food_sources: &mut [FoodSource], index: usize, decision_variable_count: usize) {
        let mut different_food_source_index = rand::thread_rng().gen_range(0..food_sources.len());
        while different_food_source_index == index {
            different_food_source_index = rand::thread_rng().gen_range(0..food_sources.len());
        }
        let selected_coordinate_index = rand::thread_rng().gen_range(0..decision_variable_count);
        let randomness = rand::thread_rng().gen_range(-1.0..=1.0);

        let candidate_one_index = food_sources[index].coordinates[selected_coordinate_index]
            + randomness
                * (food_sources[index].coordinates[selected_coordinate_index]
                    - food_sources[different_food_source_index].coordinates
                        [selected_coordinate_index]);
        let mut candidate_coordinates = food_sources[index].coordinates.clone();
        candidate_coordinates[selected_coordinate_index] = candidate_one_index;
        let candidate = FoodSource::get(candidate_coordinates);
        food_sources[index].try_counter += 1;
        if candidate.fitness > food_sources[index].fitness {
            food_sources[index] = candidate;
            food_sources[index].try_counter = 0;
        }
    }

    fn onlooker_bee(
        food_sources: &mut [FoodSource],
        index: usize,
        total_fitness: f64,
        decision_variable_count: usize,
    ) {
        let fitness_for_index = food_sources[index].fitness;
        if fitness_for_index / total_fitness <= rand::thread_rng().gen_range(0.0..=1.0) {
            Self::worker_bee(food_sources, index, decision_variable_count);
        }
    }

    fn scout_bee(
        food_sources: &mut [FoodSource],
        most_tried_index: usize,
        limit: u128,
        lower_bound: f64,
        upper_bound: f64,
        decision_variable_count: usize,
    ) {
        if food_sources[most_tried_index].try_counter > limit {
            let mut coordinates_for_new = vec![];
            for _ in 0..decision_variable_count {
                let random = lower_bound
                    + rand::thread_rng().gen_range(0.0..=1.0) * (upper_bound - lower_bound);
                coordinates_for_new.push(random);
            }
            let new_food_source = FoodSource::get(coordinates_for_new);
            food_sources[most_tried_index] = new_food_source;
        }
    }
}
struct Input {
    decision_variable_count: usize,
    food_source_number: u128,
    food_source_try_limit: u128,
    upper_bound: f64,
    lower_bound: f64,
    iteration: usize,
}
impl Input {
    fn get() -> Self {
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
fn main() {
    println!("Hello, world!");

    let input = Input::get();

    let mut food_sources = FoodSource::create_food_sources(
        input.food_source_number,
        input.decision_variable_count,
        input.upper_bound,
        input.lower_bound,
    );
    let mut best = FoodSource::get(vec![]);
    for food_source in &food_sources {
        if best.fitness < food_source.fitness {
            best.coordinates = food_source.coordinates.clone();
            best.fitness = food_source.fitness;
        }
    }
    for _ in 0..input.iteration {
        for i in 0..input.food_source_number as usize {
            Bee::worker_bee(&mut food_sources, i, input.decision_variable_count);
            let mut total_fitness = 0.0;
            for food_source in &food_sources {
                total_fitness += food_source.fitness;
            }
            Bee::onlooker_bee(
                &mut food_sources,
                i,
                total_fitness,
                input.decision_variable_count,
            );
            let mut most_tried_index = 0;
            for i in 0..food_sources.len() {
                if food_sources[most_tried_index].try_counter < food_sources[i].try_counter {
                    most_tried_index = i;
                }
            }
            best = food_sources[most_tried_index].clone();
            Bee::scout_bee(
                &mut food_sources,
                most_tried_index,
                input.food_source_try_limit,
                input.lower_bound,
                input.upper_bound,
                input.decision_variable_count,
            );
        }
    }

    println!("{:#?}", best);
}
