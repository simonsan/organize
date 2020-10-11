use crate::configuration::temporary::rules::TemporaryRule;

pub mod filters;
pub mod folders;
pub mod options;
pub mod rules;

pub trait TemporaryConfigElement<T> {
    fn unwrap(self) -> T;
    fn fill(self, parent_rule: &TemporaryRule) -> Self;
}
