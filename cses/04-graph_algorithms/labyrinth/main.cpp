#include <iostream>
#include <vector>
#include <algorithm> 
#include <queue>

using std::vector, std::cin, std::cout, std::queue;

void bfs(vector<char>& mat, int n, int m) {
    int size = n * m;
    int src = -1, des = -1;
    for(int i = 0; i < size; ++i) {
        if(mat[i] == 'A') src = i;
        if(mat[i] == 'B') des = i;
    }

    vector<int> parent(size, 4);
    parent[src] = 5;

    queue<int> q;
    q.push(src);

    int moves[] = {m, -m, 1, -1};
    char move_names[] = {'D', 'U', 'R', 'L'}; 

    bool found = false;
    while (!q.empty()) {
        int cur = q.front(); q.pop();
        if (cur == des) {
            found = true;
            break;
        }

        for (int i = 0; i < 4; i++) {
            if (i == 3 && cur % m == 0) continue;       // cannot go to left
            if (i == 2 && cur % m == m - 1) continue; // cannot go to right  
            
            int next = cur + moves[i];
            if ( ((0 <= next && next < size) && (mat[next] != '#') && (parent[next] == 4)) == false) {
                continue;
            }

            parent[next] = i;
            q.push(next);
        }
    }
    
    if (found == true) {
        cout << "YES" << '\n';
        std::string path = "";
        int cur = des;
        while (cur != src) {
            int move_idx = parent[cur];
            path += move_names[move_idx];
            cur -= moves[move_idx];
        }
        reverse(path.begin(), path.end());
        cout << path.length() << '\n';
        cout << path << '\n';
    }
    else {
        cout << "NO" << '\n';
    }
}

int main() {
    int n, m, size;
    cin >> n >> m;

    vector<char> mat(n * m);
    for (auto &it: mat) cin >> it;
    
    bfs(mat, n, m);
    

    
}