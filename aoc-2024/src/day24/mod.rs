use std::{
    collections::HashMap,
    ops::{BitAnd, BitOr, BitXor},
};

use aoc_common::util;

#[derive(Clone, Copy, PartialEq, Eq, Debug)]
enum WireState {
    Off,
    On,
}

impl TryFrom<u8> for WireState {
    type Error = String;

    fn try_from(value: u8) -> Result<Self, Self::Error> {
        match value {
            0 => Ok(Self::Off),
            1 => Ok(Self::On),
            _ => Err(format!("Invalid state {value}")),
        }
    }
}

impl BitAnd<Self> for WireState {
    type Output = Self;

    fn bitand(self, rhs: Self) -> Self::Output {
        match (self, rhs) {
            (Self::Off, _) | (_, Self::Off) => Self::Off,
            _ => Self::On,
        }
    }
}

impl BitOr<Self> for WireState {
    type Output = Self;

    fn bitor(self, rhs: Self) -> Self::Output {
        match (self, rhs) {
            (Self::On, _) | (_, Self::On) => Self::On,
            _ => Self::Off,
        }
    }
}

impl BitXor<Self> for WireState {
    type Output = Self;

    fn bitxor(self, rhs: Self) -> Self::Output {
        if self == rhs {
            Self::Off
        } else {
            Self::On
        }
    }
}

#[derive(Clone, Copy, PartialEq, Eq, Debug)]
enum GateKind {
    And,
    Or,
    Xor,
}

impl TryFrom<&str> for GateKind {
    type Error = String;

    fn try_from(value: &str) -> Result<Self, Self::Error> {
        match value {
            "AND" => Ok(Self::And),
            "OR" => Ok(Self::Or),
            "XOR" => Ok(Self::Xor),
            s => Err(format!("Invalid gate kind: {s}")),
        }
    }
}

fn process_gate(gate: GateKind, inputs: (WireState, WireState)) -> WireState {
    match gate {
        GateKind::And => inputs.0 & inputs.1,
        GateKind::Or => inputs.0 | inputs.1,
        GateKind::Xor => inputs.0 ^ inputs.1,
    }
}

#[derive(Clone, Debug)]
struct Gate {
    inputs: (usize, usize),
    output: usize,
    kind: GateKind,
}

struct Device {
    wires: Vec<String>,
    wire_states: Vec<WireState>,
    wire_name_to_id: HashMap<String, usize>,
    gates: HashMap<usize, Gate>,
}

impl Device {
    fn new() -> Self {
        Self {
            wires: vec![],
            wire_states: vec![],
            wire_name_to_id: HashMap::new(),
            gates: HashMap::new(),
        }
    }

    fn get_or_insert(&mut self, name: &str) -> usize {
        if let Some(&id) = self.wire_name_to_id.get(name) {
            return id;
        }

        let id = self.wires.len();
        self.wires.push(name.to_string());
        self.wire_states.push(WireState::Off);
        self.wire_name_to_id.insert(name.to_string(), id);

        id
    }

    fn add_input(&mut self, name: &str, state: WireState) {
        let id = self.get_or_insert(name);
        self.wire_states[id] = state;
    }

    fn add_gate(&mut self, input1: &str, input2: &str, output: &str, kind: GateKind) {
        let input1 = self.get_or_insert(input1);
        let input2 = self.get_or_insert(input2);
        let output = self.get_or_insert(output);

        self.gates.insert(
            output,
            Gate {
                inputs: (input1, input2),
                output,
                kind,
            },
        );
    }

    fn do_get_wires_topological(
        &self,
        current: usize,
        visited: &mut [bool],
        wires: &mut Vec<usize>,
    ) {
        if let Some(gate) = self.gates.get(&current) {
            let (input1, input2) = gate.inputs;
            if !visited[input1] {
                self.do_get_wires_topological(input1, visited, wires);
            }
            if !visited[input2] {
                self.do_get_wires_topological(input2, visited, wires);
            }
        }
        visited[current] = true;
        wires.push(current);
    }

