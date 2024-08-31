use rand::Rng;

use crate::food::FoodSource;

pub enum Bee {}
impl Bee {
    pub fn employed_bee(
        food_sources: &mut [FoodSource],
        index: usize,
        decision_variable_count: usize,
        upper_bound: f64,
        lower_bound: f64,
    ) {
        let mut different_food_source_index = rand::thread_rng().gen_range(0..food_sources.len());
        while different_food_source_index == index {
            different_food_source_index = rand::thread_rng().gen_range(0..food_sources.len());
        }
        let selected_coordinate_index = rand::thread_rng().gen_range(0..decision_variable_count);
        let randomness = rand::thread_rng().gen_range(-1.0..=1.0);

        let mut candidate_one_index = food_sources[index].coordinates[selected_coordinate_index]
            + randomness
                * (food_sources[index].coordinates[selected_coordinate_index]
                    - food_sources[different_food_source_index].coordinates
                        [selected_coordinate_index]);
        if candidate_one_index > upper_bound {
            candidate_one_index = upper_bound;
        }
        if candidate_one_index < lower_bound {
            candidate_one_index = lower_bound;
        }
        let mut candidate_coordinates = food_sources[index].coordinates.clone();
        candidate_coordinates[selected_coordinate_index] = candidate_one_index;
        let candidate = FoodSource::new(candidate_coordinates);
        food_sources[index].try_counter += 1;
        if candidate.fitness_calculation > food_sources[index].fitness_calculation {
            food_sources[index] = candidate;
            food_sources[index].try_counter = 0;
        }
    }

    pub fn onlooker_bee(
        food_sources: &mut [FoodSource],
        index: usize,
        total_fitness: f64,
        decision_variable_count: usize,
        upper_bound: f64,
        lower_bound: f64,
    ) {
        let fitness_for_index = food_sources[index].fitness_calculation;
        if fitness_for_index / total_fitness <= rand::thread_rng().gen_range(0.0..=1.0) {
            Self::employed_bee(
                food_sources,
                index,
                decision_variable_count,
                upper_bound,
                lower_bound,
            );
        }
    }

    pub fn scout_bee(
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
            let new_food_source = FoodSource::new(coordinates_for_new);
            food_sources[most_tried_index] = new_food_source;
        }
    }
}
