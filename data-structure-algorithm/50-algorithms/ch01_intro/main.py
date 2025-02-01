def get_first(ls):
    return ls[0]


def get_sum(my_list):
    sum = 0
    for item in my_list:
        sum = sum + item
    return sum


def get_sum_quadratic(my_list):
    sum = 0
    for row in my_list:
        for item in row:
            sum += item
    return sum


def search_binary(ls, item):
    first = 0
    last = len(ls) - 1
    found_flag = False
    while first <= last and not found_flag:
        mid = (first + last) // 2
        if ls[mid] == item:
            found_flag = True
        else:
            if item < ls[mid]:
                last = mid - 1
            else:
                first = mid + 1
    return found_flag


if __name__ == "__main__":
    # Constant time (O(1)) complexity
    get_first([1, 2, 3])
    get_first([1, 2, 3, 4, 5, 6, 7, 8, 9, 10])

    # Linear time (O(n)) complexity
    get_sum([1, 2, 3])

    # Quadratic time (O(n2)) complexity
    get_sum_quadratic([[1, 2], [3, 4]])

    search_binary([8, 9, 10, 100, 1000, 2000, 3000], 10)
    search_binary([8, 9, 10, 100, 1000, 2000, 3000], 5)
