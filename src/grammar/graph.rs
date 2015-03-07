pub use std::collections::linked_list::LinkedList;

pub struct Graph<T> {
    pub data: Vec<Node<T>>
}

pub struct Node<T> {
    pub value: T,
    pub paths: LinkedList<usize>
}

impl<T> Graph<T> {
    pub fn new() -> Graph<T> {
        Graph{ data: Vec::new() }
    }

    pub fn push_node(&mut self, value: T) {
        let node = Node{ value: value, paths: LinkedList::new() };
        self.data.push(node);
    }

    pub fn add_node_between(&mut self, value: T, first: usize, second: usize) {
        let mut node = Node{ value: value, paths: LinkedList::new() };
        node.paths.push_back(first);
        node.paths.push_back(second);
        self.data.push(node);

        let mut paths = Vec::new();
        for path in &self.data[first].paths {
            if (*path != second) {
                paths.push(*path)
            }
        }
        self.set_paths(first, &paths);

        let mut paths = Vec::new();
        for path in &self.data[second].paths {
            if (*path != first) {
                paths.push(*path)
            }
        }
        self.set_paths(second, &paths);
    }

    pub fn set_paths(&mut self, index: usize, to: &[usize]) {
        self.data[index].paths.clear();
        for i in to {
            self.data[index].paths.push_back(*i);
        }
    }

    pub fn remove_node(&mut self, index: usize) {
        self.data.remove(index);
        for i in 0..self.data.len() {
            /*
            let mut paths = Vec::new();
            let mut altered = false;
            for path in &self.data[i].paths {
                match path {
                    a if *a > index => {
                        paths.push(*a - 1);
                        altered = true;
                    },
                    b if *b == index => altered = true,
                    c => paths.push(*c),
                }
            }
            if altered {
                self.set_paths(i, &paths);
            }*/

            let mut rem = None;
            let mut curr = -1;
            for path in self.data[i].paths.iter_mut() {
                curr += 1;
                if *path == index {
                    rem = Some(curr)
                }
                else if *path > index {
                    *path -= 1;
                }
            }
            match rem {
                Some(a) => {
                    let mut end = self.data[i].paths.split_off(a);
                    end.pop_front();
                    self.data[i].paths.append(&mut end);
                }
                None => {}
            }
        }
    }
}
