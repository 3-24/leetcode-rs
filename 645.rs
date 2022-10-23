impl Solution {
    pub fn find_error_nums(nums: Vec<i32>) -> Vec<i32> {
        use std::collections::HashSet;
        let l = nums.len();
        let mut count: HashSet<usize> = (1..l+1).collect();
        let mut output: Vec<i32> = [-1, -1].to_vec();
        for n in nums {
            let b = count.remove(&(n as usize));
            if !b {
                output[0] = n;
            }
        }
        output[1] = *(count.iter().next().unwrap()) as i32;
        output
    }
}
