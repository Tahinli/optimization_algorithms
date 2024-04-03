use rand::Rng;
use std::io;

enum Bound {
    Upper,
    Lower,
    Nothing,
}

const E: f64 = std::f64::consts::E;
fn main() {
    println!("Hello, world!");

    println!("Temperature = ");
    let mut temperature = get_float_input();

    println!("Max Iteration = ");
    let max_iteration: u64 = get_float_input() as u64;

    println!("Upper Bound = ");
    let upper_bound = get_float_input();

    println!("Lower Bound = ");
    let lower_bound = get_float_input();

    let mut x1: f64 = rand::thread_rng().gen_range(lower_bound..=upper_bound);
    let mut x2: f64 = rand::thread_rng().gen_range(lower_bound..=upper_bound);

    let mut give_up_counter: u8 = 0;
    for i in 0..max_iteration {
        x1 = bound_detective(x1, upper_bound, lower_bound);
        x2 = bound_detective(x2, upper_bound, lower_bound);
        let new_x1: f64;
        let new_x2: f64;

        let where_to_go = rand::thread_rng().gen_range(0.0..=1.0);
        if where_to_go > 0.7 {
            if x1 >= 0.0 {
                new_x1 = rand::thread_rng().gen_range(x1..=(x1 + ((5.0 * x1) / 100.0)));
            } else {
                new_x1 = rand::thread_rng().gen_range(x1..=(x1 - ((5.0 * x1) / 100.0)));
            }
            if x2 >= 0.0 {
                new_x2 = rand::thread_rng().gen_range(x2..=(x2 + ((5.0 * x2) / 100.0)));
            } else {
                new_x2 = rand::thread_rng().gen_range(x2..=(x2 - ((5.0 * x2) / 100.0)));
            }
        } else if where_to_go > 0.5 {
            if x1 >= 0.0 {
                new_x1 = rand::thread_rng().gen_range(x1..=(x1 + ((5.0 * x1) / 100.0)));
            } else {
                new_x1 = rand::thread_rng().gen_range(x1..=(x1 - ((5.0 * x1) / 100.0)));
            }
            if x2 >= 0.0 {
                new_x2 = rand::thread_rng().gen_range((x2 - ((5.0 * x2) / 100.0))..=x2);
            } else {
                new_x2 = rand::thread_rng().gen_range((x2 + ((5.0 * x2) / 100.0))..=x2);
            }
        } else if where_to_go > 0.3 {
            if x1 >= 0.0 {
                new_x1 = rand::thread_rng().gen_range((x1 - ((5.0 * x1) / 100.0))..=x1);
            } else {
                new_x1 = rand::thread_rng().gen_range((x1 + ((5.0 * x1) / 100.0))..=x1);
            }
            if x2 >= 0.0 {
                new_x2 = rand::thread_rng().gen_range(x2..=(x2 + ((5.0 * x2) / 100.0)));
            } else {
                new_x2 = rand::thread_rng().gen_range(x2..=(x2 - ((5.0 * x2) / 100.0)));
            }
        } else {
            if x1 >= 0.0 {
                new_x1 = rand::thread_rng().gen_range((x1 - ((5.0 * x1) / 100.0))..=x1);
            } else {
                new_x1 = rand::thread_rng().gen_range((x1 + ((5.0 * x1) / 100.0))..=x1);
            }
            if x2 >= 0.0 {
                new_x2 = rand::thread_rng().gen_range((x2 - ((5.0 * x2) / 100.0))..=x2);
            } else {
                new_x2 = rand::thread_rng().gen_range((x2 + ((5.0 * x2) / 100.0))..=x2);
            }
        }

        let give_up_return = give_up(x1, x2, new_x1, new_x2, give_up_counter);
        if give_up_return.0 {
            println!("Time to give up\n Iteration = {}", i);
            break;
        }
        give_up_counter = give_up_return.1;

        temperature = turbofan_oven(temperature, i, max_iteration);

        let original_result = calculate(x1, x2);
        let new_result = calculate(new_x1, new_x2);
        let neighbour_dif = new_result - original_result;
        let temp_random = rand::thread_rng().gen_range(0.0..=1.0);
        let simulated_annealing_func = E.powf(-1.0 * neighbour_dif / temperature);

        if neighbour_dif <= 0.0 || simulated_annealing_func > temp_random {
            x1 = new_x1;
            x2 = new_x2;
        }
    }
    println!("x1 = {}\nx2 = {}\nResult = {}", x1, x2, calculate(x1, x2));
}

fn get_float_input() -> f64 {
    let mut input = String::new();
    io::stdin().read_line(&mut input).unwrap();
    let input: f64 = input.trim().parse().unwrap();
    input
}

fn calculate(x1: f64, x2: f64) -> f64 {
    (x1 * x1) + (x2 * x2)
}

fn bound_detective(x: f64, upper_bound: f64, lower_bound: f64) -> f64 {
    if x >= upper_bound {
        magic_trick(upper_bound, Bound::Upper)
    } else if x <= lower_bound {
        magic_trick(lower_bound, Bound::Lower)
    } else {
        magic_trick(x, Bound::Nothing)
    }
}

fn magic_trick(x: f64, what_happened: Bound) -> f64 {
    if x >= 0.0 {
        match what_happened {
            Bound::Upper => return x - (0.000000001 * x / 100.0),
            Bound::Lower => return x + (0.000000001 * x / 100.0),
            Bound::Nothing => return x,
        }
    } else {
        match what_happened {
            Bound::Upper => return x + (0.000000001 * x / 100.0),
            Bound::Lower => return x - (0.000000001 * x / 100.0),
            Bound::Nothing => return x,
        }
    }
}

fn turbofan_oven(mut temperature: f64, i: u64, max_iteration: u64) -> f64 {
    let where_we_are = (i as f64 / max_iteration as f64) * 100.0;
    if where_we_are < 5.0 {
    } else if where_we_are < 10.0 {
        temperature -= (1.0 / 100.0) * temperature;
    } else if where_we_are < 15.0 {
        temperature += (1.0 / 100.0) * temperature;
    } else if where_we_are < 20.0 {
        temperature -= (1.0 / 100.0) * temperature;
    } else if where_we_are < 25.0 {
        temperature += (1.0 / 100.0) * temperature;
    } else if where_we_are < 30.0 {
        temperature -= (1.0 / 100.0) * temperature;
    } else if where_we_are < 35.0 {
        temperature += (1.0 / 100.0) * temperature;
    } else if where_we_are < 40.0 {
        temperature -= (1.0 / 100.0) * temperature;
    } else if where_we_are < 50.0 {
        temperature += (2.0 / 100.0) * temperature;
    } else if where_we_are < 60.0 {
    } else if where_we_are < 70.0 {
        temperature -= (5.0 / 100.0) * temperature;
    }
    temperature
}

fn give_up(x1: f64, x2: f64, new_x1: f64, new_x2: f64, mut give_up_counter: u8) -> (bool, u8) {
    if x1 == 0.0 && x2 == 0.0 {
        println!("Touched 0.0 for both variables");
        return (true, give_up_counter);
    }

    if give_up_counter > 10 {
        println!("Can't move anymore");
        return (true, give_up_counter);
    }

    if x1 == new_x1 && x2 == new_x2 {
        give_up_counter += 1;
        (false, give_up_counter)
    } else {
        give_up_counter = 0;
        (false, give_up_counter)
    }
}
