use std::collections::HashMap;
impl Solution {
    pub fn two_sum(nums: Vec<i32>, target: i32) -> Vec<i32> {
        let mut h_map = HashMap::new();
         for (i, a) in nums.iter().enumerate(){
             let b = target - a;

             if let Some(&j) = h_map.get(&b){
                 return vec![j as i32,i as i32] 
             }
             h_map.insert(a,i);
         }
 
        vec![]
    }
}
