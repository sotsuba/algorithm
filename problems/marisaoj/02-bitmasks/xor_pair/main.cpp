#include <iostream>
#include <vector>
#include <algorithm>

using std::cin, std::cout, std::vector;

int main() {
    int n; 
    cin >> n;
    vector<unsigned int> arr(n);
    for (int i = 0; i < n; i++) {
        cin >> arr[i];
        arr[i] ^= (i + 1);
    }

    sort(arr.begin(), arr.end());
    int f = 0;
    unsigned int prev = -1;
    int ans = 0;
    
    for (auto cur: arr) {
        if (cur != prev) {
            ans += f * (f - 1) / 2;
            prev = cur;
            f = 1;
        }
        else {
            f++;
        }
    }
    ans += f * (f - 1) / 2;
    cout << ans << '\n';
}