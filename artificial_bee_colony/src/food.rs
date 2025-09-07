use core::fmt;

use rand::Rng;

#[derive(Debug, Clone)]
pub struct FoodSource {
    pub function_calculation: f64,
    pub fitness_calculation: f64,
    pub coordinates: Vec<f64>,
    pub try_counter: u128,
}

impl FoodSource {
    pub fn new(coordinates: Vec<f64>) -> Self {
        let mut food_source = FoodSource {
            fitness_calculation: 0.0,
            function_calculation: 1.0,
            coordinates,
            try_counter: 0,
        };
        food_source.fitness_function();
        food_source
    }
    fn fitness_function(&mut self) {
        let calculation = Self::calculate(self.coordinates.clone());
        self.function_calculation = calculation;
        if calculation >= 0.0 {
            self.fitness_calculation = 1.0 / (1.0 + calculation);
        } else {
            self.fitness_calculation = 1.0 + calculation.abs();
        }
    }

    fn calculate(decision_variables: Vec<f64>) -> f64 {
        let mut result = 0.0;
        for element in decision_variables {
            result += element * element;
        }
        result
    }

    pub fn create_food_sources(
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
            food_sources.push(FoodSource::new(coordinates));
        }
        food_sources
    }

    pub fn find_best_food_source(food_sources: &Vec<FoodSource>) -> FoodSource {
        food_sources
            .iter()
            .max_by(|x, y| x.fitness_calculation.total_cmp(&y.fitness_calculation))
            .unwrap()
            .clone()
    }
}

impl fmt::Display for FoodSource {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        writeln!(
            f,
            "function_calculation = {:e}\nfitness_calculation = {:e}\ncoordinates = [",
            self.function_calculation, self.fitness_calculation
        )?;
        for coordinate in &self.coordinates {
            writeln!(f, "   {:e},", coordinate)?;
        }
        write!(f, "]\ntry_counter = {}\n", self.try_counter)
    }
}
