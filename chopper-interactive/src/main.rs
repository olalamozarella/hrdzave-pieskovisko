use macroquad::prelude::*;
use csv::Reader;
use std::collections::{HashMap, HashSet, VecDeque};
use std::error::Error;
use std::fmt;

// ########## custom types ##########
#[derive(Clone)]
pub struct Cities {
    pub names: Vec<String>,
    pub populations: Vec<u32>,
    pub coords: Vec<(f32, f32)>
}

pub type Distances = Vec<Vec<u32>>;

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

#[derive(Clone)]
enum CityStatus {
    NotConnected,
    ConnectedOne,
    ConnectedTwo,
    Selected,
    Invalid
}

pub type Connections = HashMap<usize, HashSet<usize>>;

// ########## custom functions ##########
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

// scoring and penalties
const MAX_DISTANCE: u32 = 1500;
const PENALTY_DISTANCE: u32 = 500;
const PENALTY_CITY91: u32 = 100000;
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
    path.score = if total_population > penalty { total_population - penalty} else { 0 };
}

pub fn path_to_connections(path: &Path) -> Connections
 {
    let mut result: Connections = HashMap::new();
    for key in 0..139 {
        result.insert(key, HashSet::new());
    }

    for i in 0..path.city_indexes.len()-2 {
        result.entry(path.city_indexes[i]).or_insert_with(HashSet::new).insert(path.city_indexes[i+1]);
        result.entry(path.city_indexes[i+1]).or_insert_with(HashSet::new).insert(path.city_indexes[i]);
    }

    result
}

pub fn connections_to_path(connections: &Connections) -> Path {
    // traverse the connections, start in city 91
    let mut visited: HashSet<usize> = HashSet::new();
    let mut indexes: Vec<usize> = Vec::new();

    let mut next_neighbor = Some(91);
    loop {
        if next_neighbor.is_none() {
            break;
        }
        let city = next_neighbor.unwrap();
        next_neighbor = None;
        indexes.push(city);
        visited.insert(city);

        if let Some(neighbors) = connections.get(&city) {
            for &neighbor in neighbors {
                if !visited.contains(&neighbor) {
                    visited.insert(neighbor);
                    next_neighbor = Some(neighbor);
                    break; // to avoid adding two neighbors
                } else if neighbor == 91 {
                    indexes.push(neighbor);
                }
            }
        }
    }

    let result = Path {
        city_indexes: indexes,
        population: 0,
        distance: 0,
        score: 0,
    };
    result
}

// scaling
const X_MIN: f32 = 17.0;
const X_MAX: f32 = 22.5;
const Y_MIN: f32 = 47.5;
const Y_MAX: f32 = 49.5;
const SCREEN_WIDTH: f32 = 1800.0;
const SCREEN_HEIGHT: f32 = 800.0;
fn scale_x(x: f32) -> f32 {
    ((x - X_MIN) / (X_MAX - X_MIN)) * SCREEN_WIDTH
}
fn scale_y(y: f32) -> f32 {
    SCREEN_HEIGHT - ((y - Y_MIN) / (Y_MAX - Y_MIN)) * SCREEN_HEIGHT
}

fn population_to_size(population: u32) -> f32 {
    let min_population = 2000.0;
    let max_population = 200000.0;
    let min_size = 10.0;
    let max_size = 20.0;

    // Clamp population to be within the min and max bounds
    let clamped_population = (population as f32)
        .max(min_population)
        .min(max_population);

    // Normalize population within the range [0, 1]
    let normalized = (clamped_population - min_population) / (max_population - min_population);

    // Scale to the rectangle size range
    min_size + normalized * (max_size - min_size)
}

fn window_conf() -> Conf {
    Conf {
        window_title: "Clicking game".to_string(),
        window_width: SCREEN_WIDTH as i32,
        window_height: SCREEN_HEIGHT as i32,
        ..Default::default()
    }
}

fn process_connection(index1: usize, index2: usize, connections: &mut Connections, city_statuses: &mut Vec<CityStatus>) {
    // Check if the connection exists
    let exists = connections.get(&index1).map_or(false, |set| set.contains(&index2));
    if exists {
        // Remove the connection
        if let Some(set) = connections.get_mut(&index1) {
            set.remove(&index2);
        }
        if let Some(set) = connections.get_mut(&index2) {
            set.remove(&index1);
        }
    } else {
        // Add the connection
        connections.entry(index1).or_insert_with(HashSet::new).insert(index2);
        connections.entry(index2).or_insert_with(HashSet::new).insert(index1);
    }

    // recalculate the city statuses based on number of connections
    update_city_status(index1, connections, city_statuses);
    update_city_status(index2, connections, city_statuses);
}

fn update_city_status(index: usize, connections: &mut Connections, city_statuses: &mut Vec<CityStatus>) {
    // get the number of connections
    let number_of_connections = connections.get(&index).unwrap().len();
    match number_of_connections {
        0 => city_statuses[index] = CityStatus::NotConnected,
        1 => city_statuses[index] = CityStatus::ConnectedOne,
        2 => city_statuses[index] = CityStatus::ConnectedTwo,
        _ => city_statuses[index] = CityStatus::Invalid
    }

    // highlight bad cities
    match index {
        92..=102 => city_statuses[index] = CityStatus::Invalid,
        103 => {},
        104..=118 => city_statuses[index] = CityStatus::Invalid,
        119 => {},
        120..=138 => city_statuses[index] = CityStatus::Invalid,
        _ => {}
    }
}

