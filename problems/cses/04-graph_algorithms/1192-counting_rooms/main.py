import sys
from collections import deque


def solve():
    input_data = sys.stdin.read().split()
    if not input_data:
        return

    n = int(input_data[0])
    m = int(input_data[1])

    grid = [list(input_data[i + 2]) for i in range(n)]

    def bfs(start_i, start_j):
        q = deque([(start_i, start_j)])
        grid[start_i][start_j] = "#"

        directions = [(-1, 0), (1, 0), (0, 1), (0, -1)]

        while q:
            curr_i, curr_j = q.popleft()

            for di, dj in directions:
                ni, nj = curr_i + di, curr_j + dj

                if 0 <= ni < n and 0 <= nj < m and grid[ni][nj] == ".":
                    grid[ni][nj] = "#"
                    q.append((ni, nj))

    number_of_rooms = 0
    for i in range(n):
        for j in range(m):
            if grid[i][j] == ".":
                number_of_rooms += 1
                bfs(i, j)

    sys.stdout.write(str(number_of_rooms) + "\n")


if __name__ == "__main__":
    solve()
