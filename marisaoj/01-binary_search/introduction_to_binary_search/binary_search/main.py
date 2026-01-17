def binary_search(arr: list[int], target: int) -> int:
    l, r = 0, len(arr) - 1

    while l <= r:
        m = l + (r - l) // 2
        if arr[m] == target:
            return m
        if arr[m] < target:
            l = m + 1
        else:
            r = m - 1

    return -1


if __name__ == "__main__":
    n, q = list(map(int, input().split()))
    arr: list[int] = list(map(int, input().split()))
    while q:
        q -= 1
        target = int(input())
        print(binary_search(arr, target) + 1)
