use macroquad::prelude::{clear_background, next_frame, Conf, draw_text, draw_rectangle, draw_line, WHITE, BLACK, RED, BLUE};
use rand::Rng;
use rand::rngs::ThreadRng;
use std::io::{self, Write};
use std::sync::{Arc, Mutex};
use std::thread;
use std::time::Duration;

mod calculations;

const X_MIN: f32 = 17.0;
const X_MAX: f32 = 22.5;
const Y_MIN: f32 = 47.5;
const Y_MAX: f32 = 49.5;

// Screen size
const SCREEN_WIDTH: f32 = 1800.0;
const SCREEN_HEIGHT: f32 = 800.0;

fn scale_x(x: f32) -> f32 {
    ((x - X_MIN) / (X_MAX - X_MIN)) * SCREEN_WIDTH
}

fn scale_y(y: f32) -> f32 {
    SCREEN_HEIGHT - ((y - Y_MIN) / (Y_MAX - Y_MIN)) * SCREEN_HEIGHT
}

fn window_conf() -> Conf {
    Conf {
        window_title: "Scaled Drawing".to_string(),
        window_width: SCREEN_WIDTH as i32,
        window_height: SCREEN_HEIGHT as i32,
        ..Default::default()
    }
}

fn draw_path(path: &calculations::Path, cities: &calculations::Cities) {
    for i in 0..path.city_indexes.len() - 1 {
        let first_city_index = path.city_indexes[i];
        let second_city_index = path.city_indexes[i + 1];
        let (x1, y1) = cities.coords[first_city_index];
        let (x2, y2) = cities.coords[second_city_index];
        draw_line(scale_x(x1), scale_y(y1), scale_x(x2), scale_y(y2), 2.0, BLACK);

        let caption = format!("{}({})", cities.names[first_city_index].clone(), cities.populations[first_city_index].clone());
        draw_text(
            &caption,
            scale_x(x1) + 5.0,  // X offset for the text
            scale_y(y1) - 5.0,  // Y offset for the text
            10.0,               // Font size
            BLACK,              // Text color
        );
    }
}

fn main_battle() {
    calculations::battle_royale();
}

async fn main_with_ui() {
    let cities = calculations::load_cities("gelnica-chopper/cities.csv").unwrap();
    let distances = calculations::load_distances("gelnica-chopper/city_distance_matrix.csv").unwrap();

    let mut path = calculations::Path {
        city_indexes: vec![0, 1, 2, 3, 4, 5],
        population: 0,
        distance: 0,
        score: 0,
    };
    let last_result = Arc::new(Mutex::new(path));

    // do the calculations in a thread
    let last_result_clone = Arc::clone(&last_result);
    let cities_clone = cities.clone();
    thread::spawn(move || {
        let mut generation: calculations::Generation = std::array::from_fn(|_| {
            calculations::generate_random_path()
        });
        for tens_thousand in 0..calculations::ITERATION_COUNT/1000 {
            let best = calculations::do_x_iterations(&mut generation, &cities_clone, &distances, 1000);
            println!("{}", best);
            {
                let mut result = last_result_clone.lock().unwrap();
                *result = best; // Replace the old result with the new one
            }
        }
    });

    // use the main thread to display data
    let mut rng = rand::thread_rng();  // Create a random number generator
    loop {
        clear_background(WHITE);

        let path = last_result.lock().unwrap();
        draw_path(&path, &cities);

        thread::sleep(Duration::from_millis(100));
        next_frame().await;
    }
}

async fn main_interactive() {
    let cities = calculations::load_cities("gelnica-chopper/cities.csv").unwrap();
    let distances = calculations::load_distances("gelnica-chopper/city_distance_matrix.csv").unwrap();

    let mut path = calculations::Path {
        city_indexes: vec![0, 1, 2, 3, 4, 5],
        population: 0,
        distance: 0,
        score: 0,
    };
    let last_result = Arc::new(Mutex::new(path));

    // do the calculations in a thread
    let last_result_clone = Arc::clone(&last_result);
    let cities_clone = cities.clone();
    thread::spawn(move || {
        use std::io::{stdin,stdout,Write};

        loop{
            println!("Enter a vector of positive numbers (separated by commas): ");
            let _=stdout().flush();
            let mut ins=String::new();
            stdin().read_line(&mut ins).expect("Did not enter a correct string");
            println!("You typed: {}", ins);

            println!("Parsing input...");
            let numbers: Vec<usize> = ins
            .trim()  // Remove trailing newline/whitespace
            .split(',')  // Split by commas
            .filter_map(|s| s.trim().parse::<usize>().ok())  // Parse each element, skip invalid entries
            .collect();
            if numbers.len() == 0 {
                println!("Invalid input");
                continue;
            }

            println!("Calculating score...");
            let mut path = calculations::Path {
                city_indexes: numbers,
                population: 0,
                distance: 0,
                score: 0,
            };
            calculations::calculate_score(&mut path, &cities_clone, &distances);
            println!("{}", path);

            let mut result = last_result_clone.lock().unwrap();
            *result = path; // Replace the old result with the new one
        }
    });

    // use the main thread to display data
    let mut rng = rand::thread_rng();  // Create a random number generator
    loop {
        clear_background(WHITE);

        let path = last_result.lock().unwrap();
        draw_path(&path, &cities);

        thread::sleep(Duration::from_millis(100));
        next_frame().await;
    }
}

//#[macroquad::main(window_conf)]
pub fn main() {
    main_battle();
    // main_interactive().await;
    // main_with_ui().await;
}
