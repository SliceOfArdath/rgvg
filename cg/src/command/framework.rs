use std::collections::{BTreeSet};
use std::path::PathBuf;
use std::fmt::{self, Display};
use regex::Regex;

type Index = u8;

#[derive(Clone)]
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
impl Name {
    fn name(&self, v: Vec<String>) -> Vec<String> {
        match self {
            Name::Blank(_) => v,
            _ => panic!("Unsupported conversion!"),
        }
    }
}

#[derive(Debug, Clone)]
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

}

trait Transform<T> {
    fn transform(&mut self, value: &T);
}
trait Generate {
    /// Vec and not option, because of collection arguments.
    fn generate(self) -> Vec<String>;
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
impl From<&'static str> for Argument {
    fn from(value: &'static str) -> Self {
        return Argument::Text(Some(value.to_string()));
    }
}

impl Transform<String> for Argument {
    fn transform(&mut self, value: &String) {
        match self {
            Argument::Text(x) => {*x = Some(value.to_string())},
            _ => panic!("Unspported transformation!")
        }
    }
}
impl Transform<PathBuf> for Argument {
    fn transform(&mut self, value: &PathBuf) {
        match self {
            Argument::PathPattern(x) => {*x = Some(value.to_path_buf())},
            Argument::Text(x) => {*x = Some(value.display().to_string())},
            _ => panic!("Unspported transformation!")
        }
    }
}
impl Transform<bool> for Argument {
    fn transform(&mut self, value: &bool) {
        match self {
            Argument::BooleanFlag(x) => {*x = Some(*value)},
            Argument::Text(x) => {*x = Some(value.to_string())},
            _ => panic!("Unspported transformation!")
        }
    }
}
impl Transform<Option<PathBuf>> for Argument {
    fn transform(&mut self, value: &Option<PathBuf>) {
        match self {
            Argument::PathPattern(x) => match value {
                Some(p) => {*x = Some(p.to_path_buf())},
                None => {*x = None},
            },
            Argument::Text(x) => match value {
                Some(p) => {*x = Some(p.display().to_string())},
                None => {*x = None},
            },
            _ => panic!("Unspported transformation!")
        }
    }
}


impl Generate for Argument {
    fn generate(self) -> Vec<String> {
        match self {
            Argument::BooleanFlag(x) => optional_vectorization(x),
            Argument::Text(x) => optional_vectorization(x), //that's a string!
            Argument::PathPattern(x) => optional_vectorization(x),
            _ => panic!("Unsupported type: {:?}", self),
        }
    }
}

trait Vectorize
    where Self: Sized {
    fn vec(self) -> Vec<String>;
}
fn optional_vectorization<T: Vectorize>(v: Option<T>) -> Vec<String> {
    match v {
        Some(n) => n.vec(),
        None => Vec::new(),
    }
}
impl<T> Vectorize for Vec<T> 
    where T: Vectorize {
        fn vec(self) -> Vec<String> {
            let mut r: Vec<String> = Vec::new();
            for c in self {
                r.extend(c.vec());
            }
            return r;
        }
    }

impl Vectorize for String {
    fn vec(self) -> Vec<String> {
        return vec![self];
    }
}
impl Vectorize for bool {
    fn vec(self) -> Vec<String> {
        return vec![self.to_string()];
    }
}
impl Vectorize for PathBuf {
    fn vec(self) -> Vec<String> {
        return vec![self.display().to_string()];
    }
}

#[derive(Clone)]
pub enum SourceFormatter {
    /// No formatting
    Default,
    /// Filter in only certain elements, placeholder for now
    Filter,
}
#[derive(Clone)]
pub enum DefaultValue {
    /// CANNOT be ommited
    Mandatory, 
    /// Just forgedaboutit
    Skip,
    /// provide a default. This default is constant! may provide a formatter later lol
    Default(Argument),
}

#[derive(Clone)]
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

pub trait Transformable<U> {
    /// Takes an empty entry and fills it.
    fn fill(&mut self, with: &U);
    /// Makes conformity checks.
    fn conform(self) -> Vec<String>;
    /// Generates the entry.
    fn generate(self) -> Vec<String>;
    /// The full transform
    fn transform(self, with: &U) -> Vec<String>;
}
impl<U> Transformable<U> for Entry
where Argument: Transform<U> {
    fn fill(&mut self, with: &U) {
        self.target_type.transform(with);
    }
    fn generate(self) -> Vec<String> {
        return self.target_type.generate();
    }
    fn conform(self) -> Vec<String> {
        let mut replacement = self.clone();
        match &replacement.defaults_to {
            DefaultValue::Skip => return self.generate(),
            DefaultValue::Mandatory => {
                let e = self.generate();
                if e.len() == 0 {
                    panic!("Mandatory argument was not filled");
                } else {
                    return e;
                }
            }
            DefaultValue::Default(x) => {
                let e = self.generate();
                if e.len() == 0 {
                    replacement.target_type = x.clone();
                    return replacement.generate();
                } else {
                    return e;
                }
            }
        }
    }
    fn transform(mut self, with: &U) -> Vec<String> {
        self.fill(with);

        let n = self.target_name.clone();
        let q = self.conform();
        return n.name(q);
    }
}

pub trait Convertible<T> {
    /// Polulate entry with clap data, returns the ordered entry bundle
    fn populate(&self, with: T) -> BTreeSet<Entry>;
    /// Takes clap data, and converts it to a command string.
    fn generate(&self, with: T) -> Vec<String>;
}