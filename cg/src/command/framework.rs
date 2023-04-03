use std::path::PathBuf;
use std::fmt::{self, Display};
use regex::Regex;

type Index = u8;

/// A name of the form -<short> or --<long>. The name sent as a command. Short and long as thus mutually excusive here.
pub enum Name {
    /// A short name, format -<short>, i.e. -j
    Short(char),
    /// A long name, format --<long>, i.e. --exclude
    Long(String),
    /// A blank name, positional. The position is only in regard to other blanks.
    /// The values used for position are not nearly as important as their order (think like z-level for 2d renderers).
    Blank(Index),
    /// Skip this entry
    Undefined,
}

impl Default for Name {
    fn default() -> Self {
        return Name::Undefined;
    }
}
impl Display for Name {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            Name::Short(c) => write!(f, "-{}", c),
            Name::Long(s) => write!(f, "--{}", s),
            Name::Blank(_) => fmt::Result::Ok(()), //We do not process the index yet.
            _ => panic!("Unsupported name type!"),
        }
    }
}
impl TryFrom<(regex::Match<'_>, &str)> for Name {
    type Error = Error;
    fn try_from(value: (regex::Match, &str)) -> Result<Self, Self::Error> {
        let s = value.0.as_str();
        match value.1 {
            "d1" => Ok(Name::Blank(s[1..].parse().unwrap())),
            "d2" => Ok(Name::Short(s.chars().nth(1).unwrap())),
            "d3" => Ok(Name::Long(s[2..].to_string())),
            _ => Err(Error {  })
        } 
    }
}

#[derive(Debug)]
pub enum Argument {
    /// A collection type. Hopefully.
    CollectionText(Option<Vec<String>>),
    /// A string designating a file (or a PathBuff)
    PathPattern(Option<PathBuf>),
    /// A list of file desibnators
    CollectionPathPattern(Option<Vec<PathBuf>>),
    /// A regular string
    Text(Option<String>),
    /// Either there or not.. What do the stars say, my dear pippin, what do they say? - That we fight the good cause, merry. That we will see each other in the end.
    BooleanFlag(Option<bool>),
    /// Skip this entry. Useless?
    Undefined,
}

pub trait Transformable {
    /// Turn the clap data into valid arguments
    fn transform(self, transformer: &Entry) -> Self;
    /// Prepare the argument for command start.
    ///  This does not process collections by itself!
    fn convert(self) -> Option<String>;
}
pub trait Transform<T> {
    fn transform(self, value: T) -> Self;
    fn convert(self) -> Option<T>;
}

impl Transformable for Argument {
    fn transform(self, transformer: &Entry) -> Argument {
        match self {
            Argument::BooleanFlag(x) => match transformer.target_type {
                Argument::BooleanFlag(None) => Argument::BooleanFlag(x),
                _ => panic!("Unsupported conversion: BooleanFlag to {:?}", transformer.target_type),
            }
            Argument::Text(x) => match transformer.target_type {
                Argument::Text(None) => Argument::Text(x),
                _ => panic!("Unsupported conversion: Text to {:?}", transformer.target_type),
            },
            Argument::PathPattern(x) => match transformer.target_type {
                Argument::PathPattern(None) => Argument::PathPattern(x),
                _ => panic!("Unsupported conversion: PathPattern to {:?}", transformer.target_type),
            }
            _ => panic!("Unsupported type: {:?}", self),
        }
    }
    fn convert(self) -> Option<String> {
        match self {
            Argument::BooleanFlag(x) => Some(x?.to_string()),
            Argument::Text(x) => x, //that's a string!
            Argument::PathPattern(x) => Some(x?.display().to_string()),
            _ => panic!("Unsupported type: {:?}", self),
        }
    }
}
impl From<String> for Argument {
    fn from(value: String) -> Self {
        return Argument::Text(Some(value));
    }
}
impl From<PathBuf> for Argument {
    fn from(value: PathBuf) -> Self {
        return Argument::PathPattern(Some(value));
    }
}
/*impl Transform<String> for Entry {
    fn transform(&mut self, value: String) {
        self.target_type = match &self.target_type {
            Argument::Text(None) => Argument::Text(Some(value)),
            _ => panic!("Unspported transformation!")
        }
    }
}*/
impl Transform<String> for Argument {
    fn transform(self, value: String) -> Self {
        match self {
            Argument::Text(None) => Argument::Text(Some(value)),
            _ => panic!("Unspported transformation!")
        }
    }
}

/*impl Argument {
    pub const fn new(data: &str) -> Self {
        match data {
            "str" => Argument::Text(None),
            "path" => Argument::PathPattern(None),
            "path*" => Argument::CollectionPathPattern(None),
            _ => panic!("Invalid type!"),
        }
    }
}*/

pub enum SourceFormatter {
    /// No formatting
    Default,
    /// Filter in only certain elements, placeholder for now
    Filter,
}

pub enum DefaultValue {
    /// CANNOT be ommited
    Mandatory, 
    /// Just forgedaboutit
    Skip,
    /// provide a default. This default is constant! may provide a formatter later lol
    Default(&'static str),
}

pub struct Entry {
    pub defaults_to: DefaultValue,
    pub source: SourceFormatter,
    pub target_name: Name,
    pub target_type: Argument,
}
pub struct Error {

}

impl PartialEq for Entry {
    fn eq(&self, other: &Self) -> bool {
        match &self.target_name {
            //Reminder; we *expect* entries to be diferrent.
            Name::Blank(i) => match &other.target_name {
                Name::Blank(j) => i == j,
                Name::Long(_) => false,
                Name::Short(_) => false,
                _ => panic!("Invalid entry!"),
            },
            Name::Long(s) => match &other.target_name {
                Name::Blank(_) => false,
                Name::Long(t) => s == t,
                Name::Short(_) => false,
                _ => panic!("Invalid entry!"),
            },
            Name::Short(c) => match &other.target_name {
                Name::Blank(_) => false,
                Name::Long(_) => false,
                Name::Short(d) => c == d,
                _ => panic!("Invalid entry!"),
            },
            _ => panic!("Invalid entry!"),
        }
    }
}
impl PartialOrd for Entry {
    /// Conventional order: command <blanks> -<shorts> --<longs>. Why? good question.
    fn partial_cmp(&self, other: &Self) -> Option<std::cmp::Ordering> {
        match &self.target_name {
            //Reminder; we *expect* entries to be diferrent.
            Name::Blank(i) => match &other.target_name {
                Name::Blank(j) => i.partial_cmp(j),
                Name::Long(_) => Some(std::cmp::Ordering::Less),
                Name::Short(_) => Some(std::cmp::Ordering::Less),
                _ => panic!("Invalid entry!"),
            },
            Name::Long(s) => match &other.target_name {
                Name::Blank(_) => Some(std::cmp::Ordering::Greater),
                Name::Long(t) => s.partial_cmp(t),
                Name::Short(_) => Some(std::cmp::Ordering::Greater),
                _ => panic!("Invalid entry!"),
            },
            Name::Short(c) => match &other.target_name {
                Name::Blank(_) => Some(std::cmp::Ordering::Greater),
                Name::Long(_) => Some(std::cmp::Ordering::Less),
                Name::Short(d) => c.partial_cmp(d),
                _ => panic!("Invalid entry!"),
            },
            _ => panic!("Invalid entry!"),
        }
    }
}

impl TryFrom<&str> for Entry {
    type Error = Error;
    /// Formatter for args, defined as:
    ///   With d the default flag, d ∈ {λ,!,<?*>}
    ///     Where: 
    ///       - λ designs an empty string,
    ///       - ! the ! symbol, 
    ///       - and <?*> any string, surrounded by the symbols < and >.
    ///     Corresponds to:
    ///       - λ => DefaultValue::Skip
    ///       - ! => DefaultValue::Mandatory
    ///       - <?*> => DefaultValue::Default(?*)
    ///
    ///   With n the target name, n ∈ {#?i, -?, --?*}
    ///     Where: 
    ///       - #?i designs the # symbol followed by any number, 
    ///       - -? the - symbol followed by any character, 
    ///       - and --?* the -- symbol followed by any string.
    ///     Corresponds to:
    ///       - #?i => Name::Blank(?i)
    ///       - -? => Name::Short(?)
    ///       - --?* => Name::Long(?*)
    ///   With t the target type (or kind), t ∈ {λ} ∪ [.S.] and S = {str}
    ///     λ corresponds to a boolean flag
    ///     Corresponds to:
    ///      - str => Option<String>
    ///      - path => Option<PathBuff>
    /// 
    ///   With s = {_} where _ is any string.
    ///     Corresponds to the source item, as read by clap.
    /// 
    /// A format as: ndst
    ///   
    /// Examples: #1<->{path}[path]
    ///           -i{casei}
    ///           #0!{pattern}[str]
    fn try_from(value: &str) -> Result<Self, Self::Error> {
        let r = Regex::new(r"((?P<n1>#\d{1,3})|(?P<n2>-\pL)|(?P<n3>--\pL+))((?P<d1>!)|(?P<d2><[\pL-]*>)|(?P<d3>))(?P<s>\{\pL+\})((?P<t1>\[\pL+\])|(?P<t2>))").unwrap();
        let c = r.captures(value);



        return Err(Error{});
    }
}

pub trait Conformable<U> {
    /// Takes a full entry, calls generate, then makes the defaults_to checks. Obviously may fail
    fn conform(&self) -> String;
    /// Takes a full entry, and runs the string transform, then prepends the name.
    fn generate(&self, with: U) -> Option<String>;
    /// Proceeds through the full transform, filling the entry, then calling conform.
    ///   Processes collection arguments separately(?).
    fn transform(&self) -> Vec<String>;
}
impl Conformable<Argument> for Entry {
    fn conform(&self) -> String {
        todo!()
        //return self.target_name
    }
    fn generate(&self, with: Argument) -> Option<String> {
        
        todo!()
    }
    fn transform(&self) -> Vec<String> {
        todo!()
    }
}

pub trait Convertible<T> {
    /// Takes clap data, and converts it to a command string.
    fn generate(&self, with: T) -> Vec<String>;
}