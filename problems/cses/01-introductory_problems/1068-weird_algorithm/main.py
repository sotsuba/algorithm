if __name__ == "__main__":
    n = int(input())

    while n != 1:
        print(n, end=" ")
        n = (3 * n + 1) if (n & 1) else (n // 2)

    print(1)
