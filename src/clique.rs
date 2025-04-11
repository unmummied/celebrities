pub mod person;

use person::Person;
use petgraph::graph::DiGraph;
use std::{
    collections::{HashMap, HashSet},
    hash::Hash,
};

pub trait Clique: Sized {
    /// A clique is a non-empty set in which every members know each other.
    fn is_clique(&self) -> bool;

    /// A celebrity clique `C` is a non-empty set in which everybody at the party knows every member of `C`, but members of `C` know only each other.
    ///
    /// Theorem:
    /// All celebrity clique is a clique.
    fn is_cclique(&self, party: &Self) -> bool;

    /// Exhaustive search of cclique.
    ///
    /// Theorem:
    /// There is at most one non-empty celebrity clique.
    ///
    /// Proof:
    /// Suppose that `C1` and `C2` are two celebrity cliques.
    /// Pick any `c1` in `C1` and `c2` in `C2`.
    /// We have that `c1` knows `c2` from the fact that everybody in the clique `C2` is known by everybody at the party.
    /// But since clique members know only other members of the clique, it follows that `c2` in `C1`.
    /// Since `c2` was arbitrary, we have `C2` is a subset of `C1` and, by symmetry, `C1` is a subset of `C2`.
    fn cclique(&self) -> Option<Self>;
}

impl Clique for HashSet<Person> {
    /// A clique is a set of nodes in which each pair of nodes has an arc in both directions between them.
    fn is_clique(&self) -> bool {
        let clique = self.iter().map(|member| member.id).collect::<HashSet<_>>();
        self.iter().all(|member| {
            clique
                .difference(&member.known_people)
                .filter(|rem| **rem != member.id)
                .count()
                == 0
        })
    }

    fn is_cclique(&self, party: &Self) -> bool {
        for someone in party {
            for celebrity in self {
                if !someone.knows(celebrity)
                    || (celebrity.knows(someone) && !self.contains(someone))
                {
                    return false;
                }
            }
        }
        true
    }

    fn cclique(&self) -> Option<Self> {
        power_set(self)
            .iter()
            .skip(1)
            .find(|&people| people.is_cclique(self))
            .cloned()
    }
}

fn power_set<T: Clone + Eq + Hash>(set: &HashSet<T>) -> Vec<HashSet<T>> {
    let mut levels = Vec::with_capacity(set.len() + 1);
    levels.push(vec![HashSet::default()]);
    for k in 1..=set.len() {
        levels.push(Vec::with_capacity(binomial_approx(set.len(), k)));
    }

    for elem in set {
        for cap in (0..set.len()).rev() {
            for subset in &levels[cap].clone() {
                let mut temp = subset.clone();
                temp.insert(elem.clone());
                levels[cap + 1].push(temp);
            }
        }
    }
    levels.into_iter().flatten().collect()
}

fn binomial_approx(n: usize, k: usize) -> usize {
    if n < k {
        return 0;
    }
    if k == 0 || k == n {
        return 1;
    }
    (0..k).fold(1, |acc, v| acc * (n - v) / (v + 1))
    // equivalane to (0..=n).rev().zip(1..=k).fold(1, |mut acc, (num, denom)| {acc *= num; acc /= denom; acc})
}

pub fn clique2digraph(clique: &HashSet<Person>) -> DiGraph<usize, ()> {
    let mut graph = DiGraph::new();

    let mut nodes: HashMap<usize, _> = HashMap::with_capacity(clique.len());
    for person in clique {
        nodes.insert(person.id, graph.add_node(person.id));
    }

    for person in clique {
        for known_person_id in person.known_people.clone() {
            graph.add_edge(
                match nodes.get(&person.id) {
                    None => continue,
                    Some(node) => *node,
                },
                match nodes.get(&known_person_id) {
                    None => continue,
                    Some(node) => *node,
                },
                (),
            );
        }
    }

    graph
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    #[allow(clippy::iter_on_single_items)]
    fn test_power_set() {
        println!("{:?}", power_set(&[0; 0].into_iter().collect()).len());
        println!("{:?}", power_set(&[1].into_iter().collect()).len());
        println!("{:?}", power_set(&[1, 2, 3].into_iter().collect()).len());
        println!(
            "{:?}",
            power_set(&[1, 2, 3, 4, 5].into_iter().collect()).len()
        );
    }

    #[test]
    fn test_binomial_approx() {
        for n in 1..=3 {
            for k in 1..=3 {
                println!("({n}, {k}): {}", binomial_approx(n, k));
            }
        }
    }
}
