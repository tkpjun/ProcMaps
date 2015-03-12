pub use super::graph;
pub use super::graph::Graph;
pub use rand;
pub use rand::Rng;

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
    //Indexes of the nodes the result graph is built between
    //and their corresponding positions in the result graph data type.
    //Nodes between the two anchors in the original graph are removed.
    pub anchors: Vec<(usize, usize)>,
}

pub enum PathType {
    Tight,
    Loose,
}

impl Graph<Symbol> {

    pub fn apply_rule(&mut self, rule: &Rule) -> bool {
        let subs = self.find_sub_indexes(&rule.start, &rule.s_paths);
        if subs.len() == 0 {
            return false;
        }
        let len = subs.len();
        let random = rand::thread_rng().gen_range(0, len);
        let &(ref sub, ref sub_short) = &subs[random];
        let len = self.data.len();
        if sub.len() == 2 {
            self.remove_path(sub[0], sub[1]);
        }

        let mut skipped = 0;
        //Add the end graph sans anchors to self, update anchors
        for index in 0..rule.result.data.len() {
            if rule.anchors.iter().all(|&(_,b)| index != b) {
                let node = &rule.result.data[index];
                let paths = node.paths.iter().map(|a|{
                    let opt = rule.anchors.iter().find(|t| *a == t.1);
                    match opt {
                        Some(c) => sub_short[c.0],
                        None => a + len - skipped,
                    }
                }).collect::<Vec<usize>>();
                self.push_node(node.value.clone());
                let last = self.data.len() - 1;
                self.set_paths(last, &paths);
            }
            else {
                skipped += 1;
                let a_pair = rule.anchors.iter().find(|t| t.1 == index).unwrap();
                self.data[sub_short[a_pair.0]].value = rule.result.data[a_pair.1].value.clone();
                //let paths = self[chosen[tup.0]].paths
            }
        }

        //Remove the other nodes
        if sub.len() > 2 && rule.anchors.len() == 2 {
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
            let mut try2 = Vec::new();
            if self.data[i].value == sub[0] {
                try.push(i);
                try2.push(i);
                ret.push((try, try2));
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
                                    let mut new_2 = tup.1.clone();
                                    new.push(*path);
                                    new_2.push(*path);
                                    new_vecs.push((new, new_2));
                                }
                                else {
                                    tup.0.push(*path);
                                    tup.1.push(*path);
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
                                tup.1.push(tup.0[tup.0.len() - 1]);
                            }
                            None => {}
                        }
                    }
                }
            }
            //remove the test vectors that had no path to the next searched symbol
            ret = ret.into_iter().filter(|&(ref x, ref y)|{
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
