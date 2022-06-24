use super::value_matchers::*;

use std::fmt;

pub const SHADOW_KEYWORDS: [&str; 5] = ["none", "inherit", "initial", "revert", "unset"];

#[derive(Debug, PartialEq)]
pub enum Shadow<'a> {
    Raw([Option<&'a str>; 6]),
    Keyword(&'a str),
    Variable(&'a str),
    Shorthand1 {
        is_inset: bool,
        offset_x: &'a str,
        offset_y: &'a str,
        color: &'a str,
    },
    Shorthand2 {
        is_inset: bool,
        offset_x: &'a str,
        offset_y: &'a str,
        blur_radius: &'a str,
        color: &'a str,
    },
    Full {
        is_inset: bool,
        offset_x: &'a str,
        offset_y: &'a str,
        blur_radius: &'a str,
        spread_radius: &'a str,
        color: &'a str,
    },
}

impl<'a> Shadow<'a> {
    pub fn new_raw() -> Self {
        Self::Raw([None; 6])
    }

    /// Parse a real {`Shadow`] from a [`Shadow::Raw`] variant
    pub fn parse(&self) -> Option<Self> {
        if let Shadow::Raw(shadow) = self {
            // Handle inset shadows
            let (is_inset, shadow) = if let Some(shadow_first_part) = shadow[0] {
                if shadow_first_part == "inset" {
                    (true, &shadow[1..])
                } else {
                    (false, &shadow[..])
                }
            } else {
                (false, &shadow[..])
            };

            // Check the number of parts
            if shadow.is_empty()
                && ((is_inset && shadow.len() > 6) || (!is_inset && shadow.len() > 5))
            {
                return None;
            }

            let len = shadow.iter().position(|p| p.is_none()).unwrap_or(6);
            if len == 1 {
                // Keyword value
                if SHADOW_KEYWORDS.contains(&shadow[0].unwrap()) {
                    Some(Shadow::Keyword(shadow[0].unwrap()))
                } else if is_matching_var(shadow[0].unwrap()) {
                    Some(Shadow::Variable(shadow[0].unwrap()))
                } else {
                    None
                }
            } else if len == 3 {
                // Shorthand 1: offset-x | offset-y | color
                if is_matching_length(shadow[0].unwrap())
                    && is_matching_length(shadow[1].unwrap())
                    && is_matching_color(shadow[2].unwrap())
                {
                    Some(Shadow::Shorthand1 {
                        is_inset,
                        offset_x: shadow[0].unwrap(),
                        offset_y: shadow[1].unwrap(),
                        color: shadow[2].unwrap(),
                    })
                } else {
                    None
                }
            } else if len == 4 {
                // Shorthand 2: offset-x | offset-y | blur-radius | color
                if is_matching_length(shadow[0].unwrap())
                    && is_matching_length(shadow[1].unwrap())
                    && is_matching_length(shadow[2].unwrap())
                    && is_matching_color(shadow[3].unwrap())
                {
                    Some(Shadow::Shorthand2 {
                        is_inset,
                        offset_x: shadow[0].unwrap(),
                        offset_y: shadow[1].unwrap(),
                        blur_radius: shadow[2].unwrap(),
                        color: shadow[3].unwrap(),
                    })
                } else {
                    None
                }
            } else if len == 5 {
                // Full: offset-x | offset-y | blur-radius | spread-radius | color
                if is_matching_length(shadow[0].unwrap())
                    && is_matching_length(shadow[1].unwrap())
                    && is_matching_length(shadow[2].unwrap())
                    && is_matching_length(shadow[3].unwrap())
                    && is_matching_color(shadow[4].unwrap())
                {
                    Some(Shadow::Full {
                        is_inset,
                        offset_x: shadow[0].unwrap(),
                        offset_y: shadow[1].unwrap(),
                        blur_radius: shadow[2].unwrap(),
                        spread_radius: shadow[3].unwrap(),
                        color: shadow[4].unwrap(),
                    })
                } else {
                    None
                }
            } else {
                None
            }
        } else {
            None
        }
    }
}

impl<'a> fmt::Display for Shadow<'a> {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        match self {
            Shadow::Raw(_) => panic!("Trying to print a `Shadow::Raw` variant"),
            Shadow::Keyword(keyword) => write!(f, "{keyword}"),
            Shadow::Variable(variable) => write!(f, "{variable}"),
            Shadow::Shorthand1 {
                is_inset,
                offset_x,
                offset_y,
                color,
            } => write!(
                f,
                "{}{offset_x} {offset_y} {color}",
                if *is_inset { "inset " } else { "" }
            ),
            Shadow::Shorthand2 {
                is_inset,
                offset_x,
                offset_y,
                blur_radius,
                color,
            } => write!(
                f,
                "{}{offset_x} {offset_y} {blur_radius} {color}",
                if *is_inset { "inset " } else { "" }
            ),
            Shadow::Full {
                is_inset,
                offset_x,
                offset_y,
                blur_radius,
                spread_radius,
                color,
            } => write!(
                f,
                "{}{offset_x} {offset_y} {blur_radius} {spread_radius} {color}",
                if *is_inset { "inset " } else { "" }
            ),
        }
    }
}

#[derive(Debug, PartialEq)]
pub struct ShadowList<'a>(Vec<Shadow<'a>>);

impl<'a> ShadowList<'a> {
    pub fn replace_all_colors(&mut self, new_color: &'a str) {
        self.0.iter_mut().for_each(|shadow| match shadow {
            Shadow::Raw(_) => (),
            Shadow::Keyword(_) => (),
            Shadow::Variable(_) => (),
            Shadow::Shorthand1 { ref mut color, .. }
            | Shadow::Shorthand2 { ref mut color, .. }
            | Shadow::Full { ref mut color, .. } => *color = new_color,
        });
    }
}

impl<'a> fmt::Display for ShadowList<'a> {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        for (i, v) in self.0.iter().enumerate() {
            write!(f, "{}{}", v, if i != self.0.len() - 1 { "," } else { "" })?;
        }

        Ok(())
    }
}

