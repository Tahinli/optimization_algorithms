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
            Bee::send_all_employed_bees(&mut food_sources, &input);

            Bee::send_all_onlooker_bees(&mut food_sources, &input);
            
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
