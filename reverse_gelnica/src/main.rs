use csv::Reader;

fn tsp(distances: Vec<Vec<u32>>) -> u32 {
    let n = distances.len();
    let all_visited: u128 = (1 << n) - 1; // All nodes visited bitmask
    let mut dp: Vec<Vec<u32>> = vec![vec![u32::MAX / 2; n]; 1 << n];

    // Start from node 0, with only node 0 visited
    dp[1][0] = 0;

    // Iterate over all subsets of nodes
    for mask in 1..=all_visited {
        for i in 0..n {
            if (mask & (1 << i)) != 0 { // Node `i` is visited in this subset
                // Try to transition to another node `j`
                for j in 0..n {
                    if (mask & (1 << j)) == 0 { // Node `j` is not visited in this subset
                        let next_mask = mask | (1 << j);
                        dp[next_mask][j] = dp[next_mask][j].min(dp[mask][i] + distances[i][j]);
                    }
                }
            }
        }
    }

    // Find the minimal cost to return to the start node (node 0)
    let mut min_cost = u32::MAX;
    for i in 1..n {
        min_cost = min_cost.min(dp[all_visited][i] + distances[i][0]);
    }

    min_cost
}

pub fn load_distances(filename: &str) -> Vec<Vec<u32>> {
    let mut rdr = Reader::from_path(filename).unwrap();
    let mut distances = Vec::new();
    for result in rdr.records() {
        let record = result.unwrap();
        let row: Vec<u32> = record.iter()
                .skip(1) // Skip the first column
                .map(|value| value.parse().unwrap()) // parse all other columns as numbers
                .collect::<Vec<u32>>(); // put results into vector
        distances.push(row);
    }
    distances
}

fn main() {
    // let distances = vec![
    //     vec![0, 10, 15, 20, 25],
    //     vec![10, 0, 35, 25, 30],
    //     vec![15, 35, 0, 30, 5],
    //     vec![20, 25, 30, 0, 15],
    //     vec![25, 30, 5, 15, 0],
    // ];
    let distances = load_distances("distances_filtered.csv");

    let result = tsp(distances);
    println!("Minimal distance: {}", result);
}
