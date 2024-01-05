/// Bubble Sort
fn bubble_sort(mut nums: Vec<i32>) -> Vec<i32> {
    if nums.is_empty() {
        return vec![];
    }

    let n = nums.len();
    for i in 0..n {
        // 提前退出标志
        let mut flag = false;
        for j in 0..n - i - 1 {
            if nums[j] > nums[j + 1] {
                // 交换数据
                let tmp = nums[j];
                nums[j] = nums[j + 1];
                nums[j + 1] = tmp;
                flag = true;
            }
        }

        // 没有发生数据交换，提前退出
        if !flag {
            break;
        }
    }
    nums
}

fn main() {
    let nums = vec![4, 5, 6, 1, 2, 3];
    println!("{:?}", bubble_sort(nums));
}
