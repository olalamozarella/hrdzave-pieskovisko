use std::collections::HashSet;

fn find_subset(nums: &Vec<u32>, target: u32) -> Option<Vec<usize>> {
    let mut current_subset = Vec::new();
    let mut result = None;

    // Helper function to perform backtracking
    fn backtrack(nums: &Vec<u32>, target: u32, start: usize, current_sum: u32, current_subset: &mut Vec<usize>, result: &mut Option<Vec<usize>>) {
        if current_sum == target {
            *result = Some(current_subset.clone());
            return;
        }
        if current_sum > target || start >= nums.len() {
            return;
        }

        for i in start..nums.len() {
            current_subset.push(i);
            backtrack(nums, target, i + 1, current_sum + nums[i], current_subset, result);
            if result.is_some() {
                return;
            }
            current_subset.pop();
        }
    }

    // Start backtracking from the beginning of the Vec
    backtrack(nums, target, 0, 0, &mut current_subset, &mut result);
    result
}

fn main() {
    let nums: Vec<u32> = vec![423737, 236563, 94718, 86329, 85985, 82336, 69785, 60817, 58278, 57431, 52987, 44502, 43936, 42044, 40360, 39195, 37574, 36279, 35367, 33509, 33060, 30806, 30000, 28159, 26617, 25235, 24670, 24513, 24370, 23930, 23639, 23246, 22947, 22221, 21741, 21527, 21391, 21343, 21331, 19392, 19261, 18995, 18933, 18833, 18018, 17773, 16558, 16365, 16000, 15618, 15020, 15013, 14673, 14511, 13466, 13142, 12608, 12428, 12358, 12290, 12143, 11708, 11416, 11279, 10874, 10823, 10491, 10291, 10198, 10191, 9544, 9525, 9493, 9113, 8812, 8536, 8289, 8153, 8135, 8063, 8061, 8031, 7991, 7854, 7819, 7748, 7693, 7637, 7615, 7605, 7540, 7448, 7314, 7286, 7203, 7032, 6820, 6701, 6503, 6378, 6264, 6120, 5998, 5800, 5732, 5619, 5562, 5498, 5420, 5320, 5208, 5103, 5048, 4912, 4864, 4756, 4621, 4552, 4440, 4339, 4268, 4230, 4180, 4140, 4109, 4078, 4020, 3985, 3928, 3870, 3801, 3758, 3721, 3675, 3622, 3550, 3498, 3446, 3384];
    // let target = 2764534;
    let target = 2763991;

    match find_subset(&nums, target) {
        Some(indices) => {
            println!("Found subset at indices: {:?}", indices);


            let indices_set: HashSet<usize> = indices.into_iter().collect();
            let mut missing_numbers = Vec::new();
            for i in 0..139 {
                if !indices_set.contains(&(i as usize)) {
                    missing_numbers.push(i);
                }
            }
            println!("Skipped cities: {:?}", missing_numbers);
        }
        None => println!("No subset found."),
    }
}
