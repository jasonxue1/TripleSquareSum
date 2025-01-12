use rayon::prelude::*;
use std::collections::HashMap;
use std::sync::{Arc, Mutex};
use std::sync::atomic::{AtomicUsize, Ordering};
use std::time::Instant;

fn main() {
    // 定义限制和阈值
    let limit: usize = 1_000_000_000; // n 的上限 10^9
    let threshold: usize = 1000; // 至少 1000 种表示方法
    let max_square = (limit as f64).sqrt() as usize;

    // 生成所有平方数
    let squares: Vec<usize> = (1..=max_square).map(|x| x * x).collect();

    // 使用线程安全的 HashMap 和计数器
    let count_map = Arc::new(Mutex::new(HashMap::new()));
    let progress_counter = Arc::new(AtomicUsize::new(0)); // 用于记录已处理的 a 值
    let total_tasks = squares.len(); // 总任务数

    let start_time = Instant::now(); // 记录开始时间

    // 使用并行迭代计算
    squares.par_iter().enumerate().for_each(|(idx, &a)| {
        let local_map = Arc::clone(&count_map);
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
                let mut map = local_map.lock().unwrap();
                *map.entry(n).or_insert(0) += 1;
            }
        }

        // 更新进度计数器
        progress_counter.fetch_add(1, Ordering::SeqCst);

        // 每处理一定数量的任务输出进度
        if idx % 10 == 0 {
            let progress = progress_counter.load(Ordering::SeqCst);
            let elapsed = start_time.elapsed().as_secs();
            println!(
                "已完成 {}/{} 个任务，耗时 {} 秒...",
                progress, total_tasks, elapsed
            );
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
