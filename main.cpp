#include <iostream>
#include <string> 

int main() {
    std::ios::sync_with_stdio(false);
    std::cin.tie(NULL);

    std::string s;
    std::cin >> s;

    int res = s.empty() ? 0 : 1;
    int cur = 1;
    for (int i = 1; i < s.length(); i++) {
        cur = (s[i] == s[i - 1]) ? cur + 1 : 1;
        res = std::max(res, cur);
    }

    std::cout << res << '\n';
}