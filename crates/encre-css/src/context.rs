use super::{
    config::Config,
    selector::{Modifier, Selector},
    variant::Variant,
};

use std::{borrow::Cow, collections::BTreeMap};

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

pub struct ContextBeforeRule<'a, 'b, 'c, 'd> {
    pub config: &'a Config,
    pub selector: &'b Selector<'c>,
    pub buffer: &'d mut String,
}

pub struct ContextAfterRule<'a, 'b, 'c, 'd, 'e, 'f> {
    pub config: &'a Config,
    pub selector: &'b Selector<'c>,
    pub buffer: &'d mut String,
    pub custom_variants: &'e BTreeMap<Cow<'f, str>, Variant>,
}
