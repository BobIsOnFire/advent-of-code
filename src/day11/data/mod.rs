mod actions;
mod processors;

use actions::{Action, ActionEvent};
use processors::Processor;

pub trait Entity {
    fn add_item(&mut self, item: u64);
    fn total_events_generated(&self) -> usize;
    fn events<'a>(&'a mut self) -> Vec<Box<dyn 'a + ActionEvent>>;
}

pub struct Context {
    entities: Vec<Box<dyn Entity>>,
}

impl Context {
    pub fn get_entity_mut(&mut self, idx: usize) -> Option<&mut dyn Entity> {
        match self.entities.get_mut(idx) {
            None => None,
            Some(e) => Some(e.as_mut()),
        }
    }

    pub fn add_entity<E: 'static + Entity>(&mut self, entity: E) {
        self.entities.push(Box::new(entity));
    }
}

pub struct Monkey {
    items: Vec<u64>,
    total_events: usize,
    item_processor: Box<dyn Processor>,
    action: Box<dyn Action>,
}

impl Entity for Monkey {
    fn add_item(&mut self, item: u64) {
        self.items.push(item);
    }

    fn total_events_generated(&self) -> usize {
        self.total_events
    }

    fn events<'a>(&'a mut self) -> Vec<Box<dyn 'a + ActionEvent>> {
        let mut items = vec![];

        std::mem::swap(&mut self.items, &mut items);
        self.total_events += items.len();

        items
            .into_iter()
            .map(|item| self.item_processor.process(item))
            .map(|item| item / 3)
            .map(|item| self.action.generate_event(item).boxed())
            .collect()
    }
}
