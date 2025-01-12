import cupy as cp
from tqdm import tqdm

def find_numbers(limit, threshold):
    # 使用 GPU 生成所有平方数
    max_square = int(cp.sqrt(limit)) + 1
    squares = cp.array([i**2 for i in range(1, max_square)])

    print(f"生成所有平方数，共计 {len(squares)} 个...")
    
    # 初始化计数数组
    counts = cp.zeros(limit, dtype=cp.int32)
    
    # GPU 加速枚举 a, b, c 的组合
    for a in tqdm(range(len(squares)), desc="Processing"):
        for b in range(a, len(squares)):
            s_ab = squares[a] + squares[b]
            if s_ab >= limit:
                break
            for c in range(b, len(squares)):
                n = s_ab + squares[c]
                if n >= limit:
                    break
                counts[n] += 1

    # 找出所有符合条件的 n
    results = cp.where(counts > threshold)[0]
    return cp.asnumpy(results)  # 转回 CPU 进行输出

if __name__ == "__main__":
    limit = 10**9
    threshold = 1000

    print(f"固定参数运行：limit={limit}, threshold={threshold}")
    result = find_numbers(limit, threshold)

    print(f"找到 {len(result)} 个符合条件的数：")
    print(result)
