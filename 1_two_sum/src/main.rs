use std::collections::HashMap;
fn main() {
    let nums = vec![2, 7, 11, 15];
    let target = 9;

    let result = two_sum_efficient(nums, target);

    println!("{:?}", result);
}

pub fn two_sum(nums: Vec<i32>, target: i32) -> Vec<i32> {
    let mut output: Vec<i32> = vec![];

    for (index_1, num_1) in nums.iter().enumerate() {
        for (index_2, num_2) in nums.iter().enumerate() {
            if num_1 + num_2 == target && index_1 != index_2 {
                output.push(index_1.try_into().unwrap());
                output.push(index_2.try_into().unwrap());
                return output;
            }
        }
    }
    output
}

pub fn two_sum_efficient(nums: Vec<i32>, target: i32) -> Vec<i32> {
    let mut seen: HashMap<i32, usize> = HashMap::new();

    for (index, &num) in nums.iter().enumerate() {
        let complement = target - num;

        if let Some(&mat) = seen.get(&complement) {
            return vec![mat as i32, index as i32];
        }
        seen.insert(num, index);
    }
    panic!("No solution found!")
}
