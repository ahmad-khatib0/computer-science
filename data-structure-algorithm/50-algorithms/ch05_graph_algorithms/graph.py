import networkx as nx

# A “simple graph,” as alluded to in graph theory, is a graph that has no parallel edges or loops.

if __name__ == "__main__":
    graph = nx.Graph()

    # add a single vertex:
    graph.add_node("Mike")

    # We can also add a series of vertices using a list:
    graph.add_nodes_from(["Amine", "Wassim", "Nick"])

    # We can also add one edge between the existing vertices
    graph.add_edge("Mike", "Amine")

    print(graph.nodes())  # ['Mike', 'Amine', 'Wassim', 'Nick']
    print(graph.edges())  # [('Mike', 'Amine')]

    # note that if we add an edge, this also leads to adding the associated
    # vertices, if they do not already exist
    graph.add_edge("Amine", "Imran")
    print(graph.edges())  # [('Mike', 'Amine'), ('Amine', 'Imran')]

    # Note that the request to add a vertex that already exists is silently ignored. The request
    # is ignored or considered based on the type of graph we have created.
