import sys
from collections import deque


def solve():
    input_data = sys.stdin.read().split()

    H = int(input_data[0])
    W = int(input_data[1])
    M = "".join(input_data[2:])

    size = H * W
    src = M.find("A")
    des = M.find("B")

    q = deque([src])
    parent = bytearray([4] * size)
    parent[src] = 5

    dirs = [-W, W, -1, 1]
    dirs_names = "UDLR"

    found = False

    while q:
        curr = q.popleft()
        if curr == des:
            found = True
            break

        for i in range(4):
            nxt = curr + dirs[i]
            if 0 <= nxt < size and parent[nxt] == 4 and M[nxt] != "#":
                if i == 2 and curr % W == 0:
                    continue
                if i == 3 and curr % W == W - 1:
                    continue

                parent[nxt] = i
                q.append(nxt)

    if found:
        path = []
        curr = des
        while curr != src:
            move_idx = parent[curr]
            path.append(dirs_names[move_idx])
            curr -= dirs[move_idx]

        print("YES")
        print(len(path))
        print("".join(path[::-1]))
    else:
        print("NO")


if __name__ == "__main__":
    solve()
