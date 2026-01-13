use regex::Regex;

#[derive(Debug, Clone)]
pub struct HashInfo {
    pub name: &'static str,
    pub hashcat: Option<i32>,
    pub john: Option<&'static str>,
    pub extended: bool,
    pub description: Option<&'static str>,
}

pub struct Prototype {
    pub regex: Regex,
    pub modes: Vec<HashInfo>,
}
