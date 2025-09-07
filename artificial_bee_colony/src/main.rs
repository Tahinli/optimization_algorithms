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

    let mut food_sources = FoodSource::create_food_sources(
        input.food_source_number,
        input.decision_variable_count,
        input.upper_bound,
        input.lower_bound,
    );

    for run_counter in 0..input.run {
        let mut best_food_source = FoodSource::find_best_food_source(&food_sources);

        for _ in 0..input.iteration {
            for index in 0..input.food_source_number as usize {
                Bee::employed_bee(
                    &mut food_sources,
                    index,
                    input.decision_variable_count,
                    input.upper_bound,
                    input.lower_bound,
                );
            }
            let total_fitness = food_sources.iter().map(|food_source|food_source.fitness_calculation).sum();
            let mut last_looked = 0;
            for _ in 0..input.food_source_number {
                if last_looked >= input.food_source_number {
                    last_looked = 0;
                }
                
            }
            for index in 0..input.food_source_number as usize {
                Bee::onlooker_bee(
                    &mut food_sources,
                    index,
                    total_fitness,
                    input.decision_variable_count,
                    input.upper_bound,
                    input.lower_bound,
                );
            }
            // önce employed biticek sonra onlooker bakacak ve bakılandan tekrar başla
            best_food_source = FoodSource::find_best_food_source(&food_sources);

            for index in 0..food_sources.len() {
                if food_sources[index].try_counter >= input.food_source_try_limit {
                    // en büyüğü bul sonra limiti geçiyosa kaşif gider
                    Bee::scout_bee(
                        &mut food_sources,
                        index,
                        input.food_source_try_limit,
                        input.lower_bound,
                        input.upper_bound,
                        input.decision_variable_count,
                    );
                    break;
                }
            }
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