#[macroquad::main(window_conf)]
async fn main() {
    let cities = load_cities("cities.csv").unwrap();
    println!("{:?}", cities.populations);

    let distances = load_distances("city_distance_matrix.csv").unwrap();
    let mut city_statuses: Vec<CityStatus> = vec![CityStatus::NotConnected; 139];
    let mut connections: Connections = HashMap::new();

    // Fill the button positions vector
    let mut button_positions = Vec::with_capacity(139);
    let mut button_sizes = Vec::with_capacity(139);
    for city_index in 0..cities.coords.len() {
        let (city_x, city_y) = cities.coords[city_index];
        button_positions.push((scale_x(city_x), scale_y(city_y)));
        button_sizes.push(population_to_size(cities.populations[city_index]));
    }

    // Store the clicked button indices
    let mut selected_buttons = HashSet::new();

    // preload path
    let preloaded_path = Path {
        city_indexes: vec![91, 2, 91, 59, 20, 57, 64, 88, 33, 18, 81, 31, 14, 90, 32, 84, 1, 71, 53, 119, 25, 68, 23, 76, 50, 5, 85, 11, 56, 82, 69, 39, 44, 10, 28, 42, 49, 72, 16, 80, 61, 17, 77, 13, 66, 65, 73, 30, 60, 103, 0, 79, 45, 87, 37, 75, 52, 6, 47, 27, 3, 29, 22, 34, 62, 51, 55, 67, 35, 8, 58, 24, 43, 12, 63, 4, 46, 83, 26, 78, 70, 41, 7, 21, 19, 38, 54, 40, 9, 89, 48, 86, 36, 15, 74, 91],
        //city_indexes: vec![91, 2, 59, 20, 57, 64, 88, 33, 18, 81, 31, 14, 90, 32, 84, 1, 71, 53, 25, 68, 23, 76, 50, 5, 85, 11, 56, 82, 69, 39, 44, 10, 28, 42, 49, 72, 92, 16, 80, 61, 17, 77, 13, 66, 65, 73, 30, 60, 0, 79, 45, 87, 37, 75, 52, 6, 47, 27, 3, 29, 22, 34, 62, 51, 55, 67, 35, 8, 58, 24, 43, 12, 63, 4, 46, 117, 83, 26, 78, 70, 41, 100, 7, 21, 19, 38, 54, 40, 9, 89, 48, 86, 36, 15, 74, 91],
        population: 0,
        distance: 0,
        score: 0,
    };
    connections = path_to_connections(&preloaded_path);
    for i in 0..139 {
        update_city_status(i, &mut connections, &mut city_statuses);
    }

    loop {
        clear_background(WHITE);

        // Draw buttons
        for i in 0..139 {
            let (x, y) = button_positions[i];
            let button_size = button_sizes[i];
            let rect = Rect {
                x: x as f32,
                y: y as f32,
                w: button_size as f32,
                h: button_size as f32,
            };

            // Handle button click to toggle the state
            if is_mouse_button_pressed(MouseButton::Left) && rect.contains(mouse_position().into()) {
                // if the button is already selected, deselect it
                // else select it
                if selected_buttons.contains(&i) {
                    selected_buttons.remove(&i);
                    update_city_status(i, &mut connections, &mut city_statuses);
                } else {
                    selected_buttons.insert(i);
                    city_statuses[i] = CityStatus::Selected;
                }

                // If two buttons are clicked, draw a line between them
                if selected_buttons.len() == 2 {
                    let vec: Vec<usize> = selected_buttons.clone().into_iter().collect();
                    process_connection(vec[0], vec[1], &mut connections, &mut city_statuses);
                    selected_buttons.clear();

                    // print the current score
                    let mut path: Path = connections_to_path(&connections);
                    calculate_score(&mut path, &cities, &distances);
                    println!("{}", path);
                }
            }

            // draw city based on its button state
            match city_statuses[i] {
                CityStatus::NotConnected => draw_rectangle(x as f32, y as f32, button_size as f32, button_size as f32, GRAY),
                CityStatus::ConnectedOne => draw_rectangle(x as f32, y as f32, button_size as f32, button_size as f32, BLUE),
                CityStatus::ConnectedTwo => draw_rectangle(x as f32, y as f32, button_size as f32, button_size as f32, GREEN),
                CityStatus::Selected => draw_rectangle(x as f32, y as f32, button_size as f32, button_size as f32, YELLOW),
                CityStatus::Invalid => draw_rectangle(x as f32, y as f32, button_size as f32, button_size as f32, RED),
            }
            draw_text(&i.to_string(), x + 15.0, y + button_size / 2.0, 20.0, BLACK);
        }

        for (key, set) in &connections {
            for value in set {
                let (start_x, start_y) = button_positions[*key];
                let center_offset_start = button_sizes[*key] / 2.0;
                let (end_x, end_y) = button_positions[*value];
                let center_offset_end = button_sizes[*value] / 2.0;
                draw_line(start_x + center_offset_start, start_y + center_offset_start, end_x + center_offset_end, end_y + center_offset_end, 3.0, BLACK);
            }
        }

        next_frame().await;
    }
}
