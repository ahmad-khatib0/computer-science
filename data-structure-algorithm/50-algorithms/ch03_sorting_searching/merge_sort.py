def merge_sort(elements: list):
    """
    algorithm’s steps:
         1. Divide the list around a central midPoint.
         2. Recursively split until each section has just one element.
         3. Systematically merge the sorted sections into a comprehensive sorted list.
    """

    # # Base condition to break the recursion
    if len(elements) <= 1:
        return elements

    mid = len(elements) // 2  # Split the list in half
    left = elements[:mid]
    right = elements[mid:]

    merge_sort(left)  # Sort the left half
    merge_sort(right)  # Sort the right half

    a, b, c = 0, 0, 0
    # Merge the two halves
    while a < len(left) and b < len(right):
        if left[a] < right[b]:
            elements[c] = left[a]
            a += 1
            print("called a", a)
        else:
            elements[c] = right[b]
            b += 1
            print("called b", b)
        c += 1

    # # If there are remaining elements in the left half
    while a < len(left):
        print("a", a)
        elements[c] = left[a]
        a += 1
        c += 1

    # If there are remaining elements in the right half
    while b < len(right):
        print("b", b)
        elements[c] = right[b]
        b += 1
        c += 1

    return elements


if __name__ == "__main__":
    list = [21, 22, 23, 24, 25, 26, 27]
    print(merge_sort(list))
