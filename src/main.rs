use rayon::prelude::*;
use std::collections::HashMap;
use std::sync::{Arc, Mutex}; // 引入线程安全的 Mutex 和 Arc

fn main() {
    // 定义限制和阈值
    let limit: usize = 1_000_000_000; // n 的上限 10^9
    let threshold: usize = 1000; // 至少 1000 种表示方法
    let max_square = (limit as f64).sqrt() as usize;

    // 生成所有平方数
    let squares: Vec<usize> = (1..=max_square).map(|x| x * x).collect();

    // 使用线程安全的 HashMap
    let count_map = Arc::new(Mutex::new(HashMap::new()));

    // 使用并行迭代计算
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

                // 使用 Mutex 保护对 count_map 的访问
                let mut map = count_map.lock().unwrap();
                *map.entry(n).or_insert(0) += 1;
            }
        }
    });

    // 找出符合条件的 n
    let results: Vec<_> = {
        let map = count_map.lock().unwrap();
        map.iter()
            .filter(|&(_, &count)| count > threshold)
            .map(|(&n, _)| n)
            .collect()
    };

    // 输出结果
    println!("找到 {} 个符合条件的数：", results.len());
    for n in results {
        println!("{}", n);
    }
}
