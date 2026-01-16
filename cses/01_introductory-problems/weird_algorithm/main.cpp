#include <iostream>

void weird_algo(long long n) {
    std::cout << n << " ";
    if (n == 1)
        return;   

    long long next_number = n & 1 ? (3 * n + 1) : (n / 2);
    weird_algo(next_number);
}

int main() {
    long long n;
    std::cin >> n;
    weird_algo(n);
}

// https://cses.fi/problemset/task/1068
