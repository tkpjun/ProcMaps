pub use super::graph::Graph;

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
    start: Graph<Symbol>,
    end: Graph<Symbol>,
}

impl Graph<Symbol> {

    pub fn apply_rule(&mut self, rule: &Rule) -> bool {

    }

    fn find_sub_indexes(&self, sub: &Graph<Symbol>) -> Option<Vec<Vec<usize>>> {
        let mut ret = Vec::new();
        for s_node in sub.data {
            let mut inst = Vec::new();
            for i in 0..self.data.len() {
                if self.data[i].value == s_node.value {
                    let non_matches = s_node.paths.iter().filter(|x| {
                        self.data[i].paths.iter().any(|y| *x == y)
                    }).collect::<Vec<&usize>>();
                    if non_matches.len() == 0 {
                        inst.push(i);
                    }
                }
            }
            if inst.len() > 0 {
                ret.push(inst);
            }
            else { return None; }
        }
        Some(ret)
    }
}
