use super::contract::GraphContract;
use super::rule::Rule;
use super::graph::DirectedGraph;
use super::labels::Symbol;
use super::labels::RichSymbol;
use super::labels::SymbolSet;
use rand::{self, Rng, SeedableRng, XorShiftRng};
use super::Either::{self, Single, List};

pub struct RuleSet<S: RichSymbol, T: Symbol, U: RichSymbol + SymbolSet<S>, V: SymbolSet<T>> {
    rules: Vec<Rule<S, T, U, V>>,
    original_weights: Vec<f32>,
    weights: Vec<f32>, //for choosing which rule to apply
    weight_shift: Box<Fn(&[f32], usize, i32) -> f32>,
    contract: GraphContract,
    graph: Option<DirectedGraph<S, T>>,
    rounds_taken: i32,
    rng: XorShiftRng,
    //target_params, //for choosing which subgraph to apply the rule on
}

impl<S: RichSymbol, T: Symbol, U: RichSymbol + SymbolSet<S>, V: SymbolSet<T>> RuleSet<S, T, U, V> {

    pub fn new(rules: Vec<Rule<S, T, U, V>>,
               weights: Either<f32, Vec<f32>>,
               weight_shift: Box<Fn(&[f32], usize, i32) -> f32>,
               contract: GraphContract)
               -> RuleSet<S, T, U, V> {
        let ws = match weights {
            Single(w) => {
                let mut vec = Vec::with_capacity(rules.len());
                for _ in &rules {
                    vec.push(w);
                }
                vec
            },
            List(vec) => vec
        };
        if ws.len() != rules.len() {
            panic!("Rules and weights vectors' lengths don't match!");
        }
        RuleSet{
            rules: rules,
            original_weights: ws.clone(),
            weights: ws,
            weight_shift: weight_shift,
            contract: contract,
            rounds_taken: 0,
            graph: None,
            rng: rand::weak_rng()
        }
    }

    pub fn len(&self) -> usize {
        self.rules.len()
    }

    pub fn put_graph(&mut self, graph: DirectedGraph<S, T>) {
        assert!(self.contract.holds_for(&graph));
        self.graph = Some(graph);
    }

    pub fn take_graph(&mut self) -> Option<DirectedGraph<S, T>> {
        assert!(if let Some(g) = self.graph.as_ref() {
            self.contract.holds_for(g)
        }
        else {
            true
        });
        self.rounds_taken = 0;
        self.weights = self.original_weights.clone();
        self.graph.take()
    }

    pub fn borrow_graph(&self) -> Option<&DirectedGraph<S, T>> {
        self.graph.as_ref()
    }

    pub fn set_gen_seed(&mut self, seed: [u32; 4]) {
        self.rng.reseed(seed);
    }

    pub fn apply_rule(&mut self, index: usize) -> bool {
        let rule = &self.rules[index];
        let graph = self.graph.as_mut().expect("No graph assigned to ruleset!");
        let subs = rule.find_subgraphs(graph);
        if subs.len() > 0 {
            let &(ref sub, ref base) = &subs[self.rng.gen_range(0, subs.len())];
            rule.apply_to(graph, sub, base);
            self.rounds_taken += 1;
            let new_wt = &*self.weight_shift;
            self.weights[index] = new_wt(&self.weights, index, self.rounds_taken);
        }
        else {
            return false;
        }
        true
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
            /*
            let rule = &self.rules[index];
            let graph = self.graph.as_mut().expect("No graph assigned to ruleset!");
            let subs = rule.find_subgraphs(graph);
            if subs.len() > 0 {
                let sub = &subs[self.rng.gen_range(0, subs.len())];
                rule.apply_to(graph, sub);

                self.rounds_taken += 1;
                round += 1;
                let new_wt = &*self.weight_shift;
                self.weights[index] = new_wt(&self.weights, index, self.rounds_taken);
                if round < amount { alt_weights = self.weights.clone(); }
            }*/
            let sub_found = self.apply_rule(index);
            if sub_found {
                round += 1;
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
