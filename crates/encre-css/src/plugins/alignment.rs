use super::Plugin;
use crate::{config::Config, selector::Modifier, utils::indent};

use std::fmt::{self, Write};

#[derive(Debug)]
pub struct AlignContentPlugin;

impl Plugin for AlignContentPlugin {
    fn namespace(&self) -> &str {
        "content"
    }

    fn can_handle(&self, _config: &Config, modifier: &Modifier) -> bool {
        match modifier {
            Modifier::Basic { value, .. } => {
                ["start", "center", "end", "between", "around", "evenly"].contains(value)
            }
            Modifier::Arbitrary { .. } => false,
        }
    }

    fn handle(
        &self,
        _config: &Config,
        modifier: &Modifier,
        indentation: usize,
        buffer: &mut String,
    ) -> fmt::Result {
        indent(indentation, buffer)?;
        match modifier {
            Modifier::Basic { value, .. } => match *value {
                "start" => writeln!(buffer, "align-content: flex-start;")?,
                "center" => writeln!(buffer, "align-content: center;")?,
                "end" => writeln!(buffer, "align-content: flex-end;")?,
                "between" => writeln!(buffer, "align-content: space-between;")?,
                "around" => writeln!(buffer, "align-content: space-around;")?,
                "evenly" => writeln!(buffer, "align-content: space-evenly;")?,
                _ => unreachable!(),
            },
            Modifier::Arbitrary { .. } => unreachable!(),
        }

        Ok(())
    }
}

#[derive(Debug)]
pub struct AlignItemsPlugin;

impl Plugin for AlignItemsPlugin {
    fn namespace(&self) -> &str {
        "items"
    }

    fn can_handle(&self, _config: &Config, modifier: &Modifier) -> bool {
        match modifier {
            Modifier::Basic { value, .. } => {
                ["stretch", "start", "center", "end", "baseline"].contains(value)
            }
            Modifier::Arbitrary { .. } => false,
        }
    }

    fn handle(
        &self,
        _config: &Config,
        modifier: &Modifier,
        indentation: usize,
        buffer: &mut String,
    ) -> fmt::Result {
        indent(indentation, buffer)?;
        match modifier {
            Modifier::Basic { value, .. } => match *value {
                "stretch" => writeln!(buffer, "align-items: stretch;")?,
                "start" => writeln!(buffer, "align-items: flex-start;")?,
                "center" => writeln!(buffer, "align-items: center;")?,
                "end" => writeln!(buffer, "align-items: flex-end;")?,
                "baseline" => writeln!(buffer, "align-items: baseline;")?,
                _ => unreachable!(),
            },
            Modifier::Arbitrary { .. } => unreachable!(),
        }

        Ok(())
    }
}

#[derive(Debug)]
pub struct AlignSelfPlugin;

impl Plugin for AlignSelfPlugin {
    fn namespace(&self) -> &str {
        "self"
    }

    fn can_handle(&self, _config: &Config, modifier: &Modifier) -> bool {
        match modifier {
            Modifier::Basic { value, .. } => {
                ["auto", "start", "center", "end", "stretch"].contains(value)
            }
            Modifier::Arbitrary { .. } => false,
        }
    }

    fn handle(
        &self,
        _config: &Config,
        modifier: &Modifier,
        indentation: usize,
        buffer: &mut String,
    ) -> fmt::Result {
        indent(indentation, buffer)?;
        match modifier {
            Modifier::Basic { value, .. } => match *value {
                "auto" => writeln!(buffer, "align-self: auto;")?,
                "start" => writeln!(buffer, "align-self: flex-start;")?,
                "center" => writeln!(buffer, "align-self: center;")?,
                "end" => writeln!(buffer, "align-self: flex-end;")?,
                "stretch" => writeln!(buffer, "align-self: stretch;")?,
                _ => unreachable!(),
            },
            Modifier::Arbitrary { .. } => unreachable!(),
        }

        Ok(())
    }
}

#[derive(Debug)]
pub struct JustifyContentPlugin;

impl Plugin for JustifyContentPlugin {
    fn namespace(&self) -> &str {
        "justify"
    }

    fn can_handle(&self, _config: &Config, modifier: &Modifier) -> bool {
        match modifier {
            Modifier::Basic { value, .. } => {
                ["start", "center", "end", "between", "around", "evenly"].contains(value)
            }
            Modifier::Arbitrary { .. } => false,
        }
    }

    fn handle(
        &self,
        _config: &Config,
        modifier: &Modifier,
        indentation: usize,
        buffer: &mut String,
    ) -> fmt::Result {
        indent(indentation, buffer)?;
        match modifier {
            Modifier::Basic { value, .. } => match *value {
                "start" => writeln!(buffer, "justify-content: flex-start;")?,
                "center" => writeln!(buffer, "justify-content: center;")?,
                "end" => writeln!(buffer, "justify-content: flex-end;")?,
                "between" => writeln!(buffer, "justify-content: space-between;")?,
                "around" => writeln!(buffer, "justify-content: space-around;")?,
                "evenly" => writeln!(buffer, "justify-content: space-evenly;")?,
                _ => unreachable!(),
            },
            Modifier::Arbitrary { .. } => unreachable!(),
        }

        Ok(())
    }
}

#[derive(Debug)]
pub struct JustifyItemsPlugin;

impl Plugin for JustifyItemsPlugin {
    fn namespace(&self) -> &str {
        "justify-items"
    }

    fn can_handle(&self, _config: &Config, modifier: &Modifier) -> bool {
        match modifier {
            Modifier::Basic { value, .. } => {
                ["stretch", "start", "center", "end", "auto"].contains(value)
            }
            Modifier::Arbitrary { .. } => false,
        }
    }