    fn get_wires_topological(&self) -> Vec<usize> {
        let mut wires = vec![];
        let mut visited: Vec<bool> = vec![false; self.wires.len()];

        for wire in 0..self.wires.len() {
            if !visited[wire] {
                self.do_get_wires_topological(wire, &mut visited, &mut wires);
            }
        }

        wires
    }

    fn send_signals(&mut self) {
        let wires = self.get_wires_topological();
        for wire in wires {
            let Some(gate) = self.gates.get(&wire) else {
                continue;
            };
            self.wire_states[gate.output] = process_gate(
                gate.kind,
                (
                    self.wire_states[gate.inputs.0],
                    self.wire_states[gate.inputs.1],
                ),
            );
        }
    }
}

fn gate_matches(device: &Device, inputs: (&str, &str), wire_id: usize, kind: GateKind) -> bool {
    let Some(&input1_id) = device.wire_name_to_id.get(inputs.0) else {
        return false;
    };
    let Some(&input2_id) = device.wire_name_to_id.get(inputs.1) else {
        return false;
    };

    let Some(gate) = device.gates.get(&wire_id) else {
        return false;
    };

    gate.kind == kind
        && (gate.inputs == (input1_id, input2_id) || gate.inputs == (input2_id, input1_id))
}

fn is_bit_carry(device: &Device, wire_id: usize, bit: usize) -> bool {
    gate_matches(
        device,
        (&format!("x{bit:02}"), &format!("y{bit:02}")),
        wire_id,
        GateKind::And,
    )
}

fn is_bit_sum(device: &Device, wire_id: usize, bit: usize) -> bool {
    gate_matches(
        device,
        (&format!("x{bit:02}"), &format!("y{bit:02}")),
        wire_id,
        GateKind::Xor,
    )
}

#[allow(unused)]
fn verify_addition(device: &Device, bits: usize) {
    /*
       For each bit from 1 to (bits - 2), verify that bit result and carry are
       calculated via following gates:

       Result:

       x{bit} XOR y{bit} -> BIT_SUM{bit}
       BIT_SUM{bit} XOR CARRY{bit-1} -> z{bit}

       Carry:

       x{bit} AND y{bit} -> BIT_CARRY{bit}
       BIT_SUM{bit} AND CARRY{bit - 1} -> PREV_SUM_CARRY{bit}
       BIT_CARRY{bit} OR PREV_SUM_CARRY{bit} -> CARRY{bit}
    */

    let top_carry = format!("z{}", bits - 1);

    let mut carry = device.wire_name_to_id[&top_carry];
    for bit in (1..(bits - 1)).rev() {
        let carry_name = &device.wires[carry];
        let carry_gate = device
            .gates
            .get(&carry)
            .unwrap_or_else(|| panic!("Carry #{bit} ({carry_name}) is not computable!"));

        assert!(
            carry_gate.kind == GateKind::Or,
            "Carry #{bit} ({carry_name}) gate kind is {:?}, while Or is expected",
            carry_gate.kind
        );

        let prev_carry_sum = if is_bit_carry(device, carry_gate.inputs.0, bit) {
            carry_gate.inputs.1
        } else if is_bit_carry(device, carry_gate.inputs.1, bit) {
            carry_gate.inputs.0
        } else {
            panic!("Carry #{bit} ({carry_name}): one of operands should be a result of (x{bit:02} And y{bit:02})")
        };

        let prev_sum_name = &device.wires[prev_carry_sum];
        let prev_sum_gate = device.gates.get(&prev_carry_sum).unwrap_or_else(|| {
            panic!("Prev carry sum #{bit} ({prev_sum_name}) is not computable!")
        });

        assert!(
            prev_sum_gate.kind == GateKind::And,
            "Prev carry sum #{bit} ({prev_sum_name}) gate kind is {:?}, while And is expected",
            carry_gate.kind
        );

        let result_id = device.wire_name_to_id.get(&format!("z{bit:02}")).unwrap();
        assert!(
            gate_matches(device, (&device.wires[prev_sum_gate.inputs.0], &device.wires[prev_sum_gate.inputs.1]), *result_id, GateKind::Xor),
            "Prev carry sum #{bit} ({prev_sum_name}): its inputs are not Xor'ed to make a result z{bit:02}"
        );

        let prev_carry = if is_bit_sum(device, prev_sum_gate.inputs.0, bit) {
            prev_sum_gate.inputs.1
        } else if is_bit_sum(device, prev_sum_gate.inputs.1, bit) {
            prev_sum_gate.inputs.0
        } else {
            panic!("Prev carry sum #{bit} ({prev_sum_name}): one of operands should be a result of (x{bit:02} Xor y{bit:02})")
        };

        carry = prev_carry;
    }

    // Add some trivial checks for bit 0.
    assert!(is_bit_carry(device, carry, 0));
    assert!(is_bit_sum(
        device,
        *device.wire_name_to_id.get("z00").unwrap(),
        0
    ));
}

