/**
 * @param {number[]} nums
 * @return {boolean}
 */
const containsDuplicate = nums=>{
   const hash = {};

   for(let i = 0 ; i<nums.length; i++){
        const val = nums[i]
        if (hash[val] != null || hash[val] !=  undefined) 
            return true
        hash[val] = val
   }
   return false
    
};
