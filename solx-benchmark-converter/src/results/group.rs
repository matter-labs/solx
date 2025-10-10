//!
//! A group of results
//!

use crate::model::evm_interpreter;
use regex::Regex;
use std::fmt::Display;

///
/// Group of results.
///
#[derive(Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
pub enum Group<'a> {
    /// A group with EVM interpreter tests.
    EVMInterpreter {
        /// Codegen used to produce executables for all tests in this group.
        codegen: &'a str,
        /// Optimization level used to produce executables for all tests in this group.
        optimizations: &'a str,
    },
    /// A default group containing all tests.
    Default {
        /// Codegen used to produce executables for all tests in this group.
        codegen: &'a str,
        /// Optimization level used to produce executables for all tests in this group.
        optimizations: &'a str,
    },
    /// A user-named group.
    Named {
        /// Group name, provided by the user in the test definition file.
        name: &'a str,
        /// Codegen used to produce executables for all tests in this group.
        codegen: &'a str,
        /// Optimization level used to produce executables for all tests in this group.
        optimizations: &'a str,
    },
    /// A group comparing two groups with distinct names:
    /// - one belonging to a reference run,
    /// - another belonging to a candidate run.
    Comparison {
        /// Group belonging to the reference run.
        reference: Box<Group<'a>>,
        /// Group belonging to the candidate run.
        candidate: Box<Group<'a>>,
    },
}

impl Group<'_> {
    fn comparison_name(reference: &Group<'_>, candidate: &Group<'_>) -> String {
        if reference.name() == candidate.name() {
            format!(
                "{}: {}{} vs {}{}",
                reference.name(),
                reference.codegen().unwrap_or_default(),
                reference.optimizations().unwrap_or_default(),
                candidate.codegen().unwrap_or_default(),
                candidate.optimizations().unwrap_or_default(),
            )
        } else {
            format!("{} vs {}", reference.name(), candidate.name())
        }
    }

    ///
    /// Returns true if the provided regular expression matches the string representation of the group.
    ///
    pub fn regex_matches(&self, regex: &Regex) -> bool {
        !self.is_comparison() && (regex.is_match(&self.to_string()))
    }

    ///
    /// Codegen used in this group.
    ///
    pub fn codegen(&self) -> Option<String> {
        match self {
            Group::EVMInterpreter { codegen, .. } => Some(codegen.to_string()),
            Group::Default { codegen, .. } => Some(codegen.to_string()),
            Group::Named { codegen, .. } => Some(codegen.to_string()),
            Group::Comparison { .. } => None,
        }
    }

    ///
    /// Optimizations used in this group.
    ///
    pub fn optimizations(&self) -> Option<String> {
        match self {
            Group::EVMInterpreter { optimizations, .. } => Some(optimizations.to_string()),
            Group::Default { optimizations, .. } => Some(optimizations.to_string()),
            Group::Named { optimizations, .. } => Some(optimizations.to_string()),
            Group::Comparison { .. } => None,
        }
    }

    ///
    /// Name of the group.
    ///
    pub fn name(&self) -> String {
        match self {
            Group::EVMInterpreter { .. } => "EVMInterpreter".into(),
            Group::Default { .. } => "All".into(),
            Group::Named { name, .. } => name.to_string(),
            Group::Comparison { .. } => "Comparison".into(),
        }
    }

    /// Returns `true` if the group is [`Comparison`].
    ///
    /// [`Comparison`]: Group::Comparison
    #[must_use]
    pub fn is_comparison(&self) -> bool {
        matches!(self, Self::Comparison { .. })
    }
}

impl<'a> Group<'a> {
    ///
    /// Create a new group provided an optional tag, codegen and optimization level.
    ///
    pub fn from_tag(tag: Option<&'a str>, codegen: Option<&'a str>, opt: Option<&'a str>) -> Self {
        let codegen = codegen.unwrap_or_default();
        let optimizations = opt.unwrap_or_default();
        match tag {
            None => Self::Default {
                optimizations,
                codegen,
            },
            Some(group_name) if group_name == evm_interpreter::GROUP_NAME => Self::EVMInterpreter {
                optimizations,
                codegen,
            },
            Some(name) => Self::Named {
                name,
                codegen,
                optimizations,
            },
        }
    }

    ///
    /// Create a new group that compares two groups with distinct names:
    /// - one belonging to a reference run,
    /// - another belonging to a candidate run.
    ///
    pub fn new_comparison(reference: &Self, candidate: &Self) -> Self {
        Self::Comparison {
            reference: Box::new(reference.clone()),
            candidate: Box::new(candidate.clone()),
        }
    }
}

impl Display for Group<'_> {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            Group::EVMInterpreter {
                codegen,
                optimizations,
            } => f.write_fmt(format_args!("{} {codegen} {optimizations}", self.name())),
            Group::Default {
                codegen,
                optimizations,
            } => f.write_fmt(format_args!("{} {codegen} {optimizations}", self.name())),
            Group::Named {
                name,
                codegen,
                optimizations,
            } => f.write_fmt(format_args!("{name} {codegen} {optimizations}")),
            Group::Comparison {
                reference,
                candidate,
            } => f.write_str(&Self::comparison_name(reference, candidate)),
        }
    }
}
