use super::framework::{Entry, SourceFormatter, Name, Argument, DefaultValue, Convertible};
use super::Args;

pub struct Grepper {
    /// A regular expression used for searching.
    regex_pattern: Entry,
    /// A file or directory to search. Directories may be searched recursively.
    file: Entry,
}

pub const GREP: Grepper = Grepper {
    regex_pattern: Entry { 
        defaults_to: DefaultValue::Mandatory,
        source: SourceFormatter::Default,
        target_name: Name::Blank(0),
        target_type: Argument::Text(None),
    },
    file: Entry {
        defaults_to: DefaultValue::Default("-"),
        source: SourceFormatter::Default,
        target_name: Name::Blank(1),
        target_type: Argument::PathPattern(None),
    },
};

impl Convertible<Args> for Grepper {
    /// Yipeee ^-^
    /// 
    /// First we resolve positions,
    ///   Try to know where each element will be.
    ///   As a rule of thumb, we'd rather want elements with names to be placed later. 
    ///   Not that this would matter that much, usually!
    ///   
    ///   What we do here, is log which arguments have a fixed position, 
    ///     then throw all the non-ordered ones after.
    ///   To optimize the whole thing, we generate the arguments in the same time;
    ///     throw the non-positionals in a vec, and the positionals in a tree. 
    fn generate(&self, with: Args) -> Vec<String> {
        return Vec::new();
    }
}