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
    AreaChange(i32, i32),
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
    start: Vec<Box<Symbol>>,
    end: Vec<Box<Symbol>>,
}

impl Graph<Symbol> {

    pub fn apply_rule(&mut self, rule: &Rule) -> bool {
        let subs = self.find_sub_indexes(&rule.start);
        if subs.len() == 0 {
            return false;
        }
        let temp = subs.len();
        let chosen = &subs[rand::thread_rng().gen_range(0, temp)];

        true
    }

    //currently reads a Room-Lock connection as Room-Lock-Room-Lock...
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
            for vec in &mut ret {
                'inner: for path in &self.data[vec[index - 1]].paths {
                    if self.data[*path].value == *sub[index] {
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
