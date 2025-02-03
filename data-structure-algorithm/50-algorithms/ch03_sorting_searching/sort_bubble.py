def bubble_sort1():
    # If we implement pass one of bubble sort in Python, it will look as follows:
    list = [25, 21, 22, 24, 23, 27, 26]
    last_element_index = len(list) - 1
    print(0, list)
    for idx in range(last_element_index):
        if list[idx] > list[idx + 1]:
            list[idx], list[idx + 1] = list[idx + 1], list[idx]
        print(idx + 1, list)
    # 0 [25, 21, 22, 24, 23, 27, 26]
    # 1 [21, 25, 22, 24, 23, 27, 26]
    # 2 [21, 22, 25, 24, 23, 27, 26]
    # 3 [21, 22, 24, 25, 23, 27, 26]
    # 4 [21, 22, 24, 23, 25, 27, 26]
    # 5 [21, 22, 24, 23, 25, 27, 26]
    # 6 [21, 22, 24, 23, 25, 26, 27]


def bubble_sort2(ls):
    last_element_index = len(ls) - 1
    for pass_no in range(last_element_index, 0, -1):  # O(N2)
        for idx in range(pass_no):
            if ls[idx] > ls[idx + 1]:
                ls[idx], ls[idx + 1] = ls[idx + 1], ls[idx]
    return ls


def optimized_bubble_sort(ls):
    last_element_index = len(ls) - 1
    for pass_no in range(last_element_index, 0, -1):
        # permits the algorithm to detect early if the list is already sorted before making all N-1
        # passes. When a pass completes without any swaps, it’s a clear indicator that the list has
        # been sorted, and the algorithm can exit prematurely. Therefore, while the worst-case time
        # complexity remains O(N2) for completely unsorted or reverse-sorted lists, the best-case
        # complexity improves to O(N) for already sorted lists due to this optimization.
        swapped = False
        for idx in range(pass_no):
            if ls[idx] > ls[idx + 1]:
                ls[idx], ls[idx + 1] = ls[idx + 1], ls[idx]
                swapped = True
        if not swapped:
            break
    return ls


if __name__ == "__main__":
    # Swapping variables
    var_1 = 1
    var_2 = 2
    var_1, var_2 = var_2, var_1
    print(var_1, var_2)
    bubble_sort1()

    ls = [25, 21, 22, 24, 23, 27, 26]
    print(bubble_sort2(ls))

    list = [25, 21, 22, 24, 23, 27, 26]
    print(optimized_bubble_sort(list))
