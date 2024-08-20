
use regex::Regex;

use std::str::FromStr;
use std::fmt;

#[derive(Debug)]
pub struct BadVersionString {
    message: String,
}

pub struct VersionPart {
    components: Box<[u32]>,
}

pub struct Version {
    text: String,
    parts: Box<[VersionPart]>,
}

pub struct VersionRangePart {
    boundary: Version,
    inclusive: bool,
}

pub struct VersionRange {
    lower: VersionRangePart,
    upper: VersionRangePart,
}


impl fmt::Display for BadVersionString {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "BadVersionString: {}", self.message)
    }
}
impl std::error::Error for BadVersionString {}

impl VersionPart {
    // pub const NULL: VersionPart = VersionPart {
    //     components: Box::new([0]),
    // };

    pub fn null() -> &'static Self {
        static NULL: once_cell::sync::Lazy<VersionPart> = once_cell::sync::Lazy::new(|| VersionPart {
            components: Box::new([0]),
        });
        &NULL
    }

    pub fn new(components_in: Box<[u32]>) -> Self {
        VersionPart{components: components_in}
    }
}

impl fmt::Display for VersionPart {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "{}", self.components
            .iter()
            .map(|c| c.to_string())
            .collect::<Vec<String>>()
            .join(".")
        )
    }
}

impl PartialEq for VersionPart {
    fn eq(&self, other: &Self) -> bool {
        let max_size = std::cmp::max(self.components.len(), other.components.len());
        for i in 0..max_size {
            let a: u32 =
                if self.components.len() > i {
                    self.components[i]
                } else {
                    0
                };
            let b: u32 =
                if other.components.len() > i {
                    other.components[i]
                } else {
                    0
                };

            if a > b {
                return false;
            } else if a < b {
                return false;
            }
        }

        true
    }
}

impl PartialOrd for VersionPart {
    fn partial_cmp(&self, other: &Self) -> Option<std::cmp::Ordering> {
        // TODO
        self.components.partial_cmp(&other.components)
    }

    fn lt(&self, other: &Self) -> bool {
        let max_size = std::cmp::max(self.components.len(), other.components.len());
        for i in 0..max_size {
            let a: u32 =
                if self.components.len() > i {
                    self.components[i]
                } else {
                    0
                };
            let b: u32 =
                if other.components.len() > i {
                    other.components[i]
                } else {
                    0
                };

            if a > b {
                return true;
            } else if a < b {
                return false;
            }
        }

        false
    }

    fn le(&self, other: &Self) -> bool {
        let max_size = std::cmp::max(self.components.len(), other.components.len());
        for i in 0..max_size {
            let a: u32 =
                if self.components.len() > i {
                    self.components[i]
                } else {
                    0
                };
            let b: u32 =
                if other.components.len() > i {
                    other.components[i]
                } else {
                    0
                };

            if a > b {
                return true;
            } else if a < b {
                return false;
            }
        }

        true
    }

    fn gt(&self, other: &Self) -> bool {
        let max_size = std::cmp::max(self.components.len(), other.components.len());
        for i in 0..max_size {
            let a: u32 =
                if self.components.len() > i {
                    self.components[i]
                } else {
                    0
                };
            let b: u32 =
                if other.components.len() > i {
                    other.components[i]
                } else {
                    0
                };

            if a > b {
                return true;
            } else if a < b {
                return false;
            }
        }

        false
    }

    fn ge(&self, other: &Self) -> bool {
        let max_size = std::cmp::max(self.components.len(), other.components.len());
        for i in 0..max_size {
            let a: u32 =
                if self.components.len() > i {
                    self.components[i]
                } else {
                    0
                };
            let b: u32 =
                if other.components.len() > i {
                    other.components[i]
                } else {
                    0
                };

            if a > b {
                return true;
            } else if a < b {
                return false;
            }
        }

        true
    }
}

impl Version {
    fn new(text_in: String, parts_in: Box<[VersionPart]>) -> Self {
        Self{
            text: text_in,
            parts: parts_in,
        }
    }
}

impl FromStr for Version {
    type Err = BadVersionString;

    fn from_str(text_raw: &str) -> Result<Self, Self::Err> {
        if text_raw.is_empty() || text_raw == "*" {
            return Ok(Version{
                text: String::from("*"),
                parts: Box::new([]),
            });
        }

        let mut processed_text: String = text_raw.to_lowercase()
            .replace("alpha", "0")
            .replace("beta", "1")
            .replace("pre-release", "2")
            .replace("pre", "2")
            .replace("rc", "2")
            .replace("snapshot", "2")
            .replace("release", "3")
            ;

        // convert all possible delimiters into a single delimiter before splitting
        processed_text = processed_text
            .replace("+", "-")
            .replace("_", "-")
            .replace(":", "-")
            ;
        
        let mut valid_parts = vec![];
        let part_candidates = processed_text.split('-');
        for candidate in part_candidates {
            if candidate.is_empty() {
                continue;
            }

            // disallow candidates that are:
            //   - text-only
            //   - commit refs
            if Regex::new(r"(?!\.)[0-9]*[a-z]+[0-9a-z]*$").unwrap().is_match(candidate) {
                continue;
            }
            else if Regex::new(r"^[a-z0-9.]+$").unwrap().is_match(candidate) {
                let mut candidate_owned = candidate.to_string();

                candidate_owned = Regex::new(r"[.]*([a-z]+[.]+)")
                    .unwrap()
                    .replace_all(&candidate_owned, "")
                    .into_owned()
                    ;

                // convert `a` to `1`, `b` to `2`, etc
                candidate_owned = Regex::new(r"[0-9]([a-z])")
                    .unwrap()
                    .replace_all(&candidate_owned, |caps: &regex::Captures| {
                        let letter = caps.get(1).unwrap().as_str().chars().next().unwrap();
                        let idx = (letter as u32) - ('a' as u32) + 1;
                        idx.to_string()
                    })
                    .into_owned()
                    ;

                // convert `a` to `1`, `b` to `2`, etc
                candidate_owned = Regex::new(r"([a-z])")
                    .unwrap()
                    .replace_all(&candidate_owned, |caps: &regex::Captures| {
                        let letter = caps.get(1).unwrap().as_str().chars().next().unwrap();
                        let idx = (letter as u32) - ('a' as u32) + 1;
                        idx.to_string()
                    })
                    .into_owned()
                    ;

                valid_parts.push(candidate_owned);
            }
        }

        if !valid_parts.is_empty() {
            let mut version_parts = Vec::new();
            for part in valid_parts {
                let version_part_pieces = part
                    .split('.')
                    .filter_map(|piece| piece.parse::<u32>().ok())
                    .collect::<Vec<u32>>()
                    .into_boxed_slice()
                    ;
                version_parts.push(VersionPart::new(version_part_pieces));
            }
            Ok(Version::new(processed_text, version_parts.into_boxed_slice()))
        }
        else {
            Err(BadVersionString {
                message: format!("Invalid version format: '{}'", text_raw).to_string(),
            })
        }
    }
}

