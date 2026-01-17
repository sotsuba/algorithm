#include <iostream>
#include <vector>
#include <algorithm>

using std::cin, std::cout, std::vector;

int main() {
    int n; 
    cin >> n;
    vector<int> arr(n);
    for (auto &it: arr) cin >> it;
    sort(arr.begin(), arr.end());

    
    int prev = -36;
    
    int number_of_distinct_values = 0;
    for (int cur: arr) {
        if (cur != prev) {
            number_of_distinct_values++;
            prev = cur;
        }
    }
    
    cout << number_of_distinct_values << '\n';
}