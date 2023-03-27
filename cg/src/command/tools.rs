use super::framework::{Entry, Source, SourceName, Name, Argument, CollectionArgument, DefaultValue};

pub const grep: [Entry; 2] = [ //We list each entry. Yes its a bit disappointing of a method, but! it works
    Entry{
        defaults_to: DefaultValue::Mandatory,
        source: Source::DefaultSource(SourceName::RegexPattern),
        target_name: Name::Blank(1),
        target_type: Argument::RegexPattern,
    },
    Entry{
        defaults_to: DefaultValue::Skip,
        source: Source::DefaultSource(SourceName::PathPattern),
        target_name: Name::Blank(2),
        target_type: Argument::Collection(CollectionArgument::PathPattern),
    },
];