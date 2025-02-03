def linear_search(elements: list, item):
    index = 0
    found = False

    # Match the value with each data element
    while index < len(elements) and found is False:
        if elements[index] == item:
            found = True
        else:
            index = index + 1

    return found


if __name__ == "__main__":
    list = [12, 33, 11, 99, 22, 55, 90]
    print(linear_search(list, 12))
    print(linear_search(list, 91))
