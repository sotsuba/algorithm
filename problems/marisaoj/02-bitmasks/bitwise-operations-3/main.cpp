#include <iostream>

int seek_the_most_important_bit(int number) {
    for (int i = 31; i >= 0; i--) 
        if (number & (1 << i)) return i;
    return -1;
}

int count_bit(int number) {
    int ans = 0;
    for (int i = 0; i < 32; i++)
        ans += ((number >> i) & 1);
    return ans;
}

int main() {
    int num_queries;
    std::cin >> num_queries;
    unsigned int ans = 0;
    while (num_queries--) {
        unsigned int type, k;
        std::cin >> type;
        if (type == 1) {
            std::cin >> k;
            ans ^= k;
        }
        else if (type == 2) {
            if (ans == 0) continue;
            int idx = seek_the_most_important_bit(ans);
            ans &= ~(1 << idx);
        }
        else {
            std::cout << count_bit(ans) << '\n';
        }
    }
}