pub trait Action {
    type Event: ActionEvent;
    fn generate_event(&self, item: u64) -> Self::Event;
}

impl<T, E> Action for T
where
    E: ActionEvent,
    T: Fn(u64) -> E,
{
    type Event = E;
    fn generate_event(&self, item: u64) -> Self::Event {
        self(item)
    }
}

pub trait ActionEvent {
    fn perform(self, ctx: &mut super::Context);

    fn boxed<'a>(self) -> Box<dyn 'a + ActionEvent>
    where
        Self: 'a + Sized,
    {
        Box::new(self)
    }
}

pub trait Predicate {
    fn test(&self, item: u64) -> bool;
}

impl<T> Predicate for T
where
    T: Fn(u64) -> bool,
{
    fn test(&self, item: u64) -> bool {
        self(item)
    }
}

pub struct ConditionalAction<P, A1, A2>
where
    P: Predicate,
    A1: Action,
    A2: Action,
{
    predicate: P,
    action_if_true: A1,
    action_if_false: A2,
}

impl<P, A1, A2> Action for ConditionalAction<P, A1, A2>
where
    P: Predicate,
    A1: Action,
    A2: Action,
{
    type Event = EitherEvent<A1::Event, A2::Event>;

    fn generate_event(&self, item: u64) -> Self::Event {
        if self.predicate.test(item) {
            Self::Event::First(self.action_if_true.generate_event(item))
        } else {
            Self::Event::Second(self.action_if_false.generate_event(item))
        }
    }
}

pub struct ThrowEvent {
    destination: usize,
    item: u64,
}

impl ActionEvent for ThrowEvent {
    fn perform(self, ctx: &mut super::Context) {
        ctx.get_entity_mut(self.destination)
            .expect("Destination should exist")
            .add_item(self.item);
    }
}

pub enum EitherEvent<E1, E2>
where
    E1: ActionEvent,
    E2: ActionEvent,
{
    First(E1),
    Second(E2),
}

impl<E1, E2> ActionEvent for EitherEvent<E1, E2>
where
    E1: ActionEvent,
    E2: ActionEvent,
{
    fn perform(self, ctx: &mut super::Context) {
        match self {
            Self::First(e) => e.perform(ctx),
            Self::Second(e) => e.perform(ctx),
        }
    }
}
