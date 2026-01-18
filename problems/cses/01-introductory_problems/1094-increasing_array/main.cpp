#include <iostream>

int main() {
    int n;
    std::cin >> n;

    int prev = -1;
    long long res = 0;
    for (int 
        i = 0 ; i < n; i++) {
        int cur;
        std::cin >> cur;
        if (prev == -1 || cur >= prev) 
            prev = cur;
    
        res += (prev - cur);
    }

    std::cout << res;
}

// https://cses.fi/problemset/task/1094/
