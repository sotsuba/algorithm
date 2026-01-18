#include <iostream>

int main() {
    int num_queries;
    std::cin >> num_queries;
    
    while (num_queries--) {
        unsigned short a, b;
        std::cin >> a >> b;

        unsigned short number = 0;
        for (int i = 0; i <= 15; i++) {
            unsigned short is_on = (1 << i);

            if (i <= 4) 
                is_on &= (a ^ b);
            else if (i <= 9) 
                is_on &= (a & b);
            else 
                is_on &= (a | b);
            number |= (bool(is_on) << i);
        }
        std::cout << number << '\n';
    }
}