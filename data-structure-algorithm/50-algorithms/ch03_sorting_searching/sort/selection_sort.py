def selection_sort(elements):
    for fill_slot in range(len(elements) - 1, 0, -1):
        max_index = 0
        for location in range(1, fill_slot + 1):
            if elements[location] > elements[max_index]:
                max_index = location
        elements[fill_slot], elements[max_index] = (
            elements[max_index],
            elements[fill_slot],
        )
    return elements


if __name__ == "__main__":
    list = [21, 22, 23, 24, 25, 26, 27]
    print(selection_sort(list))