    fn handle(
        &self,
        _config: &Config,
        modifier: &Modifier,
        indentation: usize,
        buffer: &mut String,
    ) -> fmt::Result {
        indent(indentation, buffer)?;
        match modifier {
            Modifier::Basic { value, .. } => match *value {
                "stretch" => writeln!(buffer, "justify-items: stretch;")?,
                "start" => writeln!(buffer, "justify-items: start;")?,
                "center" => writeln!(buffer, "justify-items: center;")?,
                "end" => writeln!(buffer, "justify-items: end;")?,
                "auto" => writeln!(buffer, "justify-items: auto;")?,
                _ => unreachable!(),
            },
            Modifier::Arbitrary { .. } => unreachable!(),
        }

        Ok(())
    }
}

#[derive(Debug)]
pub struct JustifySelfPlugin;

impl Plugin for JustifySelfPlugin {
    fn namespace(&self) -> &str {
        "justify-self"
    }

    fn can_handle(&self, _config: &Config, modifier: &Modifier) -> bool {
        match modifier {
            Modifier::Basic { value, .. } => {
                ["stretch", "start", "center", "end", "auto"].contains(value)
            }
            Modifier::Arbitrary { .. } => false,
        }
    }

    fn handle(
        &self,
        _config: &Config,
        modifier: &Modifier,
        indentation: usize,
        buffer: &mut String,
    ) -> fmt::Result {
        indent(indentation, buffer)?;
        match modifier {
            Modifier::Basic { value, .. } => match *value {
                "stretch" => writeln!(buffer, "justify-self: stretch;")?,
                "start" => writeln!(buffer, "justify-self: start;")?,
                "center" => writeln!(buffer, "justify-self: center;")?,
                "end" => writeln!(buffer, "justify-self: end;")?,
                "auto" => writeln!(buffer, "justify-self: auto;")?,
                _ => unreachable!(),
            },
            Modifier::Arbitrary { .. } => unreachable!(),
        }

        Ok(())
    }
}

#[derive(Debug)]
pub struct PlaceContentPlugin;

impl Plugin for PlaceContentPlugin {
    fn namespace(&self) -> &str {
        "place-content"
    }

    fn can_handle(&self, _config: &Config, modifier: &Modifier) -> bool {
        match modifier {
            Modifier::Basic { value, .. } => {
                ["start", "center", "end", "between", "around", "evenly"].contains(value)
            }
            Modifier::Arbitrary { .. } => false,
        }
    }

    fn handle(
        &self,
        _config: &Config,
        modifier: &Modifier,
        indentation: usize,
        buffer: &mut String,
    ) -> fmt::Result {
        indent(indentation, buffer)?;
        match modifier {
            Modifier::Basic { value, .. } => match *value {
                "start" => writeln!(buffer, "place-content: start;")?,
                "center" => writeln!(buffer, "place-content: center;")?,
                "end" => writeln!(buffer, "place-content: end;")?,
                "between" => writeln!(buffer, "place-content: space-between;")?,
                "around" => writeln!(buffer, "place-content: space-around;")?,
                "evenly" => writeln!(buffer, "place-content: space-evenly;")?,
                _ => unreachable!(),
            },
            Modifier::Arbitrary { .. } => unreachable!(),
        }

        Ok(())
    }
}

#[derive(Debug)]
pub struct PlaceItemsPlugin;

impl Plugin for PlaceItemsPlugin {
    fn namespace(&self) -> &str {
        "place-items"
    }

    fn can_handle(&self, _config: &Config, modifier: &Modifier) -> bool {
        match modifier {
            Modifier::Basic { value, .. } => ["stretch", "start", "center", "end"].contains(value),
            Modifier::Arbitrary { .. } => false,
        }
    }

    fn handle(
        &self,
        _config: &Config,
        modifier: &Modifier,
        indentation: usize,
        buffer: &mut String,
    ) -> fmt::Result {
        indent(indentation, buffer)?;
        match modifier {
            Modifier::Basic { value, .. } => match *value {
                "stretch" => writeln!(buffer, "place-items: stretch;")?,
                "start" => writeln!(buffer, "place-items: start;")?,
                "center" => writeln!(buffer, "place-items: center;")?,
                "end" => writeln!(buffer, "place-items: end;")?,
                _ => unreachable!(),
            },
            Modifier::Arbitrary { .. } => unreachable!(),
        }

        Ok(())
    }
}

#[derive(Debug)]
pub struct PlaceSelfPlugin;

impl Plugin for PlaceSelfPlugin {
    fn namespace(&self) -> &str {
        "place-self"
    }

    fn can_handle(&self, _config: &Config, modifier: &Modifier) -> bool {
        match modifier {
            Modifier::Basic { value, .. } => {
                ["auto", "start", "center", "end", "stretch"].contains(value)
            }
            Modifier::Arbitrary { .. } => false,
        }
    }

    fn handle(
        &self,
        _config: &Config,
        modifier: &Modifier,
        indentation: usize,
        buffer: &mut String,
    ) -> fmt::Result {
        indent(indentation, buffer)?;
        match modifier {
            Modifier::Basic { value, .. } => match *value {
                "auto" => writeln!(buffer, "place-self: auto;")?,
                "start" => writeln!(buffer, "place-self: start;")?,
                "center" => writeln!(buffer, "place-self: center;")?,
                "end" => writeln!(buffer, "place-self: end;")?,
                "stretch" => writeln!(buffer, "place-self: stretch;")?,
                _ => unreachable!(),
            },
            Modifier::Arbitrary { .. } => unreachable!(),
        }

        Ok(())
    }
}
