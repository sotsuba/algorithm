#include <iostream>
#include <vector>

int binary_search(std::vector<int> &arr, int target) {
    int l = 0, r = arr.size() - 1;

    while (l <= r) {
        int m = l  + (r - l) / 2;   
        if (arr[m] == target) 
            return m;

        if (target < arr[m]) 
            r = m - 1;
        
        else 
            l = m + 1;
    }

    return -1; 
}

int main() {
    std::ios_base::sync_with_stdio(false);
    std::cin.tie(NULL);

    int n, q; 
    std::cin >> n >> q;
    
    std::vector<int> arr(n, 0);
    for (auto &i: arr)
        std::cin >> i;

    while (q--) {
        int target;
        std::cin >> target;
        std::cout << binary_search(arr, target) + 1 << '\n';
    }
}