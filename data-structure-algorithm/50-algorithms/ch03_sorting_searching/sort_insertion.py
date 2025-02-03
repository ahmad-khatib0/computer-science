def insertion_sort(elements: list):
    for i in range(1, len(elements)):
        j = i - 1
        next_element = elements[i]

        # Iterate backward through the sorted portion,
        # looking for the appropriate position for 'next_element'
        while j >= 0 and elements[j] > next_element:
            elements[j + 1] = elements[j]
            j -= 1

        elements[j + 1] = next_element
    return elements


if __name__ == "__main__":
    list = [25, 21, 22, 24, 23, 27, 26]
    print(insertion_sort(list))
