#include<iostream>
#include<cmath>
#include<chrono>
using namespace std;
using namespace std::chrono;

int find_min_combi(int num_dials, int start, int end) {
    int total = 0;
    for(int i = 0; i < num_dials; i++) {
        int start_digit = (int)(start / pow(10, i))%10; // e.g. 3rd digit of 1234 = (1234/100)%10 = 2
        int end_digit = (int)(end / pow(10, i))%10; 
        int forward_steps;

        // Inline assembly to calculate absolute difference between end_digit and start_digit
        __asm__(
            "movl %2, %%eax\n\t"
            "subl %1, %%eax\n\t"
            "jge 0f\n\t"
            "negl %%eax\n"
            "0:"
            : "=a" (forward_steps)  // Output
            : "r" (start_digit), "r" (end_digit)  // Input
            :  // Clobbered registers
        );
        int backward_steps = 10 - forward_steps;
        total += min(forward_steps, backward_steps);
    }
    return total;
}

int main() {
    ios_base::sync_with_stdio(false);
    cin.tie(NULL);

    int t = 1000, n, s, e;
    cin >> n >> s >> e;

    // TIMER START
    auto start_t = high_resolution_clock::now();
    while(t--) {
        // cout << find_min_combi(n, s, e) << endl;
        find_min_combi(n, s, e);
    }
    // TIMER END
    auto stop_t = high_resolution_clock::now();

    // Calculate Time End - Time Start
    auto duration = duration_cast<microseconds>(stop_t - start_t);

    cout << "Time taken by function: " << (float)duration.count() / 1000 << " ms" << endl;
    return 0;
}