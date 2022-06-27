use super::Plugin;
use crate::{
    context::{ContextCanHandle, ContextHandle},
    selector::Modifier,
    utils::indent,
};

use std::fmt::{self, Write};

#[derive(Debug)]
pub struct AlignContentPlugin;

impl Plugin for AlignContentPlugin {
    fn namespace(&self) -> &str {
        "content"
    }

    fn can_handle(&self, context: ContextCanHandle) -> bool {
        match context.modifier {
            Modifier::Basic { value, .. } => {
                ["start", "center", "end", "between", "around", "evenly"].contains(value)
            }
            Modifier::Arbitrary { .. } => false,
        }
    }

    fn handle(&self, context: ContextHandle) -> fmt::Result {
        indent(context.indentation, context.buffer)?;
        match context.modifier {
            Modifier::Basic { value, .. } => match *value {
                "start" => writeln!(context.buffer, "align-content: flex-start;")?,
                "center" => writeln!(context.buffer, "align-content: center;")?,
                "end" => writeln!(context.buffer, "align-content: flex-end;")?,
                "between" => writeln!(context.buffer, "align-content: space-between;")?,
                "around" => writeln!(context.buffer, "align-content: space-around;")?,
                "evenly" => writeln!(context.buffer, "align-content: space-evenly;")?,
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

    fn can_handle(&self, context: ContextCanHandle) -> bool {
        match context.modifier {
            Modifier::Basic { value, .. } => {
                ["stretch", "start", "center", "end", "baseline"].contains(value)
            }
            Modifier::Arbitrary { .. } => false,
        }
    }

    fn handle(&self, context: ContextHandle) -> fmt::Result {
        indent(context.indentation, context.buffer)?;
        match context.modifier {
            Modifier::Basic { value, .. } => match *value {
                "stretch" => writeln!(context.buffer, "align-items: stretch;")?,
                "start" => writeln!(context.buffer, "align-items: flex-start;")?,
                "center" => writeln!(context.buffer, "align-items: center;")?,
                "end" => writeln!(context.buffer, "align-items: flex-end;")?,
                "baseline" => writeln!(context.buffer, "align-items: baseline;")?,
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

    fn can_handle(&self, context: ContextCanHandle) -> bool {
        match context.modifier {
            Modifier::Basic { value, .. } => {
                ["auto", "start", "center", "end", "stretch"].contains(value)
            }
            Modifier::Arbitrary { .. } => false,
        }
    }

    fn handle(&self, context: ContextHandle) -> fmt::Result {
        indent(context.indentation, context.buffer)?;
        match context.modifier {
            Modifier::Basic { value, .. } => match *value {
                "auto" => writeln!(context.buffer, "align-self: auto;")?,
                "start" => writeln!(context.buffer, "align-self: flex-start;")?,
                "center" => writeln!(context.buffer, "align-self: center;")?,
                "end" => writeln!(context.buffer, "align-self: flex-end;")?,
                "stretch" => writeln!(context.buffer, "align-self: stretch;")?,
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

    fn can_handle(&self, context: ContextCanHandle) -> bool {
        match context.modifier {
            Modifier::Basic { value, .. } => {
                ["start", "center", "end", "between", "around", "evenly"].contains(value)
            }
            Modifier::Arbitrary { .. } => false,
        }
    }

    fn handle(&self, context: ContextHandle) -> fmt::Result {
        indent(context.indentation, context.buffer)?;
        match context.modifier {
            Modifier::Basic { value, .. } => match *value {
                "start" => writeln!(context.buffer, "justify-content: flex-start;")?,
                "center" => writeln!(context.buffer, "justify-content: center;")?,
                "end" => writeln!(context.buffer, "justify-content: flex-end;")?,
                "between" => writeln!(context.buffer, "justify-content: space-between;")?,
                "around" => writeln!(context.buffer, "justify-content: space-around;")?,
                "evenly" => writeln!(context.buffer, "justify-content: space-evenly;")?,
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

    fn can_handle(&self, context: ContextCanHandle) -> bool {
        match context.modifier {
            Modifier::Basic { value, .. } => {
                ["stretch", "start", "center", "end", "auto"].contains(value)
            }
            Modifier::Arbitrary { .. } => false,
        }
    }

    fn handle(&self, context: ContextHandle) -> fmt::Result {
        indent(context.indentation, context.buffer)?;
        match context.modifier {
            Modifier::Basic { value, .. } => match *value {
                "stretch" => writeln!(context.buffer, "justify-items: stretch;")?,
                "start" => writeln!(context.buffer, "justify-items: start;")?,
                "center" => writeln!(context.buffer, "justify-items: center;")?,
                "end" => writeln!(context.buffer, "justify-items: end;")?,
                "auto" => writeln!(context.buffer, "justify-items: auto;")?,
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

    fn can_handle(&self, context: ContextCanHandle) -> bool {
        match context.modifier {
            Modifier::Basic { value, .. } => {
                ["stretch", "start", "center", "end", "auto"].contains(value)
            }
            Modifier::Arbitrary { .. } => false,
        }
    }

    fn handle(&self, context: ContextHandle) -> fmt::Result {
        indent(context.indentation, context.buffer)?;
        match context.modifier {
            Modifier::Basic { value, .. } => match *value {
                "stretch" => writeln!(context.buffer, "justify-self: stretch;")?,
                "start" => writeln!(context.buffer, "justify-self: start;")?,
                "center" => writeln!(context.buffer, "justify-self: center;")?,
                "end" => writeln!(context.buffer, "justify-self: end;")?,
                "auto" => writeln!(context.buffer, "justify-self: auto;")?,
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

    fn can_handle(&self, context: ContextCanHandle) -> bool {
        match context.modifier {
            Modifier::Basic { value, .. } => {
                ["start", "center", "end", "between", "around", "evenly"].contains(value)
            }
            Modifier::Arbitrary { .. } => false,
        }
    }

    fn handle(&self, context: ContextHandle) -> fmt::Result {
        indent(context.indentation, context.buffer)?;
        match context.modifier {
            Modifier::Basic { value, .. } => match *value {
                "start" => writeln!(context.buffer, "place-content: start;")?,
                "center" => writeln!(context.buffer, "place-content: center;")?,
                "end" => writeln!(context.buffer, "place-content: end;")?,
                "between" => writeln!(context.buffer, "place-content: space-between;")?,
                "around" => writeln!(context.buffer, "place-content: space-around;")?,
                "evenly" => writeln!(context.buffer, "place-content: space-evenly;")?,
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

    fn can_handle(&self, context: ContextCanHandle) -> bool {
        match context.modifier {
            Modifier::Basic { value, .. } => ["stretch", "start", "center", "end"].contains(value),
            Modifier::Arbitrary { .. } => false,
        }
    }

    fn handle(&self, context: ContextHandle) -> fmt::Result {
        indent(context.indentation, context.buffer)?;
        match context.modifier {
            Modifier::Basic { value, .. } => match *value {
                "stretch" => writeln!(context.buffer, "place-items: stretch;")?,
                "start" => writeln!(context.buffer, "place-items: start;")?,
                "center" => writeln!(context.buffer, "place-items: center;")?,
                "end" => writeln!(context.buffer, "place-items: end;")?,
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

    fn can_handle(&self, context: ContextCanHandle) -> bool {
        match context.modifier {
            Modifier::Basic { value, .. } => {
                ["auto", "start", "center", "end", "stretch"].contains(value)
            }
            Modifier::Arbitrary { .. } => false,
        }
    }

    fn handle(&self, context: ContextHandle) -> fmt::Result {
        indent(context.indentation, context.buffer)?;
        match context.modifier {
            Modifier::Basic { value, .. } => match *value {
                "auto" => writeln!(context.buffer, "place-self: auto;")?,
                "start" => writeln!(context.buffer, "place-self: start;")?,
                "center" => writeln!(context.buffer, "place-self: center;")?,
                "end" => writeln!(context.buffer, "place-self: end;")?,
                "stretch" => writeln!(context.buffer, "place-self: stretch;")?,
                _ => unreachable!(),
            },
            Modifier::Arbitrary { .. } => unreachable!(),
        }

        Ok(())
    }
}
