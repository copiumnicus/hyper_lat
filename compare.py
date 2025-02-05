import sys

def parse_file(file_path):
    """Parse a file into a dictionary mapping EX_TS_MS -> LOCAL_TS_MS"""
    timestamps = {}
    with open(file_path, 'r') as f:
        for line in f:
            parts = line.strip().split()
            if len(parts) == 2:
                ex_ts = int(parts[0].split('=')[1])
                local_ts = int(parts[1].split('=')[1])
                timestamps[ex_ts] = local_ts
    return timestamps

def compare_files(file1, file2):
    """Compare two timestamp files based on EX_TS_MS and calculate LOCAL_TS_MS differences"""
    data1 = parse_file(file1)
    data2 = parse_file(file2)

    common_keys = sorted(set(data1.keys()) & set(data2.keys()))

    print(f"{'EX_TS_MS':<15} {'LOCAL_TS_1':<15} {'LOCAL_TS_2':<15} {'DIFF_MS':<10}")
    print("=" * 55)

    for ex_ts in common_keys:
        local_diff = data2[ex_ts] - data1[ex_ts]
        print(f"{ex_ts:<15} {data1[ex_ts]:<15} {data2[ex_ts]:<15} {local_diff:<10}")

if __name__ == "__main__":
    if len(sys.argv) != 3:
        print(f"Usage: {sys.argv[0]} <file1> <file2>")
        sys.exit(1)

    file1, file2 = sys.argv[1], sys.argv[2]
    compare_files(file1, file2)
