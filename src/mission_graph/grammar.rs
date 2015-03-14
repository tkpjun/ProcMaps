use super::graph;
use super::graph::Graph;
use rand;
use rand::Rng;
use std::collections::linked_list::LinkedList;
use std::num::SignedInt;

#[derive(Eq, PartialEq, Debug, Clone)]
pub enum Symbol {
    LevelEntr,
    LevelExit,
    AreaExit(i32),
    AreaEntr(i32),
    Door,
    SecretDoor,
    Key(i32),
    KeyDoor(Vec<i32>),
    Lever(i32),
    SecretLever(i32),
    LeverDoor(Vec<i32>),
    QuestStart(i32),
    QuestTest(i32),
    Boss(i32),
    Combat(i32),
    Loot(i32),
    Powerup(i32),
    PowerTest(Vec<i32>),
}

pub struct Rule {
    //A linear graph to make search computationally tolerable
    pub start: Vec<Symbol>,
    //Path types in the start graph: s_paths.len() == start.len() -1
    pub s_paths: Vec<PathType>,
    pub result: Graph<Symbol>,
    //0: Index of a node in the start graph and its position in the result graph.
    //1: Offset of a 2nd node; 0 means only one anchor node.
    //Nodes between the two anchors in the original graph are removed.
    pub anchor: ((usize, usize), (isize, Anchor)),
}

pub enum PathType {
    Tight,
    Loose,
}

pub enum Anchor {
    Is(usize),
    Connected(usize),
}

impl Graph<Symbol> {

    pub fn apply_rule(&mut self, rule: &Rule) -> bool {
        let subs = self.find_sub_indexes(&rule.start, &rule.s_paths);
        if subs.len() == 0 {
            return false;
        }
        let len = subs.len();
        let random = rand::thread_rng().gen_range(0, len);
        let &(ref sub_i, ref sub_i_i) = &subs[random];
        let len = self.data.len();
        let n1_s = sub_i_i[(rule.anchor.0).0];
        let n1_e = (rule.anchor.0).1;
        let n2_s = (sub_i_i[(rule.anchor.0).0] as isize + (rule.anchor.1).0) as usize;
        let n2_in_res = match (rule.anchor.1).1 {
            Anchor::Is(_) => true,
            _ => false
        };
        let n2_e = match (rule.anchor.1).1 {
            Anchor::Is(a) => a,
            Anchor::Connected(a) => a
        };

        if sub_i.len() == 2 {
            self.remove_path(sub_i[0], sub_i[1]);
        }
        else if (rule.anchor.1).0.abs() == 1 {
            self.remove_path(sub_i[n1_s], sub_i[n2_s]);
        }
        let mut anchors_passed_over = 0;
        //Add the end graph sans anchors to self, update anchors
        for index in 0..rule.result.data.len() {
            if n2_in_res {
                if n1_e != index && n2_e != index {
                    let node = &rule.result.data[index];
                    let paths = node.paths.iter().map(|a|{
                        if n1_e == *a {
                            sub_i[n1_s]
                        }
                        else if n2_e == *a {
                            sub_i[n2_s]
                        }
                        else {
                            a + len - anchors_passed_over
                        }
                    }).collect::<Vec<usize>>();
                    self.push_node(node.value.clone());
                    let last = self.data.len() - 1;
                    self.set_paths(last, &paths);
                }
                else {
                    let a_pair = if n1_e == index {
                        (n1_s, n1_e)
                    }
                    else {
                        (n2_s, n2_e)
                    };
                    let node = &rule.result.data[a_pair.1];
                    self.data[sub_i[a_pair.0]].value = node.value.clone();
                    let mut res_paths = node.paths.iter().map(|a|{
                        *a + len - 1//BUG: Minus anchors passed over when the data point was added
                    }).collect::<LinkedList<usize>>();
                    self.data[sub_i[a_pair.0]].paths.append(&mut res_paths);
                    anchors_passed_over += 1;
                }
            }
            else {
                if n1_e != index {
                    let node = &rule.result.data[index];
                    let paths = node.paths.iter().map(|a|{
                        if n1_e == *a {
                            sub_i[n1_s]
                        }
                        else {
                            a + len - anchors_passed_over
                        }
                    }).collect::<Vec<usize>>();
                    self.push_node(node.value.clone());
                    let last = self.data.len() - 1;
                    self.set_paths(last, &paths);
                    if index == n2_e{
                        self.add_path(last, n2_s);
                    }
                }
                else {
                    let a_pair = (n1_s, n1_e);
                    let node = &rule.result.data[a_pair.1];
                    self.data[sub_i[a_pair.0]].value = node.value.clone();
                    let mut res_paths = node.paths.iter().map(|a|{
                        *a + len - 1//BUG: Minus anchors passed over when the data point was added
                    }).collect::<LinkedList<usize>>();
                    self.data[sub_i[a_pair.0]].paths.append(&mut res_paths);
                    anchors_passed_over += 1;
                }
            }
        }

        //Remove the other nodes
        if sub_i.len() > 2 {
            //needs to remove in descending index order so indexes don't change
            /*for i in (rule.anchors[0].0 + 1)..(rule.anchors[1].0) {
                self.remove_node(sub[i]);
            }*/
        }
        true
    }

