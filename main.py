if __name__ == "__main__":
    n = int(input())
    arr = list(map(int, input().split()))

    res = 0
    prev = arr[0]
    for cur in arr:
        if cur < prev:
            res += prev - cur
        else:
            prev = cur

    print(res)
