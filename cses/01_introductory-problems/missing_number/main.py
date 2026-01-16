if __name__ == "__main__":
    n = int(input())

    expected_sum = n * (n + 1) // 2
    current_sum = sum(map(int, input().split()))

    print(expected_sum - current_sum)

# // https://cses.fi/problemset/task/1083
