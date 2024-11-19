use rand::Rng;

use crate::food::FoodSource;

pub struct Bee {}
impl Bee {
    pub fn employed_bee(
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

    pub fn onlooker_bee(
        food_sources: &mut [FoodSource],
        food_source_index: usize,
        total_fitness: f64,
        decision_variable_count: usize,
        upper_bound: f64,
        lower_bound: f64,
    ) -> bool {
        let fitness_for_index = food_sources[food_source_index].fitness_calculation;
        if fitness_for_index / total_fitness <= rand::thread_rng().gen_range(0.0..=1.0) {
            Self::send_bee(
                food_sources,
                food_source_index,
                decision_variable_count,
                upper_bound,
                lower_bound,
            );
            return true;
        }
        false
    }

    pub fn scout_bee(
        food_sources: &mut [FoodSource],
        most_tried_food_source_index: usize,
        limit: u128,
        lower_bound: f64,
        upper_bound: f64,
        decision_variable_count: usize,
    ) {
        if food_sources[most_tried_food_source_index].try_counter > limit {
            let mut coordinates_for_new = vec![];
            for _ in 0..decision_variable_count {
                let random = lower_bound
                    + rand::thread_rng().gen_range(0.0..=1.0) * (upper_bound - lower_bound);
                coordinates_for_new.push(random);
            }
            let new_food_source = FoodSource::new(coordinates_for_new);
            food_sources[most_tried_food_source_index] = new_food_source;
        }
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
            FoodSource::new(candidate_decision_variables)
        };

        food_sources[food_source_index].try_counter += 1;
        if candidate_food_source.fitness_calculation
            > food_sources[food_source_index].fitness_calculation
        {
            food_sources[food_source_index] = candidate_food_source;
            food_sources[food_source_index].try_counter = 0;
        }
    }
}
