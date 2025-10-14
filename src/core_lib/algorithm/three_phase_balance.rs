use std::collections::HashSet;

/// 三相平衡算法工具
/// 用于将一组数字分成三组，使三组的和尽可能平衡
pub struct ThreePhaseBalancer {
    /// 输入的数值数组
    nums: Vec<u32>,
    /// 分组结果
    groups: Vec<Vec<u32>>,
    /// 各组的和
    sums: Vec<u32>,
    /// 总和
    total: u32,
    /// 平均值
    average: f64,
    /// 最大迭代次数
    max_iterations: u32,
}

impl ThreePhaseBalancer {
    /// 创建三相平衡算法实例
    /// - nums: 需要分组的数值数组
    pub fn new(nums: &[u32]) -> Self {
        let mut instance = Self {
            nums: nums.to_vec(),
            groups: vec![vec![], vec![], vec![]],
            sums: vec![0, 0, 0],
            total: 0,
            average: 0.0,
            max_iterations: 1000,
        };
        instance.initialize();
        instance
    }

    /// 初始化分组和计算总和
    fn initialize(&mut self) {
        // 检查边界情况
        if self.nums.is_empty() {
            return;
        }

        // 1. 初始化分配：轮询分配数字到三个组
        for (i, &num) in self.nums.iter().enumerate() {
            self.groups[i % 3].push(num);
        }

        // 计算初始和
        self.sums = self.groups.iter().map(|g| g.iter().sum()).collect();
        self.total = self.sums.iter().sum();
        self.average = self.total as f64 / 3.0;
    }

    /// 执行平衡算法
    pub fn balance(&mut self) -> &Vec<Vec<u32>> {
        // 2. 迭代优化
        let mut visited = HashSet::new();

        for _ in 0..self.max_iterations {
            // 计算当前差值
            let diff: Vec<f64> = self.sums.iter().map(|&s| s as f64 - self.average).collect();

            // 3. 终止条件：所有组和与平均值的差值都小于1
            if diff.iter().all(|&d| d.abs() < 1.0) {
                break;
            }

            // 4. 寻找调整候选：最大和组与最小和组
            let (max_idx, min_idx) = self.find_max_min_groups();

            // 5. 尝试从最大和组移动一个数字到最小和组
            if let Some((idx, num)) = self.find_best_move(max_idx, min_idx, &diff) {
                // 6. 执行最佳移动
                self.execute_move(max_idx, min_idx, idx, num);

                // 检查循环并记录状态
                let state = (self.sums[0], self.sums[1], self.sums[2]);
                if !visited.insert(state) {
                    break; // 遇到循环状态，提前终止
                }
            } else {
                break; // 没有可改善的移动
            }
        }

        &self.groups
    }

    /// 查找当前和最大和最小的组索引
    fn find_max_min_groups(&self) -> (usize, usize) {
        let max_idx = self.sums.iter().enumerate()
            .max_by_key(|&(_, &s)| s)
            .map(|(i, _)| i)
            .unwrap();
        let min_idx = self.sums.iter().enumerate()
            .min_by_key(|&(_, &s)| s)
            .map(|(i, _)| i)
            .unwrap();
        (max_idx, min_idx)
    }

    /// 寻找最佳移动方案
    fn find_best_move(&self, max_idx: usize, min_idx: usize, diff: &[f64]) -> Option<(usize, u32)> {
        let mut best_move: Option<(usize, u32)> = None;
        let mut best_improvement = f64::MIN;

        for (i, &num) in self.groups[max_idx].iter().enumerate() {
            // 计算移动后的新和差
            let new_sum_max = self.sums[max_idx] - num;
            let new_sum_min = self.sums[min_idx] + num;

            let new_diff_max = new_sum_max as f64 - self.average;
            let new_diff_min = new_sum_min as f64 - self.average;

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

        best_move
    }

    /// 执行移动操作
    fn execute_move(&mut self, from_idx: usize, to_idx: usize, item_idx: usize, num: u32) {
        // 从源组移除
        self.groups[from_idx].remove(item_idx);
        // 添加到目标组
        self.groups[to_idx].push(num);

        // 更新和
        self.sums[from_idx] -= num;
        self.sums[to_idx] += num;
    }

    /// 获取当前的分组结果
    pub fn get_groups(&self) -> &Vec<Vec<u32>> {
        &self.groups
    }

    /// 获取各组的和
    pub fn get_sums(&self) -> &Vec<u32> {
        &self.sums
    }

    /// 获取总和
    pub fn get_total(&self) -> u32 {
        self.total
    }

    /// 获取平均值
    pub fn get_average(&self) -> f64 {
        self.average
    }

    /// 设置最大迭代次数
    pub fn set_max_iterations(&mut self, iterations: u32) {
        self.max_iterations = iterations;
    }
}

/// 便捷函数：直接进行三相平衡计算
/// - nums: 需要分组的数值数组
/// 返回值: 分组结果
pub fn three_phase_balance(nums: &[u32]) -> Vec<Vec<u32>> {
    let mut balancer = ThreePhaseBalancer::new(nums);
    let groups = balancer.balance();
    groups.to_vec()
}


#[cfg(test)]
mod tests {
    use super::*;

