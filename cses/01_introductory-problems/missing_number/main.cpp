#include <iostream>
#include <vector>
#include <numeric>
#include <iterator>

int main() {
    long long n;
    std::cin >> n;
    long long expected_sum = n * (n + 1) / 2;

    long long missing = std::accumulate(
        std::istream_iterator<int>(std::cin), 
        std::istream_iterator<int>(), 
        expected_sum,
        std::minus<long long>()
    );
    
    std::cout << missing << "\n";
}

// https://cses.fi/problemset/task/1083
