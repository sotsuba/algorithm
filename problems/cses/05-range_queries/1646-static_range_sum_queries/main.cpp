#include <bits/stdc++.h>
#define ll long long 
#define FOR(i, a, b) for (int i = a; i < b; ++i)
#define ROF(i, b, a) for (int i = b; i >= a; i--)

using namespace std;

class SegmentTree
{
    int n;
    vector<ll> inner;

public:
    SegmentTree(vector<int> data) : n(data.size()) 
    {
        inner.assign(2 * n, 0);

        for (int i = 0; i < n; i++)
            inner[i + n] = data[i];

        for (int i = n - 1; i > 0; i--)
            inner[i] = inner[i << 1] + inner[i << 1 | 1];
    }

    ll query(int l, int r) 
    {
        ll res = inner[r + n];
        for (l += n, r += n; l < r; l >>= 1, r >>= 1) {
            if (l&1) res += inner[l++];
            if (r&1) res += inner[--r];
        }
        return res;
    }
};

void setup() 
{
    ios_base::sync_with_stdio(false);
    cin.tie(NULL);
    cout.tie(NULL);
    
    #ifndef ONLINE_JUDGE
    freopen("data/input.txt", "r", stdin);
    #endif
}

int main()
{
    setup();
    int n, q;
    cin >> n >> q;

    vector<int> data(n);
    for (auto &i : data)
        cin >> i;

    SegmentTree tree(data);

    while (q--) {
        int l, r;
        cin >> l >> r;
        cout << tree.query(l - 1, r - 1) << '\n';
    }

    

}