use std::collections::{HashMap, HashSet, VecDeque};
use std::fmt::Debug;
use std::ops::Deref;
use dyn_eq::DynEq;
use indexmap::IndexMap;
use itertools::Itertools;
use num::integer::lcm;
use regex::Regex;
use tailcall::tailcall;
use crate::common::{EMPTY_STRING_VEC, load_from};

dyn_eq::eq_trait_object!(Module);

pub fn run_day() {
    let data = load_from("day20.txt");
    let mut part_1_modules = parse_modules(data.as_str());
    let mut part_2_modules = parse_modules(data.as_str());
    println!("Part 1: {}", day20a(&mut part_1_modules));
    println!("Part 2: {}", day20b(&mut part_2_modules));
}

fn day20a(modules: &mut IndexMap<String, Box<dyn Module>>) -> u64 {
    let (low, high) = cycle(modules, 1000);
    low * high
}

fn day20b(modules: &mut IndexMap<String, Box<dyn Module>>) -> u64 {
    find_rx(modules)
}

fn parse_modules(data: &str) -> IndexMap<String, Box<dyn Module>> {
    // first, parse each line to be type, (b, %, &), and outputs (...)
    let reg = Regex::new("([%&])?([a-z]+) -> (.+)").unwrap();
    let r: Vec<(&str, &str, Vec<&str>)> = data.lines().map(|x| {
        let captures = reg.captures(x).unwrap();
        let module_type = captures.get(1).map(|x| x.as_str()).unwrap_or("b");
        let module_name = captures.get(2).unwrap().as_str();
        let output_modules: Vec<&str> = captures.get(3).unwrap().as_str().split(", ").collect();
        (module_type, module_name, output_modules)
    }).collect();

    let mut modules: IndexMap<String, Box<dyn Module>> = IndexMap::new();
    let mut final_outputs: HashSet<String> = HashSet::new();
    for (m_type, input, outputs_str) in &r {
        let outputs = outputs_str.iter().sorted().map(|x| String::from(*x)).collect_vec();
        final_outputs.extend(outputs.clone());
        let module: Box<dyn Module> = match *m_type {
            "%" => {
                Box::new(FlipFlop::new(outputs.clone()))
            }
            "&" => {
                let inputs: Vec<&str> = r.iter().filter_map(|(_, potential, list)| {
                    if list.contains(&input) {
                        Some(*potential)
                    } else {
                        None
                    }
                }).collect();
                Box::new(Conjunction::new(inputs.iter().map(|x| String::from(*x)).collect_vec(), outputs.clone()))
            }
            "b" => {
                Box::new(Broadcast::new(outputs.clone()))
            }
            _ => panic!("Nope")
        };
        modules.insert(String::from(*input), module);
    }

    for output in final_outputs {
        if !modules.contains_key(&output) {
            modules.insert(output.clone(), Box::new(Output::new()));
        }
    }

    modules
}

#[derive(Eq, PartialEq, Hash)]
struct Cache {
    state: Vec<bool>
}

impl Cache {
    fn from(modules: &IndexMap<String, Box<dyn Module>>) -> Cache {
        let mut state: Vec<bool> = Vec::new();
        for m in modules.values() {
            for s in m.state() {
                state.push(s);
            }
        }
        Cache { state }
    }
}

fn find_rx(modules: &mut IndexMap<String, Box<dyn Module>>) -> u64 {

    // find what feeds into rx -- looking at data it's a Conjunction module.
    let input = modules.iter()
        .filter(|(_, module)| module.output().contains(&String::from("rx")))
        .map(|(name, _)| name.clone())
        .next().unwrap();

    let mut feed_in: HashSet<String> = modules.get(&input).unwrap().keys_to_watch().iter().map(|x| x.clone()).collect();

    // figure out what happens from the broadcaster
    let mut loops: Vec<u64> = Vec::new();
    let output_from_bcast = modules.get("broadcaster").unwrap().output().clone();

    for bcast in output_from_bcast {
        let mut cache: HashMap<Cache, u64> = HashMap::new();
        cache.insert(Cache::from(modules), 0);
        modules.insert(input.clone(), Box::new(Output::new()));
        let mut counter = 0;
        'outer: loop {
            counter += 1;
            let mut pulses: Vec<(String, Pulse)> = Vec::new();
            send_pulse(VecDeque::from([(String::from("broadcaster"), bcast.clone(), Pulse::Low)]), modules, &mut pulses);
            if let Some(x) = cache.insert(Cache::from(modules), counter) {
                panic!("We got a loop {} -> {}", x, counter);
            }
           // println!("{:?}", modules.get("fd").unwrap());
            let exit = modules.get(&input).unwrap();
            for i in &feed_in.clone() {
                if exit.has_high(i) { // they're all inverters, so all need to get Low to send High.
                    loops.push(counter);
                    feed_in.remove(i);
                    break 'outer;
                }
            }
        }
    }

    loops.iter().map(|x| x.clone()).reduce(|x, y| lcm(x, y)).unwrap()
}


