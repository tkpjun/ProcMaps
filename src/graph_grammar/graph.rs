use std::collections::linked_list::LinkedList;
use std::fmt::Debug;

#[derive(Clone)]
pub struct DirectedNode<NodeType: Eq + Clone, EdgeType: Eq + Clone> {
    pub label: NodeType,
    pub to: LinkedList<Edge<EdgeType>>,
    pub from: LinkedList<usize>,
}

#[derive(Clone)]
pub struct Edge<T: Eq + Clone> {
    pub label: T,
    pub from: usize,
    pub to: usize,
}

impl<T: Eq + Clone> Edge<T> {
    pub fn matches(&self, other: (usize, usize)) -> bool {
        (self.from, self.to) == other
    }
}

pub struct DirectedGraph<NodeType: Eq + Clone, EdgeType: Eq + Clone> {
    data: Vec<DirectedNode<NodeType, EdgeType>>
}

impl<T: Eq + Clone, S: Eq + Clone> DirectedGraph<T, S> {

    pub fn new() -> DirectedGraph<T, S> {
        DirectedGraph{ data: Vec::new() }
    }

    pub fn nodes(&self) -> usize {
        self.data.len()
    }

    pub fn push_node(&mut self, label: T) {
        let node = DirectedNode{ label: label, to: LinkedList::new(), from: LinkedList::new() };
        self.data.push(node);
    }

    pub fn get_node(&self, index: usize) -> &DirectedNode<T, S> {
        &self.data[index]
    }

    pub fn mut_node(&mut self, index: usize) -> &mut DirectedNode<T, S> {
        &mut self.data[index]
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
        'outer: for i in self.data[last].from.clone() {
            let mut iter = self.data[i].to.iter_mut();
            let mut next = iter.next();
            'inner: while next.is_some() {
                let edge = next.unwrap();
                if edge.to == last {
                    edge.to = index;
                    break 'inner;
                }
                next = iter.next();
            }
        }
    }
    fn reroot_edges_from_last(&mut self, index: usize) {
        let last = self.nodes() - 1;
        'outer: for i in self.data[last].to.iter().map(|edge| edge.to).collect::<Vec<usize>>() {
            let mut iter = self.data[i].from.iter_mut();
            let mut next = iter.next();
            'inner: while next.is_some() {
                let from = next.unwrap();
                if *from == last {
                    *from = index;
                    break 'inner;
                }
                next = iter.next();
            }
        }
        for edge in self.data[last].to.iter_mut() {
            edge.from = index;
        }
    }

    //Doesnt make sure the opposite end has the same path
    pub fn set_edges(&mut self, index: usize, edges: &[Edge<S>]) {
        self.data[index].from.clear();
        self.data[index].to.clear();
        for edge in edges {
            let from = edge.from;
            let to = edge.to;
            self.data[edge.from].to.push_back(edge.clone());
            self.data[to].from.push_back(from);
        }
    }

    pub fn add_edge(&mut self, from: usize, to: usize, label: S, is_directed: bool) {
        let edge = Edge{label: label.clone(), from: from, to: to};
        self.data[from].to.push_back(edge);
        self.data[to].from.push_back(from);
        if !is_directed {
            let edge = Edge{label: label, from: to, to: from};
            self.data[to].to.push_back(edge);
            self.data[from].from.push_back(to);
        }
    }

    pub fn remove_edge(&mut self, from: usize, to: usize) {
        let mut i = 0;
        for p in &mut self.data[from].to {
            if p.to == to {
                break;
            }
            i += 1;
        }
        let mut end = self.data[from].to.split_off(i);
        end.pop_front();
        self.data[from].to.append(&mut end);

        i = 0;
        for p in &mut self.data[to].from {
            if *p == from {
                break;
            }
            i += 1;
        }
        let mut end = self.data[to].from.split_off(i);
        end.pop_front();
        self.data[to].from.append(&mut end);
    }

    //Returns the shortest path to the target in reverse: target is result[0].
    pub fn bfs(&self, start: usize, exclude: usize, target: &T) -> Option<Vec<usize>> {
        let mut search = vec![ListNode{val: start, prev: 0}];
        let mut marked = vec![exclude];
        let build_ret = |space: &Vec<ListNode<usize>>| -> Option<Vec<usize>> {
            let index = space.len() - 1;
            let mut ret = vec![space[index].val];
            let mut next = space[index].prev;
            while next > 0 {
                ret.push(space[next].val);
                next = space[next].prev;
            }
            return Some(ret);
        };

        let mut i = 0;
        while i <= search.len() {
            for path in &self.data[search[i].val].to {
                if self.data[path.to].label == *target {
                    search.push(ListNode{val: path.to, prev: i});
                    return build_ret(&search);
                }
                if marked.iter().all(|a| *a != path.to) {
                    marked.push(path.to);
                    search.push(ListNode{val: path.to, prev: i});
                }
            }
            i += 1;
        }
        None
    }
}

impl<T: Debug + Eq + Clone, S: Debug + Eq + Clone> ToString for DirectedGraph<T, S> {
    fn to_string(&self) -> String {
        let mut s = String::new();
        for node in &self.data {
            for path in &node.from {
                s = s + &*path.to_string() + ",";
            }
            s = s + &*format!("-> {:?} -> ", node.label);
            for path in &node.to {
                s = s + &format!("{:?}", path.label) + "(" + &*path.from.to_string() + "-"
                      + &*path.to.to_string() + ")" + ",";
            }
            s = s + "\n";
        }
        s
    }
}

struct ListNode<T> {
    val: T,
    prev: usize,
}
