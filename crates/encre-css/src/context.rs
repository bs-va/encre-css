use super::{config::Config, selector::Modifier};

pub struct ContextCanHandle<'a, 'b, 'c> {
    pub config: &'a Config,
    pub modifier: &'b Modifier<'c>,
}

pub struct ContextHandle<'a, 'b, 'c, 'd> {
    pub config: &'a Config,
    pub modifier: &'b Modifier<'c>,
    pub indentation: usize,
    pub buffer: &'d mut String,
}
