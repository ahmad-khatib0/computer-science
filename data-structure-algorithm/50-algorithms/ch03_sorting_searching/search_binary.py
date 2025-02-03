def binary_search(elements, item):
    first = 0
    last = len(elements) - 1

    while first <= last:
        mid_point = (first + last) // 2
        if elements[mid_point] == item:
            return True
        else:
            if item < elements[mid_point]:
                last = mid_point - 1
            else:
                first = mid_point + 1
    return False


if __name__ == "__main__":
    list = [12, 33, 11, 99, 22, 55, 90]
    print(binary_search(list, 12))
    print(binary_search(list, 59))
