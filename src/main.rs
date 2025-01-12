use rayon::prelude::*;
use std::collections::HashMap;

fn main() {
    // 定义限制和阈值
    let limit: usize = 1_000_000_000; // n 的上限 10^9
    let threshold: usize = 1000; // 至少 1000 种表示方法
    let max_square = (limit as f64).sqrt() as usize;

    // 生成所有平方数
    let squares: Vec<usize> = (1..=max_square).map(|x| x * x).collect();

    // 使用并行迭代计算
    let mut count_map = HashMap::new();
    squares.par_iter().for_each(|&a| {
        for &b in &squares {
            if b < a {
                continue;
            }
            for &c in &squares {
                if c < b {
                    continue;
                }
                let n = a + b + c;
                if n >= limit {
                    break;
                }

                // 使用原子操作更新 HashMap
                let entry = count_map.entry(n).or_insert(0);
                *entry += 1;
            }
        }
    });

    // 找出符合条件的 n
    let results: Vec<_> = count_map
        .into_iter()
        .filter(|&(_, count)| count > threshold)
        .map(|(n, _)| n)
        .collect();

    // 输出结果
    println!("找到 {} 个符合条件的数：", results.len());
    for n in results {
        println!("{}", n);
    }
}
