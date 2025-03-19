pub fn build_array_from_permutation(nums: &Vec<i32>) -> Vec<i32> {
    let mut ans: Vec<i32> = vec![0; nums.len()];
    for i in 0..nums.len() {
        let num = usize::try_from(nums[i]).expect("distinct numbers");
        let num = nums.get(num).expect("num is index");
        ans[i] = *num;
    }
    ans
}
