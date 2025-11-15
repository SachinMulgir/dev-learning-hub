pub fn _check(nums: Vec<i32>) -> bool {
    let mut min_idx_vec: Vec<usize> = Vec::new();

    let mut min_num = nums[0];
    for i in 0..nums.len() {
        if nums[i] <= min_num {
            min_num = nums[i];
            min_idx_vec.push(i);
        }
    }

    for min_idx in min_idx_vec {
        println!(
            "nums_vec: {:?}\nmin_idx:{} && min_val:{}",
            nums, min_idx, min_num
        );

        for i in 0..nums.len() - 1 {
            let nxt_idx = (i + min_idx) % nums.len();
            let nxt_idx_2 = (i + min_idx + 1) % nums.len();

            println! {"next_idx = {} && next_idx_2 = {} -> val1 {} > val2 {}",nxt_idx,nxt_idx_2, nums[nxt_idx], nums[nxt_idx_2]};
            if nums[nxt_idx] > nums[nxt_idx_2] {
                return false;
            }
        }
    }
    true
}

pub fn check_2(nums: Vec<i32>) -> bool {
    println!("vector : {:?}", nums);
    for i in 0..nums.len() {
        if check_assc(&nums, i) {
            return true;
        }
    }
    false
}

pub fn check_assc(nums: &Vec<i32>, strt_idx: usize) -> bool {
    for i in 0..nums.len() - 1 {
        let nxt_idx = (i + strt_idx) % nums.len();
        let nxt_idx_2 = (i + strt_idx + 1) % nums.len();

        if nums[nxt_idx] > nums[nxt_idx_2] {
            return false;
        }
    }
    true
}
