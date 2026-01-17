#include <iostream>
#include <vector>

int binary_search(std::vector<int> &arr, int target) {
    int l = 0, r = arr.size() - 1;
    
    int ans = -1;
    while (l <= r) {
        int m = l  + (r - l) / 2;   
        if (arr[m] < target) 
            l = m + 1;
        else {
            if (arr[m] == target) 
                ans = m;
            r = m - 1;
        }
    }

    return ans + (ans != -1); 
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
        std::cout << binary_search(arr, target) << '\n';
    }
}