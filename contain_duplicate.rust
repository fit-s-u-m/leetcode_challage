use::std::collections::HashMap;
impl Solution {
    pub fn contains_duplicate(nums: Vec<i32>) -> bool {
        let mut hash = HashMap::new();
        for num in nums.iter(){
            match hash.get(num){
                Some(x)=>return true,
                None => hash.insert(num,num)
            };
        }
        return false;
    }
}
