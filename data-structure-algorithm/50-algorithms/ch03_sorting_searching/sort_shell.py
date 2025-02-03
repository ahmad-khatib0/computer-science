def shell_sort(elements: list):
    distance = len(elements) // 2
    while distance > 0:
        for i in range(distance, len(elements)):
            temp = elements[i]
            j = i

            # Sort the sub list for this distance
            while j >= distance and elements[j - distance] > temp:
                # shifts elements to make space for temp
                elements[j] = elements[j - distance]
                j = j - distance
            elements[j] = temp  # places temp in its correct sorted position

        # Reduce the distance for the next element
        distance = distance // 2

    return elements


if __name__ == "__main__":
    ls = [21, 22, 23, 24, 25, 26, 27]
    print(shell_sort(ls))
