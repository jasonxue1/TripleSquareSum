import numpy as np
from tqdm import tqdm

def find_numbers(limit, threshold):
    max_square = int(np.sqrt(limit)) + 1
    squares = np.array([i**2 for i in range(1, max_square)])

    print(f"生成所有平方数，共计 {len(squares)} 个...")
    counts = np.zeros(limit, dtype=np.int32)

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

    results = np.where(counts > threshold)[0]
    return results

if __name__ == "__main__":
    limit = 10**9
    threshold = 1000

    print(f"固定参数运行：limit={limit}, threshold={threshold}")
    result = find_numbers(limit, threshold)

    print(f"找到 {len(result)} 个符合条件的数：")
    print(result)
