#include <iostream>

int main() {
    unsigned int number = 0;
    int num_query;
    
    std::cin >> num_query;
    
    while (num_query--) {
        int type, k;
        std::cin >> type >> k;

        if (type == 1) 
            number |= (1 << k);
        else if (type == 2) 
            number &= ~(1 << k);
        else {
            number ^= (1 << k);
        }
        std::cout << number << '\n';
    }
}