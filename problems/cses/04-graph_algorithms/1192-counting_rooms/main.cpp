#include <iostream>
#include <vector> 
#include <queue> 

using std::cin, std::cout, std::vector, std::queue;

bool in_ranges(int x, int l, int r) {
    return l <= x && x <= r;
}

void bfs(vector<vector<char>>& grid, int i, int j) {
    
    queue<std::pair<int,int>> q;
    q.push({i, j}); 

    int dx[] = {-1, 1, 0, 0};
    int dy[] = {0, 0, 1, -1};

    while (q.size()) {
        auto tmp = q.front(); q.pop();

        if (grid[tmp.first][tmp.second] == '#') 
            continue;
        
        grid[tmp.first][tmp.second] = '#';

        for (int i = 0; i < 4; i++) {
            int x = tmp.first + dx[i];
            int y = tmp.second + dy[i];
            if  (!(in_ranges(x, 0, grid.size() - 1) && in_ranges(y, 0, grid[0].size() - 1))) 
                continue;
            
            if (grid[x][y] == '#')
                continue;
            
            q.push({x, y});
        }
    }
}

int main() {
    int n, m;
    cin >> n >> m;
    vector<vector<char>> grid(n, vector<char>(m));
    for (int i = 0; i < n; i++) 
        for (int j = 0; j < m; j++)
            cin >> grid[i][j];
    
    int number_of_rooms = 0;
    for (int i = 0; i < n; i++) 
        for (int j = 0; j < m; j++) {
            if (grid[i][j] == '.') {
                number_of_rooms++;
                bfs(grid, i, j);
            }
        }    

    cout << number_of_rooms << '\n';
    
}       