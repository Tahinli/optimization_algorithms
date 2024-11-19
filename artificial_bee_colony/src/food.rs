use rand::Rng;

#[derive(Debug, Clone)]
pub struct FoodSource {
    pub fitness: f64,
    pub coordinates: Vec<f64>,
    pub try_counter: u128,
}

impl FoodSource {
    pub fn get(coordinates: Vec<f64>) -> Self {
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
            food_sources.push(FoodSource::get(coordinates));
        }
        food_sources
    }
}