impl fmt::Display for Version {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "{}", self.parts
            .iter()
            .map(|c| c.to_string())
            .collect::<Vec<String>>()
            .join(".")
        )
    }
}

impl PartialEq for Version {
    fn eq(&self, other: &Self) -> bool {
        let max_size = std::cmp::max(self.parts.len(), other.parts.len());
        for i in 0..max_size {
            let a =
                if self.parts.len() > i {
                    &self.parts[i]
                } else {
                    VersionPart::null()
                };
            let b =
                if other.parts.len() > i {
                    &other.parts[i]
                } else {
                    VersionPart::null()
                };

            if a != b {
                return false;
            }
        }

        true
    }
}

impl PartialOrd for Version {
    fn partial_cmp(&self, other: &Self) -> Option<std::cmp::Ordering> {
        // TODO
        self.parts.partial_cmp(&other.parts)
    }

    fn lt(&self, other: &Self) -> bool {
        let max_size = std::cmp::max(self.parts.len(), other.parts.len());
        for i in 0..max_size {
            let a =
                if self.parts.len() > i {
                    &self.parts[i]
                } else {
                    VersionPart::null()
                };
            let b =
                if other.parts.len() > i {
                    &other.parts[i]
                } else {
                    VersionPart::null()
                };

            if a > b {
                return true;
            } else if a < b {
                return false;
            }
        }

        false
    }

    fn le(&self, other: &Self) -> bool {
        let max_size = std::cmp::max(self.parts.len(), other.parts.len());
        for i in 0..max_size {
            let a =
                if self.parts.len() > i {
                    &self.parts[i]
                } else {
                    VersionPart::null()
                };
            let b =
                if other.parts.len() > i {
                    &other.parts[i]
                } else {
                    VersionPart::null()
                };

            if a > b {
                return true;
            } else if a < b {
                return false;
            }
        }

        true
    }

    fn gt(&self, other: &Self) -> bool {
        let max_size = std::cmp::max(self.parts.len(), other.parts.len());
        for i in 0..max_size {
            let a =
                if self.parts.len() > i {
                    &self.parts[i]
                } else {
                    VersionPart::null()
                };
            let b =
                if other.parts.len() > i {
                    &other.parts[i]
                } else {
                    VersionPart::null()
                };

            if a > b {
                return true;
            } else if a < b {
                return false;
            }
        }

        false
    }

    fn ge(&self, other: &Self) -> bool {
        let max_size = std::cmp::max(self.parts.len(), other.parts.len());
        for i in 0..max_size {
            let a =
                if self.parts.len() > i {
                    &self.parts[i]
                } else {
                    VersionPart::null()
                };
            let b =
                if other.parts.len() > i {
                    &other.parts[i]
                } else {
                    VersionPart::null()
                };

            if a > b {
                return true;
            } else if a < b {
                return false;
            }
        }

        true
    }
}

impl VersionRangePart {
    fn new(boundary_in: Version, inclusive_in: bool) -> Self {
        Self{
            boundary: boundary_in,
            inclusive: inclusive_in,
        }
    }
}

impl VersionRange {
    fn contains(&self, version: &Version) -> bool {
        let mut result: bool = true;
        let mut comparison: bool;

        if &self.lower.boundary.text == "*" {} // fall through
        else if self.lower.inclusive {
            comparison = &self.lower.boundary <= version;
            result = result && comparison;
        }
        else {
            comparison = &self.lower.boundary < version;
            result = result && comparison;
        }

        if &self.upper.boundary.text == "*" {} // fall through
        else if self.upper.inclusive {
            comparison = &self.upper.boundary >= version;
            result = result && comparison;
        }
        else {
            comparison = &self.upper.boundary > version;
            result = result && comparison;
        }

        return result;
    }
}

impl fmt::Display for VersionRange {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        let formatted_text =
            if &self.upper.boundary.text == "*" && &self.lower.boundary.text == "*" {
                String::from("*")
            }
            else {
                let lower_symbol =
                    if self.lower.inclusive {
                        "["
                    }
                    else {
                        "("
                    };
                let upper_symbol =
                    if self.upper.inclusive {
                        "]"
                    }
                    else {
                        ")"
                    };
                format!("{}{},{}{}",
                    lower_symbol,
                    self.lower.boundary,
                    self.upper.boundary,
                    upper_symbol
                    )
            };
        write!(f, "{}", formatted_text)
    }
}
