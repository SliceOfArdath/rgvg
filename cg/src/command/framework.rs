use std::path::PathBuf;

/// An item from the source, the data asked by the formatter.
pub enum SourceName {
    /// Mandatory, designs the regex pattern to find
    RegexPattern,
    /// Optional, designs the file path to search in
    PathPattern,
}
/// A name of the form -<short> or --<long>. The name sent as a command. Short and long as thus mutually excusive here.
pub enum Name {
    /// A short name, format -<short>, i.e. -j
    Short(char),
    /// A long name, format --<long>, i.e. --exclude
    Long(String),
    /// A blank name, positional. The position is only in regard to other blanks.
    /// The values used for position are not nearly as important as their order (think like z-level for 2d renderers).
    Blank(u8),
    /// Skip this entry
    Undefined,
}
pub enum Argument {
    Collection(CollectionArgument),
    PathPattern,
    RegexPattern,
    /// Ha
    BooleanFlag,
    /// Skip this entry. Useless?
    Undefined,
}

pub enum CollectionArgument {
    PathPattern,
    RegexPattern,
    /// Ha
    BooleanFlag,
}
pub enum Source {
    // A source transformed directly to the recipient
    DefaultSource(SourceName),
    // A source formatted and/or filtered to answer to a specific specification
    FormattedSource(SourceName, SourceFormatter),

}
pub enum SourceFormatter {
    
}

impl Default for Name {
    fn default() -> Self {
        return Name::Undefined;
    }
}

pub enum DefaultValue {
    Mandatory, //CANNOT be ommited
    Skip, //Just forget it lol
    Default(String), //provide a default. This default is constant! may provide a formatter later lol
}

pub struct Entry {
    pub defaults_to: DefaultValue,
    pub source: Source,
    pub target_name: Name,
    pub target_type: Argument,
}

