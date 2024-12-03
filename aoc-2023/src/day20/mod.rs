use std::collections::{HashMap, VecDeque};

use aoc_common::util;

type Id = usize;

#[derive(Clone, Copy, PartialEq, Eq, Debug)]
enum Signal {
    Low,
    High,
}

#[derive(Clone, Copy, PartialEq, Eq, Debug)]
enum State {
    On,
    Off,
}

#[derive(Debug)]
struct Pulse {
    from: Id,
    to: Id,
    signal: Signal,
}

#[derive(Debug)]
struct Broadcaster {
    outputs: Vec<Id>,
}

impl Broadcaster {
    const fn new() -> Self {
        Self { outputs: vec![] }
    }

    fn send(&self, pulse: &Pulse) -> impl Iterator<Item = Pulse> + '_ {
        let pulse_to = pulse.to;
        let pulse_signal = pulse.signal;
        self.outputs.iter().map(move |out| Pulse {
            from: pulse_to,
            to: *out,
            signal: pulse_signal,
        })
    }
}

#[derive(Debug)]
struct FlipFlop {
    outputs: Vec<Id>,
    state: State,
}

impl FlipFlop {
    const fn new() -> Self {
        Self { outputs: vec![], state: State::Off }
    }

    fn send(&mut self, pulse: &Pulse) -> impl Iterator<Item = Pulse> + '_ {
        let pulse_to = pulse.to;
        let signal = match (pulse.signal, self.state) {
            (Signal::Low, State::Off) => {
                self.state = State::On;
                Some(Signal::High)
            }
            (Signal::Low, State::On) => {
                self.state = State::Off;
                Some(Signal::Low)
            }
            (Signal::High, _) => None,
        };

        let slice = match signal {
            Some(_) => &self.outputs[..],
            None => &self.outputs[..0],
        };

        slice.iter().map(move |out| Pulse {
            from: pulse_to,
            to: *out,
            signal: signal.unwrap(),
        })
    }
}

#[derive(Debug)]
struct Conjunction {
    inputs: HashMap<Id, Signal>,
    outputs: Vec<Id>,
}

impl Conjunction {
    fn new() -> Self {
        Self {
            inputs: HashMap::new(),
            outputs: vec![],
        }
    }

    fn send(&mut self, pulse: &Pulse) -> impl Iterator<Item = Pulse> + '_ {
        *self.inputs.get_mut(&pulse.from).expect("Input was not added to hashmap") = pulse.signal;

        let pulse_to = pulse.to;

        let out_signal = if self.inputs.values().all(|s| *s == Signal::High) {
            Signal::Low
        } else {
            Signal::High
        };

        self.outputs.iter().map(move |out| Pulse {
            from: pulse_to,
            to: *out,
            signal: out_signal,
        })
    }
}

#[derive(Debug)]
enum Module {
    Empty,
    Broadcaster(Broadcaster),
    FlipFlop(FlipFlop),
    Conjunction(Conjunction),
}

impl Module {
    fn add_output(&mut self, output: Id) {
        match self {
            Self::Empty => {}
            Self::Broadcaster(m) => m.outputs.push(output),
            Self::FlipFlop(m) => m.outputs.push(output),
            Self::Conjunction(m) => m.outputs.push(output),
        }
    }

    fn get_outputs(&self) -> &[Id] {
        match self {
            Self::Empty => &[],
            Self::Broadcaster(m) => &m.outputs,
            Self::FlipFlop(m) => &m.outputs,
            Self::Conjunction(m) => &m.outputs,
        }
    }

    fn add_input(&mut self, input: Id) {
        if let Self::Conjunction(m) = self {
            m.inputs.insert(input, Signal::Low);
        }
    }

    fn reset(&mut self) {
        match self {
            Self::FlipFlop(m) => m.state = State::Off,
            Self::Conjunction(m) => m.inputs.values_mut().for_each(|s| *s = Signal::Low),
            _ => {}
        }
    }
}

struct ModuleMap {
    name_to_id: HashMap<String, Id>,
    modules: Vec<Module>,
}

impl ModuleMap {
    fn new() -> Self {
        Self {
            name_to_id: HashMap::new(),
            modules: vec![],
        }
    }

    fn get_or_insert(&mut self, name: String) -> Id {
        if let Some(id) = self.name_to_id.get(&name) {
            *id
        } else {
            self.modules.push(Module::Empty);
            self.name_to_id.insert(name, self.modules.len() - 1);
            self.modules.len() - 1
        }
    }

