pub use super::graph::Graph;
pub use rand;
pub use rand::Rng;

#[derive(Eq, PartialEq)]
pub enum Symbol {
    Key(i32),
    Lock(i32),
    Quest(i32),
    QuestEnd(i32),
    LevelEntr,
    LevelExit,
    AreaStart(i32),
    Boss(i32),
    Combat(i32),
    Puzzle(i32),
    Loot(i32),
    Powerup(i32),
    PowerLock(i32),
    Secret,
    Empty,
}

pub struct Rule {
    pub start: Vec<Box<Symbol>>,
    pub end: Vec<Box<Symbol>>,
    pub anchors: (usize, usize),
}

impl Graph<Symbol> {

    pub fn apply_rule(&mut self, mut rule: Rule) -> bool {
        let subs = self.find_sub_indexes(&rule.start);
        if subs.len() == 0 {
            return false;
        }
        let temp = subs.len();
        let chosen = &subs[rand::thread_rng().gen_range(0, temp)];
        let last = chosen.len() - 1;
        if chosen.len() == 2 {
            self.remove_path(chosen[0], chosen[last]);
        }

        let temp = rule.end.len();
        let mut drain = rule.end.drain();
        for i in 0..temp {
            match i {
                a if a < rule.anchors.0 => {
                    self.push_node(*drain.next().unwrap());
                    let d_last = self.data.len() - 1;
                    self.add_path(chosen[0], d_last);
                },
                a if a == rule.anchors.0 => {
                    drain.next();
                },
                a if a == rule.anchors.0 + 1 => {
                    self.push_node(*drain.next().unwrap());
                    let d_last = self.data.len() - 1;
                    self.add_path(chosen[0], d_last);
                }
                a if a < rule.anchors.1 => {
                    self.push_node(*drain.next().unwrap());
                    let d_last = self.data.len() - 1;
                    self.add_path(d_last - 1, d_last);
                }
                a if a == rule.anchors.1 => {
                    drain.next();
                    let d_last = self.data.len() - 1;
                    self.add_path(chosen[last], d_last);
                }
                a if a > rule.anchors.1 => {
                    self.push_node(*drain.next().unwrap());
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
    }

    fn find_sub_indexes(&self, sub: &Vec<Box<Symbol>>) -> Vec<Vec<usize>> {
        let mut ret: Vec<Vec<usize>> = Vec::new();
        for i in 0..self.data.len() {
            let mut try = Vec::new();
            if self.data[i].value == *sub[0] {
                try.push(i);
                ret.push(try);
            }
        }
        if sub.len() == 1 {
            return ret;
        }
        //for every remaining node of the linear subgraph
        for index in 1..sub.len() {
            //for every vector in the return candidates
            for vec in &mut ret {
                //for every path for the return candidates
                'inner: for path in &self.data[vec[index - 1]].paths {
                    if self.data[*path].value == *sub[index] &&
                       vec.iter().all(|x| *x != *path) {
                        vec.push(*path);
                        break 'inner;
                    }
                }
            }
            ret = ret.into_iter().filter(|x| x.len() == index + 1).collect::<Vec<Vec<usize>>>();
        }
        return ret;
    }
}
