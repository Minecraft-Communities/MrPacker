
use regex::Regex;

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

impl fmt::Display for VersionPart {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        // TODO
        write!(f, "{}", self.value)
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
    fn new(textIn: String, partsIn: Box<[VersionPart]>) -> Self {
        Self{
            text: textIn,
            parts: partsIn,
        }
    }
}

impl FromStr for Version {
    type Err = BadVersionString;

    fn from_str(textRaw: &str) -> Result<Self, Self::Err> {
        if textRaw.is_empty() || textRaw == "*" {
            return Ok(Version{
                text: String::from("*"),
                parts: Box::new([]),
            });
        }

        let mut processedText: String = textRaw.to_lowercase()
            .replace("alpha", "0")
            .replace("beta", "1")
            .replace("pre-release", "2")
            .replace("pre", "2")
            .replace("rc", "2")
            .replace("snapshot", "2")
            .replace("release", "3")
            ;

        // convert all possible delimiters into a single delimiter before splitting
        processedText = processedText
            .replace("+", "-")
            .replace("_", "-")
            .replace(":", "-")
            ;
        
        let mut validParts = vec![];
        let partCandidates = processedText.split('-')
        for mut candidate in partCandidates {
            if candidate == '' {
                continue;
            }

            // disallow candidates that are:
            //   - text-only
            //   - commit refs
            if Regex::new(r"(?!\.)[0-9]*[a-z]+[0-9a-z]*$").unwrap().is_match(candidate) {
                continue;
            }
            else if Regex::new(r"^[a-z0-9.]+$").unwrap().is_match(candidate) {
                for word in Regex::new(r"[.]*([a-z]+[.]+)").unwrap().find_iter(candidate) {
                    candidate = candidate.replace(word, "");
                }

                for letter in Regex::new(r"[0-9]([a-z])").unwrap().find_iter(candidate) {
                    let idx = (letter.as_str().chars().next().unwrap() as u32) - ('a' as u32) + 1;
                    candidate = candidate.replace(letter.as_str(), idx.to_string());
                }

                for letter in Regex::new(r"([a-z])").unwrap().find_iter(candidate) {
                    let idx = (letter.as_str().chars().next().unwrap() as u32) - ('a' as u32) + 1;
                    candidate = candidate.replace(letter.as_str(), idx.to_string());
                }

                validParts.push(candidate);
            }
        }

        if !valid_parts.is_empty() {
            let mut versionParts = Vec::new();
            for part in validParts {
                let version_part_pieces = part
                    .split('.')
                    .filter_map(|piece| piece.parse::<u32>().ok())
                    .collect::<Vec<u32>>()
                    ;
                versionParts.push(VersionPart::new(versionPartPieces));
            }
            Ok(Version::new(processedText, versionParts))
        }
        else {
            Err(BadVersionString {
                message: format!("Invalid version format: '{}'", textRaw).to_string(),
            })
        }
    }
}

impl fmt::Display for Version {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        // TODO
        write!(f, "{}", self.value)
    }
}

impl PartialEq for Version {
    fn eq(&self, other: &Self) -> bool {
        let max_size = std::cmp::max(self.parts.len(), other.parts.len());
        for i in 0..max_size {
            let a: u32 =
                if self.parts.len() > i {
                    self.parts[i]
                } else {
                    0
                };
            let b: u32 =
                if other.parts.len() > i {
                    other.parts[i]
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

impl PartialOrd for Version {
    fn partial_cmp(&self, other: &Self) -> Option<std::cmp::Ordering> {
        // TODO
        self.parts.partial_cmp(&other.parts)
    }

    fn lt(&self, other: &Self) -> bool {
        let max_size = std::cmp::max(self.parts.len(), other.parts.len());
        for i in 0..max_size {
            let a: u32 =
                if self.parts.len() > i {
                    self.parts[i]
                } else {
                    0
                };
            let b: u32 =
                if other.parts.len() > i {
                    other.parts[i]
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
        let max_size = std::cmp::max(self.parts.len(), other.parts.len());
        for i in 0..max_size {
            let a: u32 =
                if self.parts.len() > i {
                    self.parts[i]
                } else {
                    0
                };
            let b: u32 =
                if other.parts.len() > i {
                    other.parts[i]
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
        let max_size = std::cmp::max(self.parts.len(), other.parts.len());
        for i in 0..max_size {
            let a: u32 =
                if self.parts.len() > i {
                    self.parts[i]
                } else {
                    0
                };
            let b: u32 =
                if other.parts.len() > i {
                    other.parts[i]
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
        let max_size = std::cmp::max(self.parts.len(), other.parts.len());
        for i in 0..max_size {
            let a: u32 =
                if self.parts.len() > i {
                    self.parts[i]
                } else {
                    0
                };
            let b: u32 =
                if other.parts.len() > i {
                    other.parts[i]
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

impl VersionRangePart {
    fn new(boundaryIn: Version, inclusiveIn: bool) -> Self {
        Self{
            boundary: boundaryIn,
            inclusive: inclusiveIn,
        }
    }
}

impl VersionRange {
    fn contains(&self, version: &Version) -> bool {
        let mut result: bool = true;
        let mut comparison: bool;

        if self.lower.bound.text == "*" {} // fall through
        else if self.lower.inclusive {
            comparison = (self.lower.bound <= version);
            result = result && comp;
        }
        else {
            comparison = (self.lower.bound < version);
            result = result && comp;
        }

        if self.upper.bound.text == "*" {} // fall through
        else if self.upper.inclusive {
            comparison = (self.upper.bound >= version);
            result = result && comp;
        }
        else {
            comparison = (self.upper.bound > version);
            result = result && comp;
        }

        return result;
    }
}

impl fmt::Display for VersionRange {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        // TODO
        write!(f, "{}", self.value)
    }
}
