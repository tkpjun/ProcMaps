pub use super::graph::Graph;
pub use rand;
pub use rand::Rng;

#[derive(Eq, PartialEq, Debug)]
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
    QuestChallenge(i32),
    Boss(i32),
    Combat(i32),
    Loot(i32),
    Powerup(i32),
    PowerChallenge(Vec<i32>),
}

pub struct Rule {
    pub start: Vec<Symbol>,
    pub s_paths: Vec<PathType>,
    pub result: Graph<Symbol>,
    //Indexes of the first and last symbols of the start vec in the result graph
    pub anchors: (usize, usize),
}

pub enum PathType {
    Tight,
    Loose,
}

impl Graph<Symbol> {

    /*pub fn apply_rule(&mut self, mut rule: Rule) -> bool {
        let subs = self.find_sub_indexes(&rule.start, &rule.paths);
        if subs.len() == 0 {
            return false;
        }
        let temp = subs.len();
        let chosen = &subs[rand::thread_rng().gen_range(0, temp)];
        let last = chosen.len() - 1;
        if chosen.len() == 2 {
            self.remove_path(chosen[0], chosen[last]);
        }

        let temp = rule.result.len();
        let mut drain = rule.result.drain();
        for i in 0..temp {
            match i {
                a if a < rule.anchors.0 => {
                    self.push_node(drain.next().unwrap());
                    let d_last = self.data.len() - 1;
                    self.add_path(chosen[0], d_last);
                },
                a if a == rule.anchors.0 => {
                    drain.next();
                },
                a if a == rule.anchors.0 + 1 => {
                    self.push_node(drain.next().unwrap());
                    let d_last = self.data.len() - 1;
                    self.add_path(chosen[0], d_last);
                }
                a if a < rule.anchors.1 => {
                    self.push_node(drain.next().unwrap());
                    let d_last = self.data.len() - 1;
                    self.add_path(d_last - 1, d_last);
                }
                a if a == rule.anchors.1 => {
                    drain.next();
                    let d_last = self.data.len() - 1;
                    self.add_path(chosen[last], d_last);
                }
                a if a > rule.anchors.1 => {
                    self.push_node(drain.next().unwrap());
                    let d_last = self.data.len() - 1;
                    self.add_path(chosen[last], d_last);
                }
                _ => panic!("Inexhaustive match!")
            }
        }
        if rule.start.len() > 1 {
            //needs to remove in descending order
            for i in 1..(chosen.len()-1) {
                self.remove_node(chosen[i]);
            }
        }
        true
    }*/

    fn find_sub_indexes(&self, sub: &Vec<Symbol>, paths: &Vec<PathType>) -> Vec<Vec<usize>> {
        let mut ret: Vec<Vec<usize>> = Vec::new();
        for i in 0..self.data.len() {
            let mut try = Vec::new();
            if self.data[i].value == sub[0] {
                try.push(i);
                ret.push(try);
            }
        }
        if sub.len() == 1 {
            return ret;
        }
        //for every remaining node of the linear subgraph
        for index in 1..sub.len() {
            let mut newVecs = Vec::new();
            match paths[index - 1] {
                //If next path is tight, only search the direct paths
                PathType::Tight => {
                    for vec in &mut ret {
                        let last = vec.len() - 1;
                        let mut found1 = false;
                        for path in &self.data[vec[last]].paths {
                            if self.data[*path].value == sub[index] &&
                               vec.iter().all(|x| *x != *path) {
                                if found1 {
                                    let mut new = vec.clone();
                                    new.push(*path);
                                    newVecs.push(new);
                                }
                                else {
                                    vec.push(*path);
                                    found1 = true;
                                }
                            }
                        }
                    }
                }
                //If next path is loose, search with Breadth-First-Search
                PathType::Loose => {
                    for vec in &mut ret {
                        let last = vec.len() - 1;
                        let exc = if vec[last - 1] >= 0 {vec[last - 1]} else {-1};
                        let test = self.bfs(vec[last], exc, &sub[index]);
                        match test {
                            Some(a) => {
                                for i in a.iter().rev() {
                                    vec.push(*i);
                                }
                            }
                            None => {}
                        }
                    }
                }
            }
            //remove the test vectors that had no path to the next searched symbol
            //BUG: too lenient if there are two of the same symbol in a row
            ret = ret.into_iter().filter(|x|{
                let last = x.len() - 1;
                self.data[x[last]].value == sub[index]
            }).collect::<Vec<Vec<usize>>>();
            //add the new post-branch test vectors
            for vec in newVecs.drain() {
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
