#include<bits/stdc++.h>
#include<chrono>
using namespace std;
using namespace std::chrono;

unsigned long long pow10(unsigned exponent) {
    unsigned long long result = 1;
    for (unsigned i = 0; i < exponent; ++i) {
        result *= 10;
    }
    return result;
}

int find_min_combi(int num_dials, unsigned long long start, unsigned long long end) {
    int total = 0;
    for(int i = 0; i < num_dials; i++) {
        unsigned long long start_digit = (start / pow10(i)) % 10;
        unsigned long long end_digit = (end / pow10(i)) % 10;

        int forward_steps = (int)abs((long long)(end_digit - start_digit));
        int backward_steps = 10 - forward_steps;
        total += min(forward_steps, backward_steps);
    }
    return total;
}

int main() {
    ios_base::sync_with_stdio(false);
    cin.tie(NULL);

    int t = 1000, n;
    unsigned long long s, e;
    cin >> n >> s >> e;

    // TIMER START
    auto start_t = high_resolution_clock::now();
    while(t--) {
        find_min_combi(n, s, e);
    }		
    cout << find_min_combi(n,s,e) << endl;

    // TIMER END
    auto stop_t = high_resolution_clock::now();
    // Calculate Time End - Time Start
    auto duration = duration_cast<microseconds>(stop_t - start_t);
    cout << "Time taken by function: " << duration.count() << " us" << endl;
    return 0;
}