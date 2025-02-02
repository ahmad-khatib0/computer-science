import random
import pandas as pd
import numpy as np


class Stack:
    def __init__(self) -> None:
        self.items = []

    def isEmpty(self):
        return self.items == []

    def push(self, item):
        self.items.append(item)

    def pop(self):
        return self.items.pop()

    def peek(self):
        return self.items[len(self.items) - 1]

    def size(self):
        return len(self.items)


# FIFO
class Queue(object):
    def __init__(self):
        self.items = []

    def isEmpty(self):
        return self.items == []

    def enqueue(self, item):
        self.items.insert(0, item)

    def dequeue(self):
        return self.items.pop()

    def size(self):
        return len(self.items)


if __name__ == "__main__":
    list_a = ["John", 33, "Toronto", True]
    print(list_a)
    print(list_a[1])

    list_a.append(44)

    dice_output = [random.randint(1, 6) for x in range(10)]

    bin_colors = ("Red", "Green", "Blue", "Yellow")
    print(f"The second element of the tuple is {bin_colors[1]}")
    nested_tuple = (1, 2, (100, 200, 300), 6)
    print(f"The maximum value of the inner tuple {max(nested_tuple[2])}")

    # dictionary
    bin_colors = {
        "manual_color": "Yellow",
        "approved_color": "Green",
        "refused_color": "Red",
    }
    bin_colors.get("approved_color")
    bin_colors["approved_color"]
    for k, v in bin_colors.items():
        print(k, "->", v + " color")
    del bin_colors["approved_color"]

    # sets only stores the distinct value of each element
    green = {"grass", "leaves", "leaves"}
    yellow = {"dandelions", "fire hydrant", "leaves"}
    red = {"fire hydrant", "blood", "rose", "leaves"}
    print(f"The union of yellow and red sets is {yellow | red}")
    print(f"The intersection (overlap) of yellow and red is {yellow & red}")
    for x in yellow:
        print(x)
    ## check if a specified value is present in a set by using the in keyword.
    print("leaves" in yellow)

    # A Series can be defined as follows:
    person_1 = pd.Series(["John", "Male", 33, True])
    print(person_1)

    # DataFrame (id, name, age, decision)
    df = pd.DataFrame(
        [["1", "Fares", 32, True], ["2", "Elena", 23, False], ["3", "Doug", 40, True]]
    )
    df.columns = ["id", "name", "age", "decision"]
    print(df)

    # Column And Row selection
    df[["name", "age"]]
    # A column can be retrieved by its position
    print("\n", df.iloc[:, 3])
    # 0    True
    # 1    False
    # 2    True
    # Name: decision, dtype: bool

    # A subset of rows can be retrieved by its position
    print("\n", df.iloc[1:3, :])
    #    id   name  age  decision
    # 1  2  Elena   23     False
    # 2  3   Doug   40      True

    # To create a subset by specifying the filter
    print("\n", df[df.age > 30])
    print("\n", df[(df.age < 35) & (df.decision == True)])  # noqa: E712

    # Matrices
    matrix_1 = np.array([[11, 12, 13], [21, 22, 23], [31, 32, 33]])
    print(matrix_1)
    print(matrix_1.transpose())

    # Vector: There are two ways of creating vectors in Python:
    vector_1 = [22, 33, 44, 55]  # using Python list
    print(type(vector_1))

    vector_2 = np.array([334, 55, 66, 77])  # Using a numpy array:
    print(type(vector_2))

    stack = Stack()
    stack.push("Red")
    stack.push("Green")
    stack.push("Blue")
    stack.push("Yellow")

    stack.pop()
    stack.isEmpty()

    queue = Queue()
    queue.enqueue("Red")
    queue.enqueue("Green")
    queue.enqueue("Blue")
    queue.enqueue("Yellow")

    print(f"Size of queue is {queue.size()}")  # 4
    print(queue.dequeue())  # red