fn cycle(modules: &mut IndexMap<String, Box<dyn Module>>, count: u64) -> (u64, u64) {
    let mut pulses_list: Vec<Vec<Pulse>> = Vec::new();
    let mut counter: u64 = 0;
    let mut low: u64 = 0;
    let mut high: u64 = 0;
    let mut cache: HashMap<Cache, u64> = HashMap::new();

    cache.insert(Cache::from(modules), 0);
    while counter < count {
        let mut pulses: Vec<(String, Pulse)> = Vec::new();
        send_pulse(VecDeque::from([(String::from("button"), String::from("broadcaster"), Pulse::Low)]), modules, &mut pulses);
        let actual_pulses = pulses.iter().map(|(_, p)| p.clone()).collect_vec();
        let low_cycle = actual_pulses.iter().filter(|x| **x == Pulse::Low).count() as u64;
        low += low_cycle;
        high += pulses.len() as u64 - low_cycle;
        pulses_list.push(actual_pulses);
        if let Some(previous) = cache.insert(Cache::from(modules), counter + 1) {
            // we have a repeat, so we find that range and repeat it.
            let range = counter + 1 - previous;
            let left = count - counter - 1;
            let full_cycles = left / range;
            let remainder = left % range;
            // at 0, we need all pulses, at 1, we need all but the first, so we just use previous here.
            let pulses_in_cycle: Vec<&Vec<Pulse>> = pulses_list.iter().skip(previous as usize).collect();

            if remainder != 0 {
                let r: Vec<(u64, Pulse)> = pulses_in_cycle.iter().take(remainder as usize).flat_map(|x| x.iter()).sorted().dedup_with_count().map(|(s, p)| (s as u64, p.clone())).collect();
                low += r.iter().find_or_first(|x| x.1 == Pulse::Low).map(|(x, _)| x.clone()).unwrap_or(0u64);
                high += r.iter().find_or_first(|x| x.1 == Pulse::High).map(|(x, _)| x.clone()).unwrap_or(0u64);
            }
            let full_cycle: Vec<(u64, Pulse)> = pulses_in_cycle.iter().flat_map(|x| x.iter()).sorted().dedup_with_count().map(|(s, p)| (s as u64, p.clone())).collect();
            let l = full_cycle.iter().find_or_first(|x| x.1 == Pulse::Low).map(|(x, _)| x.clone()).unwrap_or(0u64) * full_cycles;
            let h = full_cycle.iter().find_or_first(|x| x.1 == Pulse::High).map(|(x, _)| x.clone()).unwrap_or(0u64) * full_cycles;
            low += l;
            high += h;
            break;
        } else {
            counter += 1;
        }
    }

    (low, high)
}

#[tailcall]
fn send_pulse(modules_to_run: VecDeque<(String, String, Pulse)>, modules: &mut IndexMap<String, Box<dyn Module>>, pulses: &mut Vec<(String, Pulse)>) {
    let mut m = modules_to_run;

    fn run_module(module: &mut Box<dyn Module>, incoming_module: &str, pulse: &Pulse) -> Option<(Pulse, Vec<String>)> {
        module.receive(pulse, incoming_module).map(|x| (x, module.output().clone()))
    }

    let mut next: VecDeque<(String, String, Pulse)> = VecDeque::new();
    while let Some((from, module, pulse)) = m.pop_front() {
        pulses.push((from.clone(), pulse.clone()));
        let m = modules.get_mut(module.as_str()).unwrap();
        if let Some((pulse, next_modules)) = run_module(m, from.as_str(), &pulse) {
            next_modules.iter().map(|x| (module.clone(), x.clone(), pulse.clone())).for_each(|x| next.push_back(x));
        }
    }

    if !next.is_empty() {
        send_pulse(next, modules, pulses)
    }
}

#[repr(u8)]
#[derive(Debug, Clone, Hash, PartialEq, Eq, PartialOrd, Ord)]
enum Pulse {
    Low,
    High
}

// Modules
trait Module: DynEq + Debug {
    fn receive(&mut self, pulse: &Pulse, input: &str) -> Option<Pulse>;
    fn output(&self) -> &Vec<String>;

    fn state(&self) -> Vec<bool> {
        Vec::new()
    }

    fn keys_to_watch(&self) -> Vec<String> {
        Vec::new()
    }

    fn has_high(&self, _: &String) -> bool {
        false
    }
}

#[derive(PartialEq, Eq, Debug)]
struct Broadcast {
    outputs: Vec<String>
}

impl Broadcast {
    fn new(outputs: Vec<String>) -> Broadcast {
        Broadcast {
            outputs
        }
    }
}

