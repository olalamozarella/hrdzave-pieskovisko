use std::collections::HashSet;
use std::error::Error;
use std::fmt;
use std::thread;
use csv::Reader;
use rand::Rng;
use rand::rngs::ThreadRng;
use rand::seq::SliceRandom;

// constants
pub const ITERATION_COUNT: u32 = 50000;
const GENERATION_SIZE: usize = 100;
const BOUNDARY_ELITE: usize = 20;
const BOUNDARY_MUTATE: usize = 40;
const BOUNDARY_RANDOM: usize = 50;
const MAX_DISTANCE: u32 = 1500;
const PENALTY_DISTANCE: u32 = 500;
const PENALTY_CITY91: u32 = 100000;
const PENALTY_BADCITY: u32 = 10000;

const MUTATION_CHANCE_FIRSTCITY: u8 = 5;
const MUTATION_CHANCE_LASTCITY: u8 = 5;
const MUTATION_CHANCE_ADDONE: u8 = 50;
const MUTATION_CHANCE_REMOVEONE: u8 = 50;
const MUTATION_CHANCE_REORDERONE: u8 = 20;

// custom types
#[derive(Clone)]
pub struct Cities {
    pub names: Vec<String>,
    pub populations: Vec<u32>,
    pub coords: Vec<(f32, f32)>
}

#[derive(Clone)]
pub struct Path {
    pub city_indexes: Vec<usize>,
    pub population: u32,
    pub distance: u32,
    pub score: u32,
}
impl fmt::Display for Path {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(
            f,
            "Score: {}, Population: {}, Distance: {}, Cities: {:?}",
            self.score, self.population, self.distance, self.city_indexes
        )
    }
}

pub type Distances = Vec<Vec<u32>>;
pub type Generation = [Path; GENERATION_SIZE];

pub fn load_cities(filename: &str) -> Result<Cities, Box<dyn Error>> {
    let mut rdr = Reader::from_path(filename)?; // try to load the file, return Error if something goes wrong
    let mut names = Vec::new();
    let mut populations = Vec::new();
    let mut coords = Vec::new();
    for (_index, result) in rdr.records().enumerate() {
        let record = result?;
        names.push(record[1].to_string());
        let population: u32 = record[2].parse()?;
        populations.push(population);

        let coord1: f32 = record[3].parse()?;
        let coord2: f32 = record[4].parse()?;
        coords.push((coord1, coord2));
    }
    Ok(Cities { names: names, populations: populations, coords: coords})
}

pub fn load_distances(filename: &str) -> Result<Distances, Box<dyn Error>> {
    let mut rdr = Reader::from_path(filename)?;
    let mut distances = Vec::new();
    for result in rdr.records() {
        let record = result?;
        let row: Vec<u32> = record.iter()
                .skip(1) // Skip the first column
                .map(|value| value.parse().unwrap()) // parse all other columns as numbers
                .collect::<Vec<u32>>(); // put results into vector
        distances.push(row);
    }
    Ok(distances)
}

fn generate_random_path_with_rng(rng: &mut ThreadRng) -> Path {
    let size = 10; // Change this to the desired size of the vector
    let range = 0..=138; // 138 is included due to '=' sign
    let random_numbers: Vec<usize> = (0..size).map(|_| rng.gen_range(range.clone())).collect();

    Path {
        city_indexes: random_numbers.clone(),
        population: 0,
        distance: 0,
        score: 0,
    }
}

pub fn generate_random_path() -> Path {
    let mut rng = rand::thread_rng();
    generate_random_path_with_rng(&mut rng)
}

