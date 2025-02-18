def parse_line(line):
    """Parse a line into sequence number and timestamp."""
    seq_part, ts_part = line.strip().split(' ')
    seq_num = int(seq_part.split('=')[1])
    ts = int(ts_part.split('=')[1])
    return seq_num, ts

def read_file(filename):
    """Read file and return dict of sequence numbers to timestamps."""
    data = {}
    with open(filename, 'r') as f:
        for line in f:
            seq_num, ts = parse_line(line)
            data[seq_num] = ts
    return data

def compare_files():
    # List of files to compare
    files = ['brazil.txt', 'london.txt', 'ncali.txt', 'nvirginia.txt', 'tokyo.txt', 'oregon.txt']
    locations = {
        'tokyo.txt': "Tokyo",
        'brazil.txt': 'Brazil',
        'london.txt': 'London',
        'oregon.txt': 'Oregon',
        'ncali.txt': 'N. California',
        'nvirginia.txt': 'N. Virginia'
    }
    
    # Read all files
    file_data = {}
    for filename in files:
        try:
            file_data[filename] = read_file(filename)
        except FileNotFoundError:
            print(f"Warning: {filename} not found")
            continue

    # Find common sequence numbers
    seq_nums = set.intersection(*[set(data.keys()) for data in file_data.values()])

    # Track total latencies and counts for averaging
    total_latencies = {filename: 0 for filename in files}
    valid_sequences = 0

    # Compare timestamps for each sequence number
    for seq_num in sorted(seq_nums):
        timestamps = {filename: data[seq_num] for filename, data in file_data.items()}
        min_ts = min(timestamps.values())
        max_ts = max(timestamps.values())
        
        # Skip this sequence if max relative latency is >= 500ms
        if max_ts - min_ts >= 200:
            continue
            
        # Calculate and accumulate latencies relative to the minimum timestamp
        for filename, ts in timestamps.items():
            total_latencies[filename] += (ts - min_ts)
        
        valid_sequences += 1

    if valid_sequences == 0:
        print("\nNo valid sequences found with relative latencies < 500ms")
        return

    # Calculate average latencies
    avg_latencies = {filename: total_latencies[filename] / valid_sequences 
                    for filename in total_latencies}
    
    # Find the best region based on average latency
    best_region = min(avg_latencies.items(), key=lambda x: x[1])[0]
    
    print(f"\nValid sequences used for averaging: {valid_sequences}")
    print("\nAverage Latencies (ms):")
    for filename, avg_latency in avg_latencies.items():
        print(f"  {locations[filename]}: {avg_latency:.2f}")
    print(f"\nBest Region Overall: {locations[best_region]}")

if __name__ == "__main__":
    compare_files()

