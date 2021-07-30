import itertools
import math

import pytest
from all_lap import LEFT, RIGHT, BipartiteGraph, Node, NodeSet


@pytest.mark.parametrize('n, m', filter(lambda x: x[0] >= x[1], itertools.product(range(1, 6), range(0, 4))))
def test_completeness(n, m):
    # complete bipartite graph K(M, N)
    adj = [list(range(m)) for _ in range(n)]
    graph = BipartiteGraph(adj)
    valid_nodes = NodeSet([Node(LEFT, i) for i in range(n)] + [Node(RIGHT, i) for i in range(m)], lsize=n)

    ms = list(graph.iter_matchings(valid_nodes))
    count = sum(1 for _ in graph.iter_matchings(valid_nodes))
    expected_count = math.factorial(n) / math.factorial(n - m) if m > 0 else 0
    assert count == expected_count


def test_ignore_unnecessary_nodes():
    adj = [
        [0, 2],
        [0, 1],
        [2, 3],
        [2, 3],
    ]
    graph = BipartiteGraph(adj)

    valid_nodes1 = [
        Node(LEFT, 0), Node(LEFT, 1),
        Node(RIGHT, 0), Node(RIGHT, 1),
    ]
    valid_nodes2 = valid_nodes1 + [
        Node(LEFT, 2), Node(LEFT, 3),
        Node(RIGHT, 2), Node(RIGHT, 3),
    ]
    n_left = len(adj)

    # All possible solution is 2, but we ignore permutation between dummies - which makes it 1
    nodeset = NodeSet(valid_nodes1, n_left)
    solutions = list(graph.iter_matchings(nodeset))
    assert len(solutions) == 1

    # All possible solution is 2, but we ignore permutation between dummies - which makes it 1
    nodeset = NodeSet(valid_nodes2, n_left)
    solutions = list(graph.iter_matchings(nodeset))
    assert len(solutions) == 2


