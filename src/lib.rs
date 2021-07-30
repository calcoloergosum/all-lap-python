use all_lap_rust::bipartite as bp;
use all_lap_rust::contains::Contains;
use pyo3::prelude::*;
use pyo3::PyIterProtocol;

#[pyclass]
#[derive(Clone)]
struct BipartiteGraph {
    inner: bp::BipartiteGraph,
}

#[pymethods]
impl BipartiteGraph {
    #[new]
    pub fn __new__(adj: Vec<Vec<usize>>) -> Self {
        let inner = bp::BipartiteGraph::from_adj(adj);
        Self { inner }
    }
    pub fn iter_matchings(
        &self,
        allowed_start_nodes: NodeSet,
    ) -> PyResult<MaximumMatchingIterator> {
        Ok(MaximumMatchingIterator::__new__(
            self.clone(),
            allowed_start_nodes,
        ))
    }
}

#[pyclass]
#[derive(Clone)]
struct Node {
    inner: bp::Node,
}

#[pymethods]
impl Node {
    #[new]
    pub fn __new__(lr: bool, index: usize) -> Self {
        let nodegroup = match lr {
            false => bp::NodeGroup::Left,
            true => bp::NodeGroup::Right,
        };
        let inner = bp::Node::new(nodegroup, index);
        Self { inner }
    }
}

#[pyclass]
#[derive(Clone)]
struct NodeSet {
    inner: bp::NodeSet,
}

#[pymethods]
impl NodeSet {
    #[new]
    pub fn __new__(nodes: Vec<Node>, lsize: usize) -> Self {
        let hashset = nodes.into_iter().map(|x| x.inner).collect();
        Self {
            inner: bp::NodeSet::new(hashset, lsize),
        }
    }
}

impl Contains<bp::Node> for NodeSet {
    fn contains_node(&self, item: &bp::Node) -> bool {
        self.inner.contains_node(item)
    }
}

impl Contains<usize> for NodeSet {
    fn contains_node(&self, item: &usize) -> bool {
        self.inner.contains_node(item)
    }
}

#[pyclass]
struct MaximumMatchingIterator {
    inner: bp::MaximumMatchingsIterator<NodeSet>,
}

#[pymethods]
impl MaximumMatchingIterator {
    #[new]
    pub fn __new__(graph: BipartiteGraph, allowed_start_nodes: NodeSet) -> Self {
        let inner = bp::MaximumMatchingsIterator::from_graph(graph.inner, allowed_start_nodes);
        Self { inner }
    }
}

#[pyproto]
impl<'p> PyIterProtocol for MaximumMatchingIterator {
    fn __iter__(slf: PyRef<'p, Self>) -> PyRef<Self> {
        slf
    }

    fn __next__(mut slf: PyRefMut<'p, Self>) -> Option<Vec<Option<usize>>> {
        let m = slf.inner.next()?;
        if m.l2r.iter().all(|x| x.is_none()) {
            return None;
        }
        Some(m.l2r)
    }
}

#[pymodule]
fn rust_ext(_py: Python<'_>, m: &PyModule) -> PyResult<()> {
    // wrapper of `Iter`
    m.add_class::<BipartiteGraph>()?;
    m.add_class::<NodeSet>()?;
    m.add_class::<Node>()?;
    m.add_class::<MaximumMatchingIterator>()?;

    Ok(())
}
