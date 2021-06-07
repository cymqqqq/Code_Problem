pub fn sub_arr_sum(nums: &mut Vec<i32>) -> i32 {
    let mut ptr1: usize = 0;
    let mut ptr2: usize = ptr1 + 1;
    let end: usize = nums.len() - 1;
    let mut sum: i32 = 0;
    while ptr1 <= ptr2 && ptr2 <= end - 1 {
        if nums[ptr1] > 0 && nums[ptr2] > 0{
            sum = nums[ptr1] + nums[ptr2];
            ptr2 += 1;
            if nums[ptr2] > 0 {
                sum += nums[ptr2];
            }
        }
        ptr1 += 1;
        ptr2 += 1;
    }
    sum
    
}
fn main() {
    let mut nums1 = vec![8, -1, 3, 4];
    println!("{:?}",sub_arr_sum(&mut nums1));
}
