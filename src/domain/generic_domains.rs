use std::collections::BTreeSet;

#[derive(Ord, PartialEq, PartialOrd, Eq, Clone, Copy)]
pub struct Voter(pub String);

#[derive(Ord, PartialEq, PartialOrd, Eq)]
pub struct Candidate(pub String);

#[derive(Ord, PartialEq, PartialOrd, Eq)]
pub struct Score(pub usize);

pub struct AttendenceSheet(pub BTreeSet<Voter>);