pub fn calculate_score(path: &mut Path, cities: &Cities, distances: &Distances) {
    let mut total_distance = 0;
    for i in 0..path.city_indexes.len()-1 {
        total_distance += distances[path.city_indexes[i]][path.city_indexes[i+1]]
    }
    path.distance = total_distance;

    let unique_cities: HashSet<usize> = path.city_indexes.iter().copied().collect(); // Convert &usize to usize
    let mut total_population = 0;
    for item in unique_cities {
        total_population += cities.populations[item]
    }
    path.population = total_population;

    // apply penalty -1000 point for each extra kilometer
    let mut penalty = if total_distance <= MAX_DISTANCE { 0 } else { (total_distance - MAX_DISTANCE) * PENALTY_DISTANCE };
    // apply penalty -100000 for not starting in city 91
    if path.city_indexes.first() != Some(&91) {
        penalty += PENALTY_CITY91;
    }
    // apply penalty -100000 for not finishing in city 91
    if path.city_indexes.last() != Some(&91) {
        penalty += PENALTY_CITY91;
    }
    // apply penalty for cities
    let penalty_cities =
    for i in 0..path.city_indexes.len()-1 {
        let mut penalties = 0;
        match path.city_indexes[i] {
            92..=102 => penalties += 1,
            103 => {},
            104..=118 => penalties += 1,
            119 => {},
            120..=138 => penalties += 1,
            _ => {}
        };
        penalty += penalties * PENALTY_BADCITY;
    };

    path.score = if total_population > penalty { total_population - penalty} else { 0 };
}

pub fn calculate_scores(generation: &mut Generation, cities: &Cities, distances: &Distances) {
    for path in generation.iter_mut() {
        calculate_score(path, cities, distances);
    }

    generation.sort_by_key(|path| std::cmp::Reverse(path.score));
}

fn mutate(parent: &Path, rng: &mut ThreadRng) -> Path {
    let mut offspring = parent.clone();

    if offspring.city_indexes.len() < 2 {
        return offspring; // Nothing to swap if the vector has fewer than 2 elements
    }

    // apply mutations with a random chance
    let mut random_chance: u8 = rng.gen_range(0..100);
    if random_chance < MUTATION_CHANCE_FIRSTCITY {
        if parent.city_indexes.first() != Some(&91) {
            offspring.city_indexes.insert(0, 91);
        }
    }

    random_chance = rng.gen_range(0..100);
    if random_chance < MUTATION_CHANCE_LASTCITY {
        if parent.city_indexes.last() != Some(&91) {
            offspring.city_indexes.push(91);
        }
    }

    random_chance = rng.gen_range(0..100);
    if random_chance < MUTATION_CHANCE_ADDONE {
        // Add a random city to random index - but only if it is not visited
        let visited_cities: HashSet<usize> = offspring.city_indexes.iter().cloned().collect();
        let unvisited_cities: Vec<usize> = (0..139).filter(|n| !visited_cities.contains(n)).collect();
        let insertion_index = rng.gen_range(0..offspring.city_indexes.len());
        let mutation_value = unvisited_cities.choose(rng).unwrap();
        offspring.city_indexes.insert(insertion_index, *mutation_value);
    }

    random_chance = rng.gen_range(0..100);
    if random_chance < MUTATION_CHANCE_REMOVEONE {
        // Remove a random city from random index
        let index = rng.gen_range(0..offspring.city_indexes.len());
        offspring.city_indexes.remove(index);
    }

    random_chance = rng.gen_range(0..100);
    if random_chance < MUTATION_CHANCE_REORDERONE {
        // Take a random city and put it somewhere else
        let pick_from_index = rng.gen_range(0..offspring.city_indexes.len()); // First random index
        let mut insert_into_index = rng.gen_range(0..offspring.city_indexes.len()); // Second random index
        while pick_from_index == insert_into_index {
            insert_into_index = rng.gen_range(0..offspring.city_indexes.len());
        }
        let picked_number = offspring.city_indexes.remove(pick_from_index); // Remove the picked number
        offspring.city_indexes.insert(insert_into_index, picked_number);
    }

    offspring
}

