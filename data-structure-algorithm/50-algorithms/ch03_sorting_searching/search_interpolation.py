from sort_bubble import optimized_bubble_sort


def interpolation_search(ls, x):
    idx0 = 0
    idxn = len(ls) - 1

    while idx0 <= idxn and x >= ls[idx0] and x <= ls[idxn]:
        # Find the mid point
        #
        # 1- Range of the Search Space idxn - idx0: This gives the length of the current
        #    search space (the number of elements between idx0 and idxn).
        # 2- Value Range: ls[idxn] - ls[idx0]: This gives the range of values between
        #    the first and last elements of the current search space.
        # 3- Proportional Position: (x - ls[idx0]): This calculates how far the target
        #    value x is from the first element in the current search space.
        #
        # 4- Scaling Factor: (float(idxn - idx0) / (ls[idxn] - ls[idx0])): This is a
        #    scaling factor that maps the value range to the index range. It tells us
        #    how much the index should change per unit change in value.
        # 5- Estimated Position: ((float(idxn - idx0) / (ls[idxn] - ls[idx0])) * (x - ls[idx0])):
        #    This estimates how far the target value x is from idx0 in terms of indices, based
        #    on the value distribution.
        # 6- Final Midpoint Calculation: idx0 + int(...): This adds the estimated offset to the
        #    starting index idx0 to get the final estimated position (mid).
        #
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
