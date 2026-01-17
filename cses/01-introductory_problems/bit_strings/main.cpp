#include <iostream>
#include <vector>
#include <algorithm>
#define ll long long 

using std::cin, std::cout, std::vector;

int main() {
    int n; 
    cin >> n;
    
    ll ans = 1;
    const int MOD = 1e9 + 7;

    for (int i = 0; i < n; i++) 
        (ans *= 2LL) %= MOD;
    
    cout << ans << '\n';
}