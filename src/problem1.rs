pub fn ls(nums: &mut Vec<i32>) -> &mut Vec<i32> {
    for i in 0..nums.len() - 1{
        let mut end = nums.len() - 1;
        while i < end {
            if nums[i] == nums[end]{
                nums.remove(end);
           }
           end -= 1;
        }
    }
    nums
}
fn main() {
    let mut nums = vec![5,1,3,5,2,3,4,1];
    println!("{:?}",ls(&mut nums));
}
