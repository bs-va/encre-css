use super::Plugin;
use crate::{config::Config, selector::Modifier, utils::indent};

use std::fmt::{self, Write};

pub struct ScreenReaderPlugin;

impl Plugin for ScreenReaderPlugin {
    fn can_handle(&self, _config: &Config, modifier: &Modifier) -> bool {
        match modifier {
            Modifier::Basic { value, .. } => value == "sr-only" || value == "not-sr-only",
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
        match modifier {
            Modifier::Basic { value, .. } => match value.as_str() {
                "sr-only" => {
                    indent(indentation, buffer)?;
                    writeln!(buffer, "position: absolute;")?;
                    indent(indentation, buffer)?;
                    writeln!(buffer, "width: 1px;")?;
                    indent(indentation, buffer)?;
                    writeln!(buffer, "height: 1px;")?;
                    indent(indentation, buffer)?;
                    writeln!(buffer, "padding: 0;")?;
                    indent(indentation, buffer)?;
                    writeln!(buffer, "margin: -1px;")?;
                    indent(indentation, buffer)?;
                    writeln!(buffer, "overflow: hidden")?;
                    indent(indentation, buffer)?;
                    writeln!(buffer, "clip: rect(0, 0, 0, 0)")?;
                    indent(indentation, buffer)?;
                    writeln!(buffer, "white-space: nowrap;")?;
                    indent(indentation, buffer)?;
                    writeln!(buffer, "border-width: 0;")?;
                }
                "not-sr-only" => {
                    indent(indentation, buffer)?;
                    writeln!(buffer, "position: static;")?;
                    indent(indentation, buffer)?;
                    writeln!(buffer, "width: auto;")?;
                    indent(indentation, buffer)?;
                    writeln!(buffer, "height: auto;")?;
                    indent(indentation, buffer)?;
                    writeln!(buffer, "padding: 0;")?;
                    indent(indentation, buffer)?;
                    writeln!(buffer, "margin: 0;")?;
                    indent(indentation, buffer)?;
                    writeln!(buffer, "overflow: visible;")?;
                    indent(indentation, buffer)?;
                    writeln!(buffer, "clip: auto;")?;
                    indent(indentation, buffer)?;
                    writeln!(buffer, "white-space: normal;")?;
                }
                _ => unreachable!(),
            },
            Modifier::Arbitrary { .. } => unreachable!(),
        }

        Ok(())
    }
}
