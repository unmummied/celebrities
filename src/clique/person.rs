use std::{
    collections::HashSet,
    convert::Into,
    fmt::{Display, Error, Formatter},
    hash::{Hash, Hasher},
};

#[derive(Debug, Clone)]
pub struct Person {
    pub id: usize,
    pub known_people: HashSet<usize>,
}

impl Person {
    pub fn knows(&self, other: &Self) -> bool {
        self == other // x `knows` x, for all x.
            || self.known_people.contains(&other.id)
    }
}

impl<N, V> From<(N, V)> for Person
where
    V: IntoIterator<Item = N>,
    N: Into<usize>,
{
    fn from((id, knows): (N, V)) -> Self {
        let id = id.into();
        Self {
            id,
            known_people: knows
                .into_iter()
                .map(Into::into)
                .filter(|people_id| *people_id != id) // remove myself.
                .collect(),
        }
    }
}

impl PartialEq for Person {
    fn eq(&self, other: &Self) -> bool {
        self.id.eq(&other.id)
    }
}
impl Eq for Person {}

impl Hash for Person {
    fn hash<H: Hasher>(&self, state: &mut H) {
        self.id.hash(state);
    }
}

impl Display for Person {
    fn fmt(&self, f: &mut Formatter) -> Result<(), Error> {
        write!(f, "id: {} knows {:?}", self.id, self.known_people)
    }
}
