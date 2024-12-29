// Implementation of methods for the 'solution' struct
// Implementations define methods for a particular struct,enum or trait
impl Solution {

    pub fn two_sum(nums: Vec<i32>, target: i32) -> Vec<i32> {
        
        for i in 0..nums.len() {
            for j in 0..nums.len() {

                // check if target is present
                if (nums[i] + nums[j] == target) && (i != j) {

                    let indices : Vec<i32> = vec![i as i32,j as i32]; // Create a vector of indices, type cast to integers
                    return indices; 
                }
            }
        }
        return vec![]; // Return empty vector if no sulution is found
    }
}