#include <bits/stdc++.h>
#define ll long long 
#define FOR(i, a, b) for (int i = a; i < b; ++i)
#define ROF(i, b, a) for (int i = b; i >= a; i--)

using namespace std;

void setup();
void solve();

int main()
{
    setup();
    solve();
}

void setup() 
{
    ios_base::sync_with_stdio(false);
    
    cin.tie(NULL);
    cout.tie(NULL);
    #ifndef ONLINE_JUDGE
    freopen("data/input.txt", "r", stdin);
    #endif
}

struct Input {
    int n;
}

namespace BacktrackSolution
{
    int backtrack(vector<int> &dp, int key) 
    {
        if (key == 0) return 0;
        if (dp[key] != -1) return dp[key];
    
    
        dp[key] = INT_MAX;
    
        for (int partial = key; partial > 0; partial /= 10) {
            int digit = partial % 10;
            if (digit == 0) continue;
    
            int next_key = key - digit;
            dp[key] = min(backtrack(dp, next_key) + 1, dp[key]);
        }
        
        return dp[key];
    }

    void solve(Input in)
    {
        cerr << "BacktrackSolution: \n";

        vector<int> dp(in.n + 1, -1);
        backtrack(dp, in.n);
        cout << dp[in.n] << '\n';
    }
};


namespace DpSolution  
{
    void solve(Input in) {
        cerr << "DpSolution: \n";
        int n = in.n;

        vector<int> dp(n + 1, INT_MAX);
        dp[0] = 0;

        for (int i = 1; i <= n; i++) {
            for (int partial = i; partial > 0; partial /= 10) {
                int digit = partial % 10;
                if (digit == 0 || dp[i - digit] == INT_MAX) continue;

                dp[i] = min(dp[i], dp[i - digit] + 1);
            }
        }
        cout << dp[n] << '\n';
    }
}

namespace GreedySolution 
{
    void solve(Input in) {
        cerr << "Greedy Solution: \n";

        int n = in.n;
        int steps = 0;
        while (n) {
            int max_jump = n % 10;
            for (int tmp = n / 10; tmp > 0; tmp /= 10) {
                max_jump = max(max_jump, tmp % 10);
            }
            n -= max_jump;
            steps++;
        }
        cout << steps << '\n';
    }
}

void solve() 
{   
    int n;
    cin >> n;
    // BacktrackSolution::solve(n);
    // DpSolution::solve(n);
    GreedySolution::solve(n);
}
