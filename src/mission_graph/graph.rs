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

    /*pub fn add_node_between(&mut self, value: T, first: usize, second: usize) {
        let mut node = Node{ value: value, paths: LinkedList::new() };
        node.paths.push_back(first);
        node.paths.push_back(second);
        self.data.push(node);

        let mut paths = Vec::new();
        for path in &self.data[first].paths {
            if *path != second {
                paths.push(*path)
            }
        }
        self.set_paths(first, &paths);

        let mut paths = Vec::new();
        for path in &self.data[second].paths {
            if *path != first {
                paths.push(*path)
            }
        }
        self.set_paths(second, &paths);
    }*/

    pub fn remove_node(&mut self, index: usize) {
        self.data.swap_remove(index);
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

    //Doesnt make sure the opposite end has the same path
    pub fn set_paths(&mut self, index: usize, to: &Vec<usize>) {
        self.data[index].paths.clear();
        for i in to {
            self.data[index].paths.push_back(*i);
        }
    }

    pub fn add_path(&mut self, from: usize, to: usize) {
        self.data[from].paths.push_back(to);
        self.data[to].paths.push_back(from);
    }

    pub fn remove_path(&mut self, from: usize, to: usize) {
        let mut i = 0;
        for p in &mut self.data[from].paths {
            if *p == to {
                break;
            }
            i += 1;
        }
        let mut end = self.data[from].paths.split_off(i);
        end.pop_front();
        self.data[from].paths.append(&mut end);

        i = 0;
        for p in &mut self.data[to].paths {
            if *p == from {
                break;
            }
            i += 1;
        }
        end = self.data[to].paths.split_off(i);
        end.pop_front();
        self.data[to].paths.append(&mut end);
    }
}

impl<T: Eq> Graph<T> {
    //Returns the shortest path to the target in reverse: target is result[0].
    pub fn bfs(&self, start: usize, exclude: usize, target: &T) -> Option<Vec<usize>> {
        let mut search = vec![TreeNode{val: start, parent: 0}];
        let mut marked = vec![exclude];
        let mut i = 0;
        let buildRet = |space: &Vec<TreeNode<usize>>| -> Option<Vec<usize>> {
            let index = space.len() - 1;
            let mut ret = vec![space[index].val];
            let mut next = space[index].parent;
            while next > 0 {
                ret.push(space[next].val);
                next = space[next].parent;
            }
            return Some(ret);
        };

        while i <= search.len() {
            for path in &self.data[search[i].val].paths {
                if self.data[*path].value == *target {
                    search.push(TreeNode{val: *path, parent: i});
                    return buildRet(&search);
                }
                if marked.iter().all(|a| *a != *path) {
                    marked.push(*path);
                    search.push(TreeNode{val: *path, parent: i});
                }
            }
            i += 1;
        }
        None
    }
}

struct TreeNode<T> {
    val: T,
    parent: usize,
}