use super::Plugin;

use std::fmt::{Result, Write};

#[derive(Debug)]
pub struct ScreenReaderPlugin;

impl Plugin for ScreenReaderPlugin {
    fn namespace(&self) -> &str {
        ""
    }

    fn get_css_for_modifier(&self, modifier: &str, css_content: &mut String) -> Result {
        match modifier {
            "sr-only" => write!(css_content, "position: absolute;
  width: 1px;
  height: 1px;
  padding: 0;
  margin: -1px;
  overflow: hidden;
  clip: rect(0, 0, 0, 0);
  white-space: nowrap;
  border-width: 0;"),
            "not-sr-only" => write!(css_content, "position: static;
  width: auto;
  height: auto;
  padding: 0;
  margin: 0;
  overflow: visible;
  clip: auto;
  white-space: normal;"),
            _ => Ok(()),
        }
    }
}