pub fn get_answer(mut lines: impl Iterator<Item = String>) -> util::GenericResult<(usize, String)> {
    let mut device = Device::new();

    for line in lines.by_ref().take_while(|s| !s.is_empty()) {
        let mut lexer = util::Lexer::of(&line);
        let name = lexer.before_literal(": ")?;
        let state = lexer.unsigned_number::<u8>()?;
        lexer.end()?;

        device.add_input(name, state.try_into()?);
    }

    for line in lines {
        let mut lexer = util::Lexer::of(&line);
        let input1 = lexer.before_literal(" ")?;
        let gate = lexer.before_literal(" ")?;
        let input2 = lexer.before_literal(" -> ")?;
        let output = lexer.take_rest()?;

        device.add_gate(input1, input2, output, gate.try_into()?);
    }

    device.send_signals();

    let mut z_names = device
        .wires
        .iter()
        .filter(|name| name.starts_with('z'))
        .collect::<Vec<_>>();

    z_names.sort_unstable();
    z_names.reverse();

    // let bits = z_names.len();

    let mut output = 0;
    for name in z_names {
        let id = device.wire_name_to_id.get(name).unwrap();
        let bit = usize::from(device.wire_states[*id] == WireState::On);

        output = output * 2 + bit;
    }

    // Calculated by uncommenting next line and continuously solving errors produced by it.
    // verify_addition(&device, bits);
    let mut bad_outputs = vec![];

    // Errors and solutions log:

    // Carry #39 (tnc) gate kind is Xor, while Or is expected
    // - Found a gate to satisfy is_bit_carry : y39 AND x39 -> rvd
    // - "rvd" is an input of "z39", so swapped "tnc" and "z39"
    bad_outputs.extend(["tnc", "z39"]);

    // Carry #35 (bwc): one of operands should be a result of (x35 And y35)
    // - "bwc" gate: ftc OR fsq -> bwc
    // - The gate which produces such result: y35 AND x35 -> dvb
    // - "ftc" consumes "dvb" to produce prev carry sum, so "dvb" and "fsq" should be swapped
    bad_outputs.extend(["dvb", "fsq"]);

    // Prev carry sum #17 (fhg) gate kind is Or, while And is expected
    // - Found a gate to satisfy is_bit_sum : x17 XOR y17 -> qjg
    // - "qjg" is an input of "z17", so swapped "fhg" and "z17"
    bad_outputs.extend(["fhg", "z17"]);

    // Carry #10 (fgb): one of operands should be a result of (x10 And y10)
    // - "fgb" gate : sst OR vcf -> fgb
    // - The gate which produces such result: x10 AND y10 -> z10
    // - "sst" gate : skm AND kck -> sst, is a correct prev carry sum
    // - Swapping "vcf" and "z10"
    bad_outputs.extend(["vcf", "z10"]);

    bad_outputs.sort_unstable();
    let bad_outputs_string = bad_outputs.join(",");

    Ok((output, bad_outputs_string))
}
