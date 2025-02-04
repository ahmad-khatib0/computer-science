import numpy as np
import networkx as nx
import matplotlib.pyplot as plt


## Note that the network is from https://networkx.org/. For the purpose of this demonstration,
###


def create_page_rank(a_graph):
    "Will return G, which represents the transition matrix for our graph"
    nodes_set = len(a_graph)
    M = nx.to_numpy_array(a_graph)
    outwards = np.squeeze(np.asarray(np.sum(M, axis=1)))
    prob_outwards = np.array([1.0 / count if count > 0 else 0.0 for count in outwards])

    G = np.asarray(np.multiply(M.T, prob_outwards))
    p = np.ones(nodes_set) / float(nodes_set)
    return G, p


if __name__ == "__main__":
    # let’s assume that we are analyzing only five web pages in the network
    my_web = nx.DiGraph()
    my_pages = range(1, 6)

    # let’s connect them randomly to simulate an actual network:
    connections = [
        (1, 3),
        (2, 1),
        (2, 3),
        (3, 1),
        (3, 2),
        (3, 4),
        (4, 5),
        (5, 1),
        (5, 4),
    ]
    my_web.add_nodes_from(my_pages)
    my_web.add_edges_from(connections)

    # let’s plot this graph:
    pos = nx.shell_layout(my_web)
    nx.draw(my_web, pos, arrows=True, with_labels=True)
    plt.show()

    # In the PageRank algorithm, the patterns of a web page are contained in a matrix called the
    # transition matrix. There are algorithms that constantly update the transition matrix to capture
    # the constantly changing state of the web. The size of the transition matrix is n x n, where n
    # is the number of nodes. The numbers in the matrix are the probability that a visitor will next
    # go to that link due to the outbound link.

    G, p = create_page_rank(my_web)
    print(G)