    /// 测试三相平衡算法的基本功能
    #[test]
    fn test_three_phase_balance() {
        let test_cases = vec![
            vec![8, 7, 6, 5, 4, 3, 2, 1],
            vec![10, 20, 30, 40, 50, 60],
            vec![100, 200, 300, 400],
            vec![1, 1, 1, 1, 1, 1, 1, 1, 1],
        ];

        for nums in test_cases {
            let groups = three_phase_balance(&nums);
            let sums: Vec<u32> = groups.iter().map(|g| g.iter().sum()).collect();
            let total: u32 = sums.iter().sum();
            let avg = total as f64 / 3.0;

            // 验证分组数量正确
            assert_eq!(groups.len(), 3, "分组数量应为3");
            
            // 验证所有元素都被正确分组
            let all_elements: Vec<u32> = groups.iter().flatten().copied().collect();
            let mut sorted_input = nums.clone();
            let mut sorted_output = all_elements.clone();
            sorted_input.sort();
            sorted_output.sort();
            assert_eq!(sorted_input, sorted_output, "所有元素应被正确分组");
            
            // 验证总和计算正确
            assert_eq!(total, nums.iter().sum(), "分组总和应等于输入总和");
            
            // 验证各组和的差异在合理范围内
            let max_diff = sums.iter().map(|&s| (s as f64 - avg).abs()).max_by(|a, b| a.partial_cmp(b).unwrap()).unwrap_or(0.0);
            // 根据数据规模设置合理的差异阈值
            let threshold = if nums.len() > 5 {
                avg * 0.1 // 对于较大的数据集，允许10%的差异
            } else {
                2.0 // 对于较小的数据集，允许2的绝对差异
            };
            assert!(max_diff <= threshold, "组和差异应在合理范围内");
        }
    }

    /// 测试结构体接口功能
    #[test]
    fn test_three_phase_balancer_struct() {
        let nums = vec![8, 7, 6, 5, 4, 3, 2, 1];
        
        // 测试结构体的创建和方法调用
        let mut balancer = ThreePhaseBalancer::new(&nums);
        
        // 测试自定义最大迭代次数
        balancer.set_max_iterations(500);
        assert_eq!(balancer.max_iterations, 500, "最大迭代次数应被正确设置");
        
        // 测试平衡算法
        let groups = balancer.balance();
        assert_eq!(groups.len(), 3, "分组数量应为3");
        
        // 测试getter方法
        let sums = balancer.get_sums();
        let total = balancer.get_total();
        let average = balancer.get_average();
        
        assert_eq!(sums.len(), 3, "组和数量应为3");
        assert_eq!(total, nums.iter().sum(), "总和应等于输入总和");
        assert_eq!(average, total as f64 / 3.0, "平均值计算应正确");
    }

    /// 测试边界情况
    #[test]
    fn test_edge_cases() {
        // 测试空数组
        let empty_array: Vec<u32> = vec![];
        let groups = three_phase_balance(&empty_array);
        assert_eq!(groups, vec![vec![], vec![], vec![]], "空数组应返回三个空组");
        
        // 测试单元素数组
        let single_element = vec![42];
        let groups = three_phase_balance(&single_element);
        assert_eq!(groups.len(), 3, "分组数量应为3");
        assert!(groups.iter().any(|g| g == &vec![42]), "单元素应被正确分组");
        
        // 测试恰好能均分的数组
        let perfectly_divisible = vec![3, 3, 3, 3, 3, 3];
        let groups = three_phase_balance(&perfectly_divisible);
        let sums: Vec<u32> = groups.iter().map(|g| g.iter().sum()).collect();
        assert!(sums.iter().all(|&s| s == 6), "能均分的数组应被均匀分组");
    }
}