    fn find_sub_indexes(&self, sub: &Vec<Symbol>, paths: &Vec<PathType>) -> Vec<(Vec<usize>, Vec<usize>)> {
        let mut ret: Vec<(Vec<usize>, Vec<usize>)> = Vec::new();
        for i in 0..self.data.len() {
            let mut try = Vec::new();
            let mut try_i = Vec::new();
            if self.data[i].value == sub[0] {
                try.push(i);
                try_i.push(0);
                ret.push((try, try_i));
            }
        }
        if sub.len() == 1 {
            return ret;
        }
        //for every remaining node of the linear subgraph
        for index in 1..sub.len() {
            let mut new_vecs = Vec::new();
            match paths[index - 1] {
                //If next path is tight, only search the direct paths
                PathType::Tight => {
                    for tup in &mut ret {
                        let last = tup.0.len() - 1;
                        let mut found1 = false;
                        for path in &self.data[tup.0[last]].paths {
                            if self.data[*path].value == sub[index] &&
                               tup.1.iter().all(|x| *x != *path) {
                                if found1 {
                                    let mut new = tup.0.clone();
                                    let mut new_i = tup.1.clone();
                                    new_i.push(new.len());
                                    new.push(*path);
                                    new_vecs.push((new, new_i));
                                }
                                else {
                                    tup.1.push(tup.0.len());
                                    tup.0.push(*path);
                                    found1 = true;
                                }
                            }
                        }
                    }
                }
                //If next path is loose, search with Breadth-First-Search
                PathType::Loose => {
                    for tup in &mut ret {
                        let last = tup.0.len() - 1;
                        let exc = if tup.0.len() > 1 {tup.0[last - 1]} else {-1};
                        let test = self.bfs(tup.0[last], exc, &sub[index]);
                        match test {
                            Some(a) => {
                                for i in a.into_iter().rev() {
                                    tup.0.push(i);
                                }
                                tup.1.push(tup.0.len() - 1);
                            }
                            None => {}
                        }
                    }
                }
            }
            //remove the test vectors that had no path to the next searched symbol
            ret = ret.into_iter().filter(|&(_, ref y)|{
                y.len() == index + 1
            }).collect::<Vec<(Vec<usize>, Vec<usize>)>>();
            //add the new post-branch test vectors
            for vec in new_vecs.drain() {
                ret.push(vec);
            }
        }
        return ret;
    }
}

impl ToString for Graph<Symbol> {
    fn to_string(&self) -> String {
        let mut s = String::new();
        for node in &self.data {
            s = s + &*format!("{:?}", node.value);
            for path in &node.paths {
                s = s + " -> " + &*path.to_string();
            }
            s = s + "\n";
        }
        s
    }
}