impl Module for Broadcast {
    fn receive(&mut self, pulse: &Pulse, _: &str) -> Option<Pulse> {
        Some(pulse.clone())
    }

    fn output(&self) -> &Vec<String> {
        &self.outputs
    }
}

#[derive(PartialEq, Eq, Debug)]
struct FlipFlop {
    outputs: Vec<String>,
    state: bool
}

impl FlipFlop {
    fn new(outputs: Vec<String>) -> FlipFlop {
        FlipFlop {
            outputs,
            state: false
        }
    }
}

impl Module for FlipFlop {
    fn receive(&mut self, pulse: &Pulse, _: &str) -> Option<Pulse> {
        match pulse {
            Pulse::Low => {
                self.state = !self.state;
                if self.state {
                    Some(Pulse::High)
                } else {
                    Some(Pulse::Low)
                }
            },
            Pulse::High => None
        }
    }
    fn output(&self) -> &Vec<String> {
        &self.outputs
    }

    fn state(&self) -> Vec<bool> {
        Vec::from([self.state])
    }
}

#[derive(PartialEq, Eq, Debug)]
struct Conjunction {
    outputs: Vec<String>,
    high_from: IndexMap<String, Pulse>
}

impl Conjunction {
    fn new(inputs: Vec<String>, outputs: Vec<String>) -> Conjunction {
        let mut high_from: IndexMap<String, Pulse> = IndexMap::new();
        for x in inputs.iter().map(|x| (x.clone(), Pulse::Low)) {
            high_from.insert(x.0, x.1);
        }
        Conjunction {
            outputs,
            high_from
        }
    }
}

impl Module for Conjunction {

    fn receive(&mut self, pulse: &Pulse, input: &str) -> Option<Pulse> {
        self.high_from.insert(String::from(input), pulse.clone());
        if self.high_from.values().all(|x| *x == Pulse::High) {
            Some(Pulse::Low)
        } else {
            Some(Pulse::High)
        }
    }
    fn output(&self) -> &Vec<String> {
        &self.outputs
    }

    fn state(&self) -> Vec<bool> {
        self.high_from.values().map(|x| *x == Pulse::High).collect_vec()
    }

    fn keys_to_watch(&self) -> Vec<String> {
        self.high_from.keys().map(|x| x.clone()).collect_vec()
    }
}

#[derive(PartialEq, Eq, Debug)]
struct Output {
    last_received: HashSet<String>
}

impl Output {
    fn new() -> Output {
        Output { last_received: HashSet::new() }
    }
}

impl Module for Output {

    fn receive(&mut self, p: &Pulse, input: &str) -> Option<Pulse> {
        if *p == Pulse::High {
            self.last_received.insert(String::from(input));
        }
        None
    }

    fn output(&self) -> &Vec<String> {
        EMPTY_STRING_VEC.deref()
    }

    fn has_high(&self, from: &String) -> bool {
        self.last_received.contains(from)
    }
}

#[cfg(test)]
mod test {
    use indexmap::IndexMap;
    use indoc::indoc;
    use rstest::rstest;
    use crate::day20::{Broadcast, Conjunction, day20a, FlipFlop, Module, Output, parse_modules, Pulse};

    const TEST_DATA_1: &str = indoc! {
        "broadcaster -> a, b, c
         %a -> b
         %b -> c
         %c -> inv
         &inv -> a"
    };

    fn parsed_data_1() -> IndexMap<String, Box<dyn Module>> {
        IndexMap::from([
            (String::from("broadcaster"), Box::new(Broadcast::new(vec![String::from("a"), String::from("b"), String::from("c")])) as Box<dyn Module>),
            (String::from("a"), Box::new(FlipFlop::new(vec![String::from("b")])) as Box<dyn Module>),
            (String::from("b"), Box::new(FlipFlop::new(vec![String::from("c")])) as Box<dyn Module>),
            (String::from("c"), Box::new(FlipFlop::new(vec![String::from("inv")])) as Box<dyn Module>),
            (String::from("inv"), Box::new(Conjunction::new(vec![String::from("c")], vec![String::from("a")])) as Box<dyn Module>)
        ])
    }

