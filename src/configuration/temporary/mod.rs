use crate::configuration::temporary::rules::TemporaryRule;

pub mod actions;
pub mod conflicts;
pub mod folders;
pub mod options;
pub mod rules;

pub trait TemporaryConfigElement<T> {
    fn unwrap(self) -> T;
    fn fill(&mut self, parent_rule: &TemporaryRule) -> Self;
}
