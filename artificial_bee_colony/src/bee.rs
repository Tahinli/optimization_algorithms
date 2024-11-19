use rand::Rng;

use crate::{food::FoodSource, utils::Input};

pub struct Bee {}
impl Bee {
    pub fn send_all_employed_bees(food_sources: &mut [FoodSource], input: &Input) {
        for food_source_index in 0..input.food_source_number {
            Bee::employed_bee(
                food_sources,
                food_source_index,
                input.decision_variable_count,
                input.upper_bound,
                input.lower_bound,
            );
        }
    }

    pub fn send_all_onlooker_bees(food_sources: &mut [FoodSource], input: &Input) {
        let total_fitness = food_sources
            .iter()
            .map(|food_source| food_source.fitness_calculation)
            .sum::<f64>();

        let probabilities = food_sources
            .iter()
            .map(|x| x.fitness_calculation / total_fitness)
            .collect::<Vec<f64>>();

        let onlooker_bee_count = input.food_source_number;
        let mut where_to_look = 0;
        for _ in 0..onlooker_bee_count {
            'decide_what_dance_to_follow: loop {
                if where_to_look >= input.food_source_number {
                    where_to_look = 0;
                }

                if rand::thread_rng().gen_range(0.0..=1.0) < probabilities[where_to_look] {
                    Bee::onlooker_bee(
                        food_sources,
                        where_to_look,
                        input.decision_variable_count,
                        input.upper_bound,
                        input.lower_bound,
                    );
                    break 'decide_what_dance_to_follow;
                }

                where_to_look += 1;
            }
        }
    }

    fn employed_bee(
        food_sources: &mut [FoodSource],
        food_source_index: usize,
        decision_variable_count: usize,
        upper_bound: f64,
        lower_bound: f64,
    ) {
        Self::send_bee(
            food_sources,
            food_source_index,
            decision_variable_count,
            upper_bound,
            lower_bound,
        );
    }

    fn onlooker_bee(
        food_sources: &mut [FoodSource],
        food_source_index: usize,
        decision_variable_count: usize,
        upper_bound: f64,
        lower_bound: f64,
    ) {
        Self::send_bee(
            food_sources,
            food_source_index,
            decision_variable_count,
            upper_bound,
            lower_bound,
        );
    }

    pub fn scout_bee(
        food_sources: &mut [FoodSource],
        most_tried_food_source_index: usize,
        lower_bound: f64,
        upper_bound: f64,
        decision_variable_count: usize,
    ) {
        let new_food_source = FoodSource::new(decision_variable_count, lower_bound, upper_bound);
        food_sources[most_tried_food_source_index] = new_food_source;
    }

    fn send_bee(
        food_sources: &mut [FoodSource],
        food_source_index: usize,
        decision_variable_count: usize,
        upper_bound: f64,
        lower_bound: f64,
    ) {
        let mut different_food_source_index = rand::thread_rng().gen_range(0..food_sources.len());
        while different_food_source_index == food_source_index {
            different_food_source_index = rand::thread_rng().gen_range(0..food_sources.len());
        }
        let decision_variable_index = rand::thread_rng().gen_range(0..decision_variable_count);
        let randomness = rand::thread_rng().gen_range(-1.0..=1.0);

        let mut candidate_decision_variable = food_sources[food_source_index].coordinates
            [decision_variable_index]
            + randomness
                * (food_sources[food_source_index].coordinates[decision_variable_index]
                    - food_sources[different_food_source_index].coordinates
                        [decision_variable_index]);
        if candidate_decision_variable > upper_bound {
            candidate_decision_variable = upper_bound;
        }
        if candidate_decision_variable < lower_bound {
            candidate_decision_variable = lower_bound;
        }

        let candidate_food_source = {
            let mut original_decision_variables =
                food_sources[food_source_index].coordinates.clone();
            original_decision_variables[decision_variable_index] = candidate_decision_variable;
            let candidate_decision_variables = original_decision_variables;
            FoodSource::from_coordinates(candidate_decision_variables)
        };

        food_sources[food_source_index].try_counter += 1;
        if candidate_food_source.fitness_calculation
            > food_sources[food_source_index].fitness_calculation
        {
            food_sources[food_source_index] = candidate_food_source;
        }
    }
}