    const TEST_DATA_2: &str = indoc! { "broadcaster -> a
                                        %a -> inv, con
                                        &inv -> b
                                        %b -> con
                                        &con -> output" };

    fn parsed_data_2() -> IndexMap<String, Box<dyn Module>> {
        IndexMap::from([
            (String::from("broadcaster"), Box::new(Broadcast::new(vec![String::from("a")])) as Box<dyn Module>),
            (String::from("a"), Box::new(FlipFlop::new(vec![String::from("con"), String::from("inv")])) as Box<dyn Module>),
            (String::from("inv"), Box::new(Conjunction::new(vec![String::from("a")], vec![String::from("b")])) as Box<dyn Module>),
            (String::from("b"), Box::new(FlipFlop::new(vec![String::from("con")])) as Box<dyn Module>),
            (String::from("con"), Box::new(Conjunction::new(vec![String::from("a"), String::from("b")], vec![String::from("output")])) as Box<dyn Module>),
            (String::from("output"), Box::new(Output::new()) as Box<dyn Module>)
        ])
    }

    #[test]
    fn test_parse_modules() {
        assert_eq!(parse_modules(TEST_DATA_1), parsed_data_1());
    }

    #[test]
    fn test_parse_modules_2() {
        assert_eq!(parse_modules(TEST_DATA_2), parsed_data_2());
    }

    #[test]
    fn test_day20a_1() {
        let mut data = parse_modules(TEST_DATA_1);
        assert_eq!(day20a(&mut data), 32000000);
    }

    #[test]
    fn test_day20a_2() {
        let mut data = parse_modules(TEST_DATA_2);
        assert_eq!(day20a(&mut data), 11687500);
    }


    #[rstest]
    #[case(false, Pulse::High, None, false)]
    #[case(true, Pulse::High, None, true)]
    #[case(false, Pulse::Low, Some(Pulse::High), true)]
    #[case(true, Pulse::Low, Some(Pulse::Low), false)]
    fn test_flip_flop(#[case] state: bool, #[case] input: Pulse, #[case] output: Option<Pulse>, #[case] expected_state: bool) {
        let mut flip = FlipFlop::new(vec![String::from("a")]);
        flip.state = state;
        let result = flip.receive(&input, "_");
        assert_eq!(flip.state, expected_state);
        assert_eq!(result, output);
    }

    #[rstest]
    #[case(IndexMap::from([(String::from("a"), Pulse:: Low)]), IndexMap::from([(String::from("a"), Pulse:: High)]), Pulse::High, Pulse::Low, "a")]
    #[case(IndexMap::from([(String::from("a"), Pulse:: High)]), IndexMap::from([(String::from("a"), Pulse:: High)]), Pulse::High, Pulse::Low, "a")]
    #[case(IndexMap::from([(String::from("a"), Pulse:: High)]), IndexMap::from([(String::from("a"), Pulse:: Low)]), Pulse::Low, Pulse::High, "a")]
    #[case(IndexMap::from([(String::from("a"), Pulse:: High), (String::from("b"), Pulse:: High),]), IndexMap::from([(String::from("a"), Pulse:: Low), (String::from("b"), Pulse:: High)]), Pulse::Low, Pulse::High, "a")]
    #[case(IndexMap::from([(String::from("a"), Pulse:: Low), (String::from("b"), Pulse:: Low),]), IndexMap::from([(String::from("a"), Pulse:: Low), (String::from("b"), Pulse:: Low)]), Pulse::Low, Pulse::High, "a")]
    #[case(IndexMap::from([(String::from("a"), Pulse:: Low), (String::from("b"), Pulse:: High),]), IndexMap::from([(String::from("a"), Pulse:: High), (String::from("b"), Pulse:: High)]), Pulse::High, Pulse::Low, "a")]
    #[case(IndexMap::from([(String::from("a"), Pulse:: High), (String::from("b"), Pulse:: High),]), IndexMap::from([(String::from("a"), Pulse:: High), (String::from("b"), Pulse:: High)]), Pulse::High, Pulse::Low, "a")]
    #[case(IndexMap::from([(String::from("a"), Pulse:: Low), (String::from("b"), Pulse:: Low),]), IndexMap::from([(String::from("a"), Pulse:: High), (String::from("b"), Pulse:: Low)]), Pulse::High, Pulse::High, "a")]
    fn test_conjunction(#[case] initial_state: IndexMap<String, Pulse>, #[case] final_state: IndexMap<String, Pulse>, #[case] pulse: Pulse, #[case] expected: Pulse, #[case] from: &str) {
        let mut conjunction = Conjunction {
            high_from: initial_state.clone(),
            outputs: vec![]
        };

        let result = conjunction.receive(&pulse, from);
        assert_eq!(conjunction.high_from, final_state);
        assert_eq!(result, Some(expected));
    }

    #[rstest]
    #[case(vec![Pulse::High, Pulse::Low])]
    #[case(vec![Pulse::High])]
    #[case(vec![Pulse::Low, Pulse::High])]
    #[case(vec![Pulse::Low])]
    fn test_output_module(#[case] expected: Vec<Pulse>) {
        let mut output = Output::new();
        assert_eq!(output.has_high(&String::from("")), false);
        for p in &expected {
            output.receive(&p, "");
        }
        assert_eq!(output.has_high(&String::from("")), expected.contains(&Pulse::High));
    }
}