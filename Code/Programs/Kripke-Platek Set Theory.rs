use std::collections::{HashMap, HashSet};

#[derive(Debug, PartialEq, Eq, Hash, Clone)]
enum VonNeumann {
    Set(HashSet<VonNeumann>),
    Successor(Box<VonNeumann>),
}

// A structure to represent a model in Kripke-Platek set theory
#[derive(Debug)]
struct KripkePlatekModel {
    universe: HashSet<VonNeumann>,
    admissible_sets: HashSet<HashSet<VonNeumann>>,
}

impl KripkePlaket {
    fn new() -> Self {
        KripkePlatekModel {
            universe: HashSet::new(),
            admissible_sets: HashSet::new(),
        }     
    } 

    // Axiom of extensionality: For all sets x and y, x = y if and only if for all z, z is in x if and only if z is in y.
    fn axiom_of_extensionality(&self, set1: &HashSet<VonNeumann>, set2: &HashSet<VonNeumann>) -> bool {
        set1 == set2
    }

    // Axiom of empty set: There exists a set with no elements.
    fn axiom_of_empty_set(&self) -> HashSet<VonNeumann> {
        HashSet::new()
    }

    // Axiom of pairing: For any sets x and y, there exists a set {x, y}.
    fn axiom_of_pairing(&self, set1: &HashSet<VonNeumann>, set2: &HashSet<VonNeumann>) -> HashSet<VonNeumann> {
        let mut result = HashSet::new();
        result.insert(VonNeumann::Set(set1.clone()));
        result.insert(VonNeumann::Set(set2.clone()));
        result    
    }

    // Axiom of union: For any set x, there exists a set y such that for any z, z is in y if and only if there exists a set w in x such that z is in w.
    fn axiom_of_union(&self, set: &HashSet<VonNeumann>) -> HashSet<VonNeumann> {
        
    }
}