impl<'a> From<Vec<Shadow<'a>>> for ShadowList<'a> {
    fn from(v: Vec<Shadow<'a>>) -> Self {
        Self(v)
    }
}

// https://developer.mozilla.org/en-US/docs/Web/CSS/box-shadow
pub fn parse_shadow(value: &str) -> Option<ShadowList> {
    let mut parenthesis_level = 0;
    let mut last_index = 0;
    let mut shadows = vec![Shadow::new_raw()];

    for (ch_index, ch) in value.chars().enumerate() {
        match ch {
            '(' => {
                parenthesis_level += 1;
            }
            ')' => {
                parenthesis_level -= 1;
            }
            ' ' if parenthesis_level == 0 => {
                let shadow = if let Shadow::Raw(shadow) = shadows.last_mut().unwrap() {
                    shadow
                } else {
                    // Shadow already parsed but a space was encountered
                    return None;
                };

                if !value[last_index..ch_index].is_empty() {
                    // Find the index of the first free part
                    let index = shadow.iter().position(|p| p.is_none()).unwrap_or(5);

                    // Insert the part
                    shadow[index] = Some(&value[last_index..ch_index]);
                }

                // Update the index (and ignore the space)
                last_index = ch_index + 1;
            }
            ',' if parenthesis_level == 0 => {
                // Add the last part (not suffixed by `_`)
                let shadow = if let Shadow::Raw(shadow) = shadows.last_mut().unwrap() {
                    shadow
                } else {
                    // Shadow already parsed but a space was encountered
                    return None;
                };

                // Find the index of the first free part
                let index = shadow.iter().position(|p| p.is_none()).unwrap_or(5);

                // Insert the part
                shadow[index] = Some(&value[last_index..ch_index]);

                // Ignore the shadow if it is empty
                if !shadow.iter().all(Option::is_none) {
                    // Parse the shadow
                    let parsed_shadow = shadows.last().unwrap().parse()?;
                    *shadows.last_mut().unwrap() = parsed_shadow;

                    // Start the next shadow
                    shadows.push(Shadow::new_raw());
                }

                // Update the index (and ignore the comma)
                last_index = ch_index + 1;
            }
            _ => (),
        }
    }

    // Add the last part (not suffixed by `,`)
    if last_index != value.len() - 1 {
        // Find the index of the first free part
        let shadow = if let Shadow::Raw(shadow) = shadows.last_mut().unwrap() {
            shadow
        } else {
            return None;
        };
        let index = shadow.iter().position(|p| p.is_none()).unwrap_or(5);

        // Insert the part
        shadow[index] = Some(&value[last_index..value.len()]);

        // Parse the shadow
        let parsed_shadow = shadows.last().unwrap().parse()?;
        *shadows.last_mut().unwrap() = parsed_shadow;
    }

    Some(ShadowList::from(shadows))
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn parse_shadow_test() {
        let shadow = "20px 35px 60px -15px rgba(0,0,0,0.3),0 72px rgba(0,2,42,0.2),inset 23px 42em 42px rgba(255,0,0,1)";
        let result = parse_shadow(shadow).unwrap();
        assert_eq!(
            result,
            ShadowList(vec![
                Shadow::Full {
                    is_inset: false,
                    offset_x: "20px",
                    offset_y: "35px",
                    blur_radius: "60px",
                    spread_radius: "-15px",
                    color: "rgba(0,0,0,0.3)",
                },
                Shadow::Shorthand1 {
                    is_inset: false,
                    offset_x: "0",
                    offset_y: "72px",
                    color: "rgba(0,2,42,0.2)",
                },
                Shadow::Shorthand2 {
                    is_inset: true,
                    offset_x: "23px",
                    offset_y: "42em",
                    blur_radius: "42px",
                    color: "rgba(255,0,0,1)",
                }
            ])
        );

        assert_eq!(
            parse_shadow("var(--a, 0 0 1px rgb(0, 0, 0)),1px 2px 3rem rgb(0, 0, 0)").unwrap(),
            ShadowList(vec![
                Shadow::Variable("var(--a, 0 0 1px rgb(0, 0, 0))"),
                Shadow::Shorthand2 {
                    is_inset: false,
                    offset_x: "1px",
                    offset_y: "2px",
                    blur_radius: "3rem",
                    color: "rgb(0, 0, 0)",
                },
            ])
        );

        assert_eq!(
            parse_shadow("none").unwrap(),
            ShadowList(vec![Shadow::Keyword("none")])
        );
    }

    #[test]
    fn format_shadow() {
        let shadow = "20px 35px 60px -15px rgba(0,0,0,0.3),0 72px rgba(0,2,42,0.2),inset 23px 42em rgba(255,0,0,1)";
        let result = parse_shadow(shadow).unwrap();
        assert_eq!(&result.to_string(), shadow);
    }
}
