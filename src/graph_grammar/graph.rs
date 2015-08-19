use std::fmt::Debug;

#[derive(Clone)]
pub struct DirectedNode<NodeType: PartialEq + Clone, EdgeType: PartialEq + Clone> {
    pub label: NodeType,
    pub to: Vec<Edge<EdgeType>>,
    pub from: Vec<usize>,
}

#[derive(Clone)]
pub struct Edge<T: PartialEq + Clone> {
    pub label: T,
    pub from: usize,
    pub to: usize,
}

impl<T: PartialEq + Clone> Edge<T> {
    pub fn matches(&self, other: (usize, usize)) -> bool {
        (self.from, self.to) == other
    }
}

#[derive(Clone)]
pub struct DirectedGraph<NodeType: PartialEq + Clone, EdgeType: PartialEq + Clone> {
    data: Vec<DirectedNode<NodeType, EdgeType>>
}

impl<T: PartialEq + Clone, S: PartialEq + Clone> DirectedGraph<T, S> {

    pub fn new() -> DirectedGraph<T, S> {
        DirectedGraph{ data: Vec::new() }
    }

    pub fn from_vec(nodes: &[T], edges: &[(usize, usize, S)]) -> DirectedGraph<T, S> {
        let mut g = DirectedGraph{ data: Vec::new() };
        for node in nodes {
            g.push_node(node.clone());
        }
        for edge in edges {
            g.add_edge(edge.0, edge.1, edge.2.clone());
        }
        return g;
    }

    pub fn nodes(&self) -> usize {
        self.data.len()
    }

    pub fn push_node(&mut self, label: T) {
        let node = DirectedNode{ label: label, to: Vec::new(), from: Vec::new() };
        self.data.push(node);
    }

    pub fn get_node(&self, index: usize) -> &DirectedNode<T, S> {
        &self.data[index]
    }

    pub fn mut_label(&mut self, index: usize) -> &mut T {
        &mut self.data[index].label
    }

    pub fn mut_edge_label(&mut self, start_index: usize, target_index: usize) -> &mut S {
        &mut self.data[start_index].to.iter_mut().find(|e| e.to == target_index).unwrap().label
    }

    pub fn remove_node(&mut self, index: usize) {
        self.remove_outward_edges(index);
        self.remove_inward_edges(index);
        self.repoint_edges_to_last(index);
        self.reroot_edges_from_last(index);
        self.data.swap_remove(index);
    }

    fn remove_outward_edges(&mut self, index: usize) {
        for e in self.data[index].to.iter().map(|edge| edge.to).collect::<Vec<usize>>() {
            self.data[e].from = self.data[e].from.iter()
                .filter(|&edge| *edge != index)
                .map(|&edge| edge)
                .collect();
        }
    }
    fn remove_inward_edges(&mut self, index: usize) {
        for i in self.data[index].from.clone() {
            let mut indexes = None;
            for edge in self.data[i].to.iter() {
                if edge.to == index {
                    indexes = Some((edge.from, edge.to));
                }
            }
            match indexes {
                Some(tup) => self.remove_edge(tup.0, tup.1),
                _ => {}
            };
        }
    }
    fn repoint_edges_to_last(&mut self, index: usize) {
        let last = self.nodes() - 1;
        for i in self.data[last].from.clone() {
            let mut iter = self.data[i].to.iter_mut();
            'inner: while let Some(edge) = iter.next() {
                if edge.to == last {
                    edge.to = index;
                    break 'inner;
                }
            }
        }
    }
    fn reroot_edges_from_last(&mut self, index: usize) {
        let last = self.nodes() - 1;
        for i in self.data[last].to.iter().map(|edge| edge.to).collect::<Vec<usize>>() {
            let mut iter = self.data[i].from.iter_mut();
            'inner: while let Some(from) = iter.next() {
                if *from == last {
                    *from = index;
                    break 'inner;
                }
            }
        }
        for edge in self.data[last].to.iter_mut() {
            edge.from = index;
        }
    }

    pub fn add_edge(&mut self, from: usize, to: usize, label: S) {
        let edge = Edge{label: label.clone(), from: from, to: to};
        self.data[from].to.push(edge);
        self.data[to].from.push(from);
    }

    pub fn add_undir_edge(&mut self, from: usize, to: usize, label: S) {
        let edge = Edge{label: label.clone(), from: from, to: to};
        self.data[from].to.push(edge);
        self.data[to].from.push(from);
        let edge = Edge{label: label, from: to, to: from};
        self.data[to].to.push(edge);
        self.data[from].from.push(to);
    }

    pub fn remove_edge(&mut self, from: usize, to: usize) {
        let mut i = 0;
        for p in &mut self.data[from].to {
            if p.to == to {
                break;
            }
            i += 1;
        }
        self.data[from].to.swap_remove(i);

        i = 0;
        for p in &mut self.data[to].from {
            if *p == from {
                break;
            }
            i += 1;
        }
        self.data[to].from.swap_remove(i);
    }

    pub fn cut_at(&mut self, node1: T, node2: T) -> Option<DirectedGraph<T, S>> {
        unimplemented!();
    }

    pub fn connect_at(&mut self, s_node: T, other: DirectedGraph<T, S>, o_node: T) {
        unimplemented!();
    }
}

impl<T: Debug + PartialEq + Clone, S: Debug + PartialEq + Clone> ToString for DirectedGraph<T, S> {
    fn to_string(&self) -> String {
        let mut s = String::new();
        for node in &self.data {
            let mut sorted = node.from.clone();
            sorted.sort_by(|a, b| a.cmp(b));
            for path in sorted {
                s = s + &*path.to_string() + ",";
            }

            s = s + &*format!("-> {:?} -> ", node.label);

            let mut sorted = node.to.clone();
            sorted.sort_by(|a, b| a.to.cmp(&b.to));
            for path in sorted {
                s = s + &format!("{:?}", path.label) + "(" + &*path.from.to_string() + "-"
                      + &*path.to.to_string() + ")" + ",";
            }
            s = s + "\n";
        }
        s
    }
}
