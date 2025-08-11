use std::collections::VecDeque;

#[derive(PartialEq, Copy, Clone, Debug)]
pub struct NodeId {
    pub idx: usize,
}

impl From<usize> for NodeId {
    fn from(idx: usize) -> Self {
        NodeId { idx }
    }
}

#[derive(Clone, Debug)]
pub struct Graph<T> {
    entries: Vec<Node<T>>,
}

impl<T: Clone> Graph<T> {
    pub fn new() -> Self {
        Graph {
            entries: Vec::new(),
        }
    }

    pub fn add(&mut self, mut node: Node<T>) -> NodeId {
        let index = self.entries.len();
        node.set_node_id(index.into());
        self.entries.push(node);

        index.into()
    }

    pub fn get_mut(&mut self, id: NodeId) -> &mut Node<T> {
        self.entries.get_mut(id.idx).unwrap()
    }

    pub fn get(&self, id: NodeId) -> &Node<T> {
        self.entries.get(id.idx).unwrap()
    }

    pub fn entries(&self) -> &Vec<Node<T>> {
        &self.entries
    }

    pub fn entries_mut(&mut self) -> &mut [Node<T>] {
        self.entries.as_mut_slice()
    }

    // BFS traversal of the graph
    pub fn traverse(&self) -> Vec<&Node<T>> {
        let mut queue = VecDeque::new();
        if self.entries.len() == 0 {
            return vec![];
        }

        let entries = self.entries();
        let root = entries.get(0).unwrap();
        queue.push_back(root);
        let mut traversed = Vec::new();
        while !queue.is_empty() {
            let current = queue.pop_front().unwrap();
            for child in current.get_children() {
                queue.push_back(entries.get(child.idx).unwrap());
            }
            traversed.push(current);
        }

        traversed
    }

    pub fn copy_graph(graph: &Graph<T>) -> Graph<T> {
        let mut copy = Graph::new();

        for node in graph.entries() {
            copy.add(node.clone());
        }

        copy
    }
}

#[derive(Clone, Debug)]
pub struct Node<T> {
    value: T,
    idx: NodeId,
    parent: Option<NodeId>,
    children: Vec<NodeId>,
}

impl<T: Clone> Node<T> {
    pub fn new(value: T) -> Self {
        Node {
            value,
            idx: 0.into(),
            parent: None,
            children: Vec::new(),
        }
    }

    pub fn get(&self) -> &T {
        &self.value
    }

    pub fn set(&mut self, v: T) {
        self.value = v
    }

    fn set_node_id(&mut self, idx: NodeId) {
        self.idx = idx
    }

    pub fn node_id(&self) -> NodeId {
        self.idx
    }

    pub fn get_children(&self) -> &Vec<NodeId> {
        &self.children
    }

    pub fn append_children(&mut self, children: Vec<NodeId>) {
        for c in children {
            self.append_child(c.into())
        }
    }

    pub fn append_child(&mut self, child: NodeId) {
        self.children.push(child);
    }

    pub fn parent(&self) -> Option<NodeId> {
        self.parent
    }

    pub fn set_parent(mut self, parent: NodeId) -> Self {
        self.parent = Some(parent);
        self
    }
}
