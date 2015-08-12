use super::contract::GraphContract;
use super::rule::Rule;
use super::graph::DirectedGraph;
use super::labels::Symbol;
use super::labels::SymbolSet;
use rand::{self, Rng, SeedableRng, XorShiftRng};
use super::Either::{self, One, Another};

pub struct RuleSet<S: Symbol, T: Symbol, U: SymbolSet<S>, V: SymbolSet<T>> {
    rules: Vec<Rule<S, T, U, V>>,
    weights: Vec<f32>, //for choosing which rule to apply
    weight_shift: Box<Fn(f32, u32) -> f32>,
    //hard_maximums: Vec<Option<i32>>,
    contract: GraphContract,
    graph: Option<DirectedGraph<S, T>>,
    rounds_taken: u32,
    rng: XorShiftRng,
    //target_params, //for choosing which subgraph to apply the rule on
}

impl<S: Symbol, T: Symbol, U: SymbolSet<S>, V: SymbolSet<T>> RuleSet<S, T, U, V> {

    pub fn new(rules: Vec<Rule<S, T, U, V>>,
               weights: Either<f32, Vec<f32>>,
               weight_shift: Box<Fn(f32, u32) -> f32>,
               contract: GraphContract)
               -> RuleSet<S, T, U, V> {
        let ws = match weights {
            One(w) => {
                let mut vec = Vec::with_capacity(rules.len());
                for _ in &rules {
                    vec.push(w);
                }
                vec
            },
            Another(vec) => vec
        };
        if ws.len() != rules.len() {
            panic!("Rules and weights vectors' lengths don't match!");
        }
        RuleSet{
            rules: rules,
            weights: ws,
            weight_shift: weight_shift,
            contract: contract,
            rounds_taken: 0,
            graph: None,
            rng: rand::weak_rng()
        }
    }

    pub fn put_graph(&mut self, graph: DirectedGraph<S, T>) {
        assert!(self.contract.holds_for(&graph));
        self.graph = Some(graph);
    }

    pub fn take_graph(&mut self) -> Option<DirectedGraph<S, T>> {
        self.rounds_taken = 0;
        self.graph.take()
    }

    pub fn borrow_graph(&self) -> Option<&DirectedGraph<S, T>> {
        self.graph.as_ref()
    }

    pub fn set_gen_seed(&mut self, seed: [u32; 4]) {
        self.rng.reseed(seed);
    }

    pub fn apply_rounds(&mut self, amount: u32) -> bool {
        let mut alt_weights = self.weights.clone();
        let mut round = 0;
        while round < amount {
            let mut point = self.rng.gen_range(0f32, alt_weights.iter().fold(0f32, |a, b| a + b));
            let index = {
                let mut res = 0;
                for i in 0..alt_weights.len() {
                    if alt_weights[i] >= point {
                        res = i;
                        break;
                    }
                    else {
                        point -= alt_weights[i];
                    }
                }
                res
            };

            let rule = &self.rules[index];
            let graph = self.graph.as_mut().expect("No graph assigned to ruleset!");
            let subs = rule.find_subgraphs(graph);
            if subs.len() > 0 {
                let sub = &subs[self.rng.gen_range(0, subs.len())];
                rule.apply_to(graph, sub);

                self.rounds_taken += 1;
                round += 1;
                let new = &*self.weight_shift;
                self.weights[index] = new(self.weights[index], self.rounds_taken);
                if round < amount { alt_weights = self.weights.clone(); }
            }
            else {
                alt_weights[index] = 0f32;
                if alt_weights.iter().all(|&a| a == 0f32) {
                    return false;
                }
            }
        }
        true
    }

    pub fn apply_until_finished(&mut self) {
        let not_finished = self.apply_rounds(1);
        if not_finished == false {
            return;
        }
    }
}