fn crossover(parent1: &Path, parent2: &Path, rng: &mut ThreadRng) -> Path {
    // Determine the size of the offspring (let's just take the maximum size of the two parents)
    let offspring_size = rng.gen_range(1..=parent1.city_indexes.len().max(parent2.city_indexes.len()));
    let mut offspring = Vec::with_capacity(offspring_size);

    let random_choice = rng.gen_range(0..2);
    match random_choice {
        0 => {
            // Choose a random crossover point
            // Take the first part from parent1 and the second part from parent2
            let crossover_point = rng.gen_range(0..offspring_size.min(parent1.city_indexes.len()).min(parent2.city_indexes.len()));
            offspring.extend_from_slice(&parent1.city_indexes[..crossover_point.min(parent1.city_indexes.len())]);
            offspring.extend_from_slice(&parent2.city_indexes[..(offspring_size - offspring.len()).min(parent2.city_indexes.len())]);
        },
        1 => {
            // fill the offspring from slices from either parent
            while offspring.len() < offspring_size {
                // if we reached the end of parent1, take the rest from parent2
                let remaining_from_p1 = parent1.city_indexes.len() as isize - offspring.len() as isize;
                if remaining_from_p1 <= 0 {
                    offspring.extend_from_slice(&parent2.city_indexes[offspring.len()..]);
                    break;
                }
                // if we reached the end of parent2, take the rest from parent1
                let remaining_from_p2 = parent2.city_indexes.len() as isize - offspring.len() as isize;
                if remaining_from_p2 <= 0 {
                    offspring.extend_from_slice(&parent1.city_indexes[offspring.len()..]);
                    break;
                }
                if rng.gen_bool(0.5) {
                    // take slice from parent1
                    let slice_size = rng.gen_range(1..=remaining_from_p1 as usize);
                    offspring.extend_from_slice(&parent1.city_indexes[offspring.len()..offspring.len()+slice_size]);
                } else {
                    // take slice from parent2
                    let slice_size = rng.gen_range(1..=remaining_from_p2 as usize);
                    offspring.extend_from_slice(&parent2.city_indexes[offspring.len()..offspring.len()+slice_size]);
                }
            }
        },
        _ => unreachable!(),
    }

    Path {
        city_indexes: offspring.clone(),
        population: 0,
        distance: 0,
        score: 0,
    }
}

pub fn do_x_iterations(generation: &mut Generation, cities: &Cities, distances: &Distances, iterations: usize) -> Path {
    let mut rng = rand::thread_rng();

    calculate_scores(generation, &cities, &distances);

    let boundary_elite = GENERATION_SIZE * BOUNDARY_ELITE / 100;
    let boundary_mutate = GENERATION_SIZE * BOUNDARY_MUTATE / 100;
    let boundary_random = GENERATION_SIZE * BOUNDARY_RANDOM / 100;
    for iteration in 1..=iterations {
        // order paths by score
        // top 20 paths remain unchanged
        // next 20 paths mutate slightly (reorder single cities, or replace segments)
        // create 10 random paths
        // create 50 crossovers (pick random pairs from the 50)

        for i in boundary_elite..boundary_mutate {
            generation[i] = mutate(&generation[i], &mut rng);
        }
        for i in boundary_mutate..boundary_random {
            generation[i] = generate_random_path_with_rng(&mut rng);
        }

        // single-threaded solution
        for i in boundary_random..GENERATION_SIZE {
            let parent1_index = rng.gen_range(0..boundary_random);
            let parent2_index = rng.gen_range(0..boundary_random);
            let parent1 = &generation[parent1_index];
            let parent2 = &generation[parent2_index];
            generation[i] = crossover(&parent1, &parent2, &mut rng);
        }

        // evaluate this generation
        calculate_scores(generation, &cities, &distances);
    }

    generation[0].clone()
}

pub fn battle_royale() {
    let cities = load_cities("cities.csv").unwrap();
    let distances = load_distances("city_distance_matrix.csv").unwrap();
    let mut rng = rand::thread_rng();

    println!("Starting battle..");
    let mut golden_generation: Generation = std::array::from_fn(|_| {
        generate_random_path_with_rng(&mut rng)
    });

    let runs = 50;
    for run in 0..runs {
        let mut generation: Generation = std::array::from_fn(|_| {
            generate_random_path_with_rng(&mut rng)
        });
        for tens_thousand in 0..ITERATION_COUNT/10000 {
            let _best = do_x_iterations(&mut generation, &cities, &distances, 10000);
            println!("{}", _best);
        }
        golden_generation[run] = generation[0].clone();
        println!("CANDIDATE {}: {}", run, golden_generation[run]);
    }

    println!("Battle royale begins");
    let best = do_x_iterations(&mut golden_generation, &cities, &distances, 10000);
    println!("Best of the best: {}", best);
}
