impl Solution {
    pub fn check_subarray_sum(nums: Vec<i32>, k: i32) -> bool {
        use std::collections::HashSet;

        let mut set: HashSet<i32> = HashSet::new();
        set.insert(0);
        let mut prev = 0;
        let mut sum = 0;
        for v in nums {
            sum = (sum + v) % k;
            if (prev != sum && set.contains(&sum)){
                return true;
            }
            prev = if set.contains(&sum) {-1} else {sum};
            set.insert(sum);
        }
        false
    }
}
