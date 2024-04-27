import time

def find_min_combi(num_dials, start, end):
    total = 0
    for i in range(num_dials):
        start_digit = (start // (10 ** i)) % 10
        end_digit = (end // (10 ** i)) % 10

        forward_steps = abs(end_digit - start_digit)
        backward_steps = 10 - forward_steps
        total += min(forward_steps, backward_steps)
    return total

n, s, e = map(int, input().split(' '))

t = 1000

# Timer start
start_t = time.time()
for _ in range(t):
    find_min_combi(n, s, e)
print(find_min_combi(n,s,e))
# Timer end
stop_t = time.time()

# Calculate and print time taken in milliseconds
duration = (stop_t - start_t) * 1000  # Convert seconds to milliseconds
print("Time taken by function: {:.3f} ms".format(duration))