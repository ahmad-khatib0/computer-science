import random
from collections import Counter
from itertools import permutations
from time import time

import matplotlib.pyplot as plt

# Let’s first define three utility functions:
# • distance_points: Calculates the absolute distance between two points
# • distance_tour: Calculates the total distance the salesperson has to cover in a given tour
# • generate_cities: Randomly generates a set of n cities located in a rectangle of width
#   500 and height 300

# represented the distance with a complex number
aCity = complex


def distance_tour(aTour):
    # Calculating the distance between two cities, a and b, is as simple as distance (a,b).
    return sum(distance_points(aTour[i - 1], aTour[i]) for i in range(len(aTour)))


def distance_points(first, second):
    return abs(first - second)


def generate_cities(number_of_cities):
    seed = 111
    width = 500
    height = 300
    random.seed((number_of_cities, seed))
    return frozenset(
        aCity(random.randint(1, width), random.randint(1, height))
        for _ in range(number_of_cities)
    )


def brute_force(cities):
    # brute_force, that generates all the possible tours of the cities.
    return shortest_tour(permutations(cities))


def shortest_tour(tours):
    return min(tours, key=distance_tour)


def visualize_tour(tour, style="bo-"):
    """
    visualize_tour: Plots all the cities and links in a particular tour.
    It also highlights the city from which the tour started.
    """
    if len(tour) > 1000:
        plt.figure(figsize=(15, 10))
    start = tour[0:1]
    visualize_segment(tour + start, style)
    visualize_segment(start, "rD")


def visualize_segment(segment, style="bo-"):
    "visualize_segment: Used by visualize_tour to plot cities and links in a segment."
    plt.plot([X(c) for c in segment], [Y(c) for c in segment], style, clip_on=False)
    plt.axis("scaled")
    plt.axis("off")


def X(city):
    "X axis"
    return city.real


def Y(city):
    "Y axis"
    return city.imag


def tsp(algorithm, cities):
    """
    1- Generates the tour based on the algorithm and number of cities requested.
    2- Calculates the time it took for the algorithm to run.
    3- Generates a plot.
    """
    t0 = time()
    tour = algorithm(cities)
    t1 = time()
    # Every city appears exactly once in tour
    assert Counter(tour) == Counter(cities)
    visualize_tour(tour)
    print(
        "{}:{} cities => tour length {;.0f} (in {:.3f} sec".format(
            name(algorithm), len(tour), distance_tour(tour), t1 - t0
        )
    )


def name(algorithm):
    return algorithm.__name__.replace("_tsp", "")


# ******************************************************************
# ^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^
def greedy_algorithm(cities, start=None):
    city_ = start or first(cities)
    tour = [city_]
    unvisited = set(cities - {city_})
    while unvisited:
        city_ = nearest_neighbor(city_, unvisited)
        tour.append(city_)
        unvisited.remove(city_)
    return tour


def first(collection):
    return next(iter(collection))


def nearest_neighbor(city_a, cities):
    return min(cities, key=lambda city_: distance_points(city_, city_a))


if __name__ == "__main__":
    # Note that we have used it to generate the tour for 10 cities. As n = 10, it will generate
    # (10-1)! = 362,880 possible permutations. If n increases, the number of permutations sharply
    # increases and the brute-force method cannot be used.
    tsp(brute_force, generate_cities(10))

    #
    # Using a greedy algorithm
    # If we use a greedy algorithm to solve the TSP, then, at each step, we can choose a city that
    # seems reasonable, instead of finding a city to visit that will result in the best overall path.
    # So, whenever we need to select a city, we just select the nearest city without bothering to verify
    # that this choice will result in the globally optimal path.
    # The approach of the greedy algorithm is simple:
    # 1. Start from any city.
    # 2. At each step, keep building the tour by moving to the next city where the nearest
    #    neighborhood has not been visited before.
    # 3. Repeat step 2.
    #
    # use greedy_algorithm to create a tour for 2,000 cities:
    #
    tsp(greedy_algorithm, generate_cities(2000))
    #
    # Note that it took only 0.514 seconds to generate the tour for 2,000 cities. If we had used
    # the bruteforce method, it would have generated (2000-1)! = 1.65e^5732 permutations, which is
    # almost infinite. Note that the greedy algorithm is based on heuristics and there is no proof
    # that the solution will be optimal.
