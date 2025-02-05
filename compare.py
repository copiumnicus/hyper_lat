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

    if not common_keys:
        print("No matching timestamps found.")
        return

    print(f"{'EX_TS_MS':<15} {'LOCAL_TS_1':<15} {'LOCAL_TS_2':<15} {'DIFF_MS':<10}")
    print("=" * 55)

    diffs = []
    for ex_ts in common_keys:
        local_diff = data2[ex_ts] - data1[ex_ts]
        diffs.append(local_diff)
        print(f"{ex_ts:<15} {data1[ex_ts]:<15} {data2[ex_ts]:<15} {local_diff:<10}")

    # Calculate average difference
    avg_diff = sum(diffs) / len(diffs)
    print("=" * 55)
    print(f"Average DIFF_MS: {avg_diff:.2f} ms")

if __name__ == "__main__":
    if len(sys.argv) != 3:
        print(f"Usage: {sys.argv[0]} <file1> <file2>")
        sys.exit(1)

    file1, file2 = sys.argv[1], sys.argv[2]
    compare_files(file1, file2)