    fn get_id(&self, name: &str) -> Option<Id> {
        self.name_to_id.get(name).copied()
    }

    fn get_by_id_mut(&mut self, id: Id) -> &mut Module {
        &mut self.modules[id]
    }

    fn insert(&mut self, name: String, module: Module) -> Id {
        let id = self.get_or_insert(name);
        *self.get_by_id_mut(id) = module;
        id
    }

    fn add_output(&mut self, id: Id, output: String) {
        let output_id = self.get_or_insert(output);
        self.get_by_id_mut(id).add_output(output_id);
    }

    fn flush_inputs(&mut self) {
        let mut inputs = vec![];
        for (id, module) in self.modules.iter().enumerate() {
            for output in module.get_outputs() {
                inputs.push((*output, id));
            }
        }
        for (id, input) in inputs {
            self.get_by_id_mut(id).add_input(input);
        }
    }

    fn reset_modules(&mut self) {
        self.modules.iter_mut().for_each(Module::reset);
    }
}

pub fn press_buttons(lines: impl Iterator<Item = String>) -> util::GenericResult<(usize, usize)> {
    let mut modules = ModuleMap::new();

    for line in lines {
        let (name, outputs_str) = line.split_once(" -> ").expect("Invalid input line");

        let id = if name == "broadcaster" {
            modules.insert(name.to_owned(), Module::Broadcaster(Broadcaster::new()))
        } else {
            match name.split_at(1) {
                ("&", name) => modules.insert(name.to_owned(), Module::Conjunction(Conjunction::new())),
                ("%", name) => modules.insert(name.to_owned(), Module::FlipFlop(FlipFlop::new())),
                _ => panic!("Invalid input line"),
            }
        };

        for output in outputs_str.split(", ") {
            modules.add_output(id, output.to_owned());
        }
    }

    let button_id = modules.insert("button".to_owned(), Module::Empty);
    let broadcaster_id = modules.get_id("broadcaster").unwrap();

    modules.add_output(button_id, "broadcaster".to_owned());

    modules.flush_inputs();
    // println!("{:#?}", modules.name_to_id);

    let mut low_count = 0;
    let mut high_count = 0;

    for _ in 0..1000 {
        let mut queue = VecDeque::new();
        queue.push_back(Pulse {
            from: button_id,
            to: broadcaster_id,
            signal: Signal::Low,
        });

        while let Some(pulse) = queue.pop_front() {
            // println!("{} -{:?}-> {}", pulse.from, pulse.signal, pulse.to);

            match pulse.signal {
                Signal::Low => low_count += 1,
                Signal::High => high_count += 1,
            }

            match modules.get_by_id_mut(pulse.to) {
                Module::Empty => {}
                Module::Broadcaster(m) => queue.extend(m.send(&pulse)),
                Module::FlipFlop(m) => queue.extend(m.send(&pulse)),
                Module::Conjunction(m) => queue.extend(m.send(&pulse)),
            }

            // println!("Queue: {:?}", queue);
        }
    }

    modules.reset_modules();
    let ls_id = modules.get_id("ls").unwrap();
    let mut presses = 0;

    let mut input_flips: HashMap<usize, Vec<usize>> = HashMap::new();

    loop {
        let mut queue = VecDeque::new();
        queue.push_back(Pulse {
            from: button_id,
            to: broadcaster_id,
            signal: Signal::Low,
        });

        presses += 1;

        while let Some(pulse) = queue.pop_front() {
            // println!("{} -{:?}-> {}", pulse.from, pulse.signal, pulse.to);

            if pulse.to == ls_id && pulse.signal == Signal::High {
                input_flips.entry(pulse.from).or_default().push(presses);
            }

            match modules.get_by_id_mut(pulse.to) {
                Module::Empty => {}
                Module::Broadcaster(m) => queue.extend(m.send(&pulse)),
                Module::FlipFlop(m) => queue.extend(m.send(&pulse)),
                Module::Conjunction(m) => queue.extend(m.send(&pulse)),
            }

            // println!("Queue: {:?}", queue);
        }

        if presses == 100_000 {
            break;
        }
    }

    for flips in input_flips.values() {
        assert!(flips.windows(2).all(|w| w[1] - w[0] == flips[0]));
    }

    let presses = input_flips.values().map(|flips| flips[0]).product();

    Ok((low_count * high_count, presses))
}
