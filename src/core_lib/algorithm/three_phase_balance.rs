use std::collections::HashSet;

fn three_phase_balance(nums: &[u32]) -> Vec<Vec<u32>> {
    // 0. 检查边界情况
    if nums.is_empty() {
        return vec![vec![], vec![], vec![]];
    }

    // 1. 初始化分配：轮询分配数字到三个组
    let mut groups = vec![vec![], vec![], vec![]];
    for (i, &num) in nums.iter().enumerate() {
        groups[i % 3].push(num);
    }

    // 计算初始和
    let mut sums: Vec<u32> = groups.iter().map(|g| g.iter().sum()).collect();
    let total: u32 = sums.iter().sum();
    let avg = total as f64 / 3.0;

    // 2. 迭代优化
    let mut visited = HashSet::new();
    let max_iterations = 1000;

    for _ in 0..max_iterations {
        // 计算当前差值
        let diff: Vec<f64> = sums.iter().map(|&s| s as f64 - avg).collect();

        // 3. 终止条件：所有组和与平均值的差值都小于1
        if diff.iter().all(|&d| d.abs() < 1.0) {
            break;
        }

        // 4. 寻找调整候选：最大和组与最小和组
        let (max_idx, min_idx) = {
            let max_idx = sums.iter().enumerate()
                .max_by_key(|&(_, &s)| s)
                .map(|(i, _)| i)
                .unwrap();
            let min_idx = sums.iter().enumerate()
                .min_by_key(|&(_, &s)| s)
                .map(|(i, _)| i)
                .unwrap();
            (max_idx, min_idx)
        };

        // 5. 尝试从最大和组移动一个数字到最小和组
        let mut best_move: Option<(usize, u32)> = None;
        let mut best_improvement = f64::MIN;

        for (i, &num) in groups[max_idx].iter().enumerate() {
            // 计算移动后的新和差
            let new_sum_max = sums[max_idx] - num;
            let new_sum_min = sums[min_idx] + num;

            let new_diff_max = new_sum_max as f64 - avg;
            let new_diff_min = new_sum_min as f64 - avg;

            // 计算改善程度（减小最大差值）
            let current_max_diff = diff[max_idx].abs().max(diff[min_idx].abs());
            let new_max_diff = new_diff_max.abs().max(new_diff_min.abs());
            let improvement = current_max_diff - new_max_diff;

            // 记录最佳移动
            if improvement > best_improvement {
                best_improvement = improvement;
                best_move = Some((i, num));
            }
        }

        // 6. 执行最佳移动
        if let Some((idx, _num)) = best_move {
            // 从最大和组移除
            let num = groups[max_idx].remove(idx);
            // 添加到最小和组
            groups[min_idx].push(num);

            // 更新和
            sums[max_idx] -= num;
            sums[min_idx] += num;

            // 检查循环并记录状态
            let state = (sums[0], sums[1], sums[2]);
            if !visited.insert(state) {
                break; // 遇到循环状态，提前终止
            }
        } else {
            break; // 没有可改善的移动
        }
    }

    groups
}

// fn main() {
//     let test_cases = vec![
//         vec![8, 7, 6, 5, 4, 3, 2, 1],
//         vec![10, 20, 30, 40, 50, 60],
//         vec![100, 200, 300, 400],
//         vec![1, 1, 1, 1, 1, 1, 1, 1, 1],
//     ];
//
//     for nums in test_cases {
//         let groups = three_way_partition(&nums);
//         let sums: Vec<u32> = groups.iter().map(|g| g.iter().sum()).collect();
//         let total: u32 = sums.iter().sum();
//         let avg = total as f64 / 3.0;
//
//         println!("输入: {:?}", nums);
//         println!("分组:");
//         for (i, group) in groups.iter().enumerate() {
//             let sum = sums[i];
//             let diff = sum as f64 - avg;
//             println!("  组{}: {:?} (和={}, 差值={:.2})", i+1, group, sum, diff);
//         }
//         println!("总差异: {:.2}\n", sums.iter().map(|&s| (s as f64 - avg).abs()).sum::<f64>());
//     }
// }