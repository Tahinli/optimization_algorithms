use artificial_bee_colony::{bee::Bee, food::FoodSource, Input};
fn main() {
    println!("Hello, world!");

    let input = Input::get();

    let mut food_sources = FoodSource::create_food_sources(
        input.food_source_number,
        input.decision_variable_count,
        input.upper_bound,
        input.lower_bound,
    );

    for run_counter in 0..input.run {
        let mut best = FoodSource::get(vec![]);
        for food_source in &food_sources {
            if best.fitness < food_source.fitness {
                best.coordinates.clone_from(&food_source.coordinates);
                best.fitness = food_source.fitness;
            }
        }
        for _ in 0..input.iteration {
            for index in 0..input.food_source_number as usize {
                Bee::worker_bee(
                    &mut food_sources,
                    index,
                    input.decision_variable_count,
                    input.upper_bound,
                    input.lower_bound,
                );
                let mut total_fitness = 0.0;
                for food_source in &food_sources {
                    total_fitness += food_source.fitness;
                }
                Bee::onlooker_bee(
                    &mut food_sources,
                    index,
                    total_fitness,
                    input.decision_variable_count,
                    input.upper_bound,
                    input.lower_bound,
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
        println!("-------------------------------");
        println!("\n\t|Run {}|\n", run_counter);
        println!("{:#?}", best);
    }
}
