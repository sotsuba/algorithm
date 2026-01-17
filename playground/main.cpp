#include <iostream>

int get_msb(long long num) {
    int msb = 0;
    while (num > 1) {
        num = num >> 1;
        msb++;
    }
    return msb;
}

int get_next_msb(long long num, long long msb) {
    long long tmp = (1 << msb);
    while (num < tmp) {
        tmp = tmp >> 1;
        msb--;
    }
    return msb;
}

long long count(long long n) {
    if (n == 0) return 0;

    int msb = get_msb(n);
    msb = get_next_msb(n, msb);

    if (n == (1LL << (msb + 1)) - 1) 
        return (msb + 1) * (1 << msb);
    
    n = n - (1LL << msb);
    return (n + 1) * count(n) + msb * (1LL << (msb - 1));
}   

int main() {
    long long n;
    
    std::cin >> n;
    std::cout << count(n) << '\n';

}