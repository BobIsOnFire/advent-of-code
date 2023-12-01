use std::collections::HashSet;

use crate::util;

#[derive(Clone, Copy, PartialEq, Eq, Hash)]
enum ResourceType {
    Ore,
    Clay,
    Obsidian,
    Geode,
}

#[derive(Default, Clone, Debug, PartialEq, Eq, Hash)]
struct ResourceMapping<T> {
    data: [T; 4],
}

#[derive(Clone, Debug, PartialEq, Eq, Hash)]
struct ResourceCount(ResourceMapping<u16>);
#[derive(Clone, Debug, PartialEq, Eq, Hash)]
struct WorkerCount(ResourceMapping<u16>);

#[derive(Clone, Debug, PartialEq, Eq, Hash)]
struct ResourceState {
    count: ResourceCount,
    workers: WorkerCount,
}

#[derive(Default)]
struct WorkerCost(ResourceMapping<u16>);
#[derive(Default)]
struct Blueprint(ResourceMapping<WorkerCost>);

impl ResourceType {
    fn all_types() -> [Self; 4] {
        [Self::Ore, Self::Clay, Self::Obsidian, Self::Geode]
    }
}

impl<T> ResourceMapping<T> {
    fn get(&self, res_type: ResourceType) -> &T {
        &self.data[res_type as usize]
    }

    fn get_mut(&mut self, res_type: ResourceType) -> &mut T {
        &mut self.data[res_type as usize]
    }
}

impl ResourceCount {
    fn new() -> Self {
        Self(ResourceMapping::default())
    }
}

impl WorkerCount {
    fn new() -> Self {
        let mut mapping = ResourceMapping::default();
        *mapping.get_mut(ResourceType::Ore) = 1;
        Self(mapping)
    }
}

impl ResourceState {
    fn new() -> Self {
        Self {
            count: ResourceCount::new(),
            workers: WorkerCount::new(),
        }
    }

    fn find_previous_state(&self) -> Option<Self> {
        let mut new_state = self.clone();
        for res_type in ResourceType::all_types() {
            let previous_count =
                u16::checked_sub(*self.count.0.get(res_type), *self.workers.0.get(res_type))?;

            *new_state.count.0.get_mut(res_type) = previous_count;
        }

        Some(new_state)
    }

    fn generate_resources(&self) -> Self {
        let mut new_state = self.clone();
        for res_type in ResourceType::all_types() {
            *new_state.count.0.get_mut(res_type) += new_state.workers.0.get(res_type);
        }
        new_state
    }

    fn build_worker(&self, worker_type: ResourceType, worker_cost: &WorkerCost) -> Option<Self> {
        let can_afford = ResourceType::all_types()
            .into_iter()
            .all(|res| worker_cost.0.get(res) <= self.count.0.get(res));

        if can_afford {
            let mut new_state = self.generate_resources();
            for res in ResourceType::all_types() {
                *new_state.count.0.get_mut(res) -= worker_cost.0.get(res);
            }
            *new_state.workers.0.get_mut(worker_type) += 1;

            Some(new_state)
        } else {
            None
        }
    }
}

impl Blueprint {
    fn generate_states(&self, state: &ResourceState) -> Vec<ResourceState> {
        let prev_state = state.find_previous_state();

        let mut states = vec![state.generate_resources()];

        for worker_type in ResourceType::all_types() {
            let worker_cost = self.0.get(worker_type);

            // If we could have built this type of worker on previous state, we already have a state where we built it
            if prev_state
                .as_ref()
                .and_then(|s| s.build_worker(worker_type, worker_cost))
                .is_some()
            {
                continue;
            }

            if let Some(s) = state.build_worker(worker_type, worker_cost) {
                states.push(s);
            }
        }

        states
    }
}

fn parse_resource_type(lexer: &mut util::Lexer) -> util::GenericResult<ResourceType> {
    let data = lexer.take_while(|ch| ch.is_ascii_alphabetic())?;
    match data {
        "ore" => Ok(ResourceType::Ore),
        "clay" => Ok(ResourceType::Clay),
        "obsidian" => Ok(ResourceType::Obsidian),
        "geode" => Ok(ResourceType::Geode),
        _ => Err(format!("Unknown resource type: {}", data).into()),
    }
}

fn parse_worker_cost(lexer: &mut util::Lexer) -> util::GenericResult<(WorkerCost, ResourceType)> {
    lexer.literal("Each ")?;
    let worker_type = parse_resource_type(lexer)?;
    lexer.literal(" robot costs ")?;

    let mut worker_cost = WorkerCost::default();
    loop {
        let count = lexer.unsigned_number()?;
        lexer.literal(" ")?;
        let res_type = parse_resource_type(lexer)?;
        *worker_cost.0.get_mut(res_type) = count;

        if lexer.literal(".").is_ok() {
            break;
        }
        lexer.literal(" and ")?;
    }

    Ok((worker_cost, worker_type))
}

fn parse_blueprint(data: &str) -> util::GenericResult<Blueprint> {
    let mut lexer = util::Lexer::of(data);
    lexer.literal("Blueprint ")?;
    let _: usize = lexer.unsigned_number()?;
    lexer.literal(": ")?;

    let mut blueprint = Blueprint::default();
    loop {
        let (worker_cost, worker_type) = parse_worker_cost(&mut lexer)?;
        *blueprint.0.get_mut(worker_type) = worker_cost;

        if lexer.end().is_ok() {
            break;
        }
        lexer.literal(" ")?;
    }

    Ok(blueprint)
}

fn find_max_geodes(blueprint: &Blueprint, minutes: usize) -> u16 {
    let mut states = HashSet::from([ResourceState::new()]);
    for i in 1..minutes {
        states = states
            .iter()
            .flat_map(|s| blueprint.generate_states(s))
            .collect();

        println!("After {} minutes: len = {}", i, states.len());
    }

    states
        .iter()
        .map(|s| *s.generate_resources().count.0.get(ResourceType::Geode))
        .max()
        .unwrap_or(0)
}

pub fn get_answer(lines: impl Iterator<Item = String>) -> util::GenericResult<(usize, usize)> {
    let mut quality = 0;
    let mut first_three = 1;

    for (num, line) in lines.enumerate() {
        let blueprint = parse_blueprint(&line)?;

        let regular = find_max_geodes(&blueprint, 24) as usize;
        println!("Blueprint {}: {}", num + 1, regular);
        quality += (num + 1) * regular;

        if num < 3 {
            let thirty_two = find_max_geodes(&blueprint, 32) as usize;
            println!("Blueprint {}: {}", num + 1, thirty_two);
            first_three *= thirty_two;
        }
    }
    Ok((quality, first_three))
}
