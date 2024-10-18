use artificial_bee_colony::{
    bee::Bee,
    food::FoodSource,
    utils::{give_output, Input},
};
fn main() {
    println!("Hello, world!");

    let input = Input::get();
    let mut function_results = vec![];
    let mut fitness_results = vec![];

    for run_counter in 0..input.run {
        let mut food_sources = FoodSource::create_food_sources(
            input.food_source_number,
            input.decision_variable_count,
            input.upper_bound,
            input.lower_bound,
        );
        let mut best_food_source = FoodSource::find_best_food_source(&food_sources);

        for _ in 0..input.iteration {
            for food_source_index in 0..input.food_source_number {
                Bee::employed_bee(
                    &mut food_sources,
                    food_source_index,
                    input.decision_variable_count,
                    input.upper_bound,
                    input.lower_bound,
                );
            }

            let total_fitness = food_sources
                .iter()
                .map(|food_source| food_source.fitness_calculation)
                .sum();
            let onlooker_bee_count = input.food_source_number;
            let mut last_looked = 0;
            for _ in 0..onlooker_bee_count {
                loop {
                    if last_looked >= input.food_source_number {
                        last_looked = 0;
                    }
                    if Bee::onlooker_bee(
                        &mut food_sources,
                        last_looked,
                        total_fitness,
                        input.decision_variable_count,
                        input.upper_bound,
                        input.lower_bound,
                    ) {
                        break;
                    }
                    last_looked += 1;
                }
            }
            best_food_source = FoodSource::find_best_food_source(&food_sources);

            let most_tried_food_source_index =
                FoodSource::find_most_tried_food_source_index(&food_sources);
            Bee::scout_bee(
                &mut food_sources,
                most_tried_food_source_index,
                input.food_source_try_limit,
                input.lower_bound,
                input.upper_bound,
                input.decision_variable_count,
            );
        }
        function_results.push(best_food_source.function_calculation);
        fitness_results.push(best_food_source.fitness_calculation);

        give_output(
            &best_food_source,
            &function_results[..],
            &fitness_results[..],
            input.run,
            run_counter,
        )
    }
}
