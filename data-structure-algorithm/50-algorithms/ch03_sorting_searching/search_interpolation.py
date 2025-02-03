from sort_bubble import optimized_bubble_sort


def interpolation_search(ls, x):
    idx0 = 0
    idxn = len(ls) - 1

    while idx0 <= idxn and x >= ls[idx0] and x <= ls[idxn]:
        # Find the mid point
        mid = idx0 + int(
            ((float(idxn - idx0) / (ls[idxn] - ls[idx0])) * (x - ls[idx0]))
        )

        # Compare the value at mid point with search value
        if ls[mid] == x:
            return True

        if ls[mid] < x:
            idx0 = mid + 1

    return False


if __name__ == "__main__":
    list = [12, 33, 11, 99, 22, 55, 90]
    sorted_list = optimized_bubble_sort(list)
    print(interpolation_search(list, 12))
    print(interpolation_search(list, 91))
