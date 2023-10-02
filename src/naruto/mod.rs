mod model;

use std::collections::HashMap;

use async_graphql::{EmptyMutation, EmptySubscription, Schema};
pub use model::QueryRoot;
use model::{Element, Village};
use slab::Slab;
pub type NarutoShippudenSchema = Schema<QueryRoot, EmptyMutation, EmptySubscription>;

pub struct NarutoChar {
    id: &'static str,
    name: &'static str,
    friends: Vec<usize>,
    village: Village,
    elements: Vec<Element>,
}

pub struct NarutoShippuden {
    naruto: usize,
    gara: usize,
    pain: usize,
    chars: Slab<NarutoChar>,
    chars_by_id: HashMap<&'static str, usize>,
}

impl NarutoShippuden {
    #[allow(clippy::new_without_default)]
    pub fn new() -> Self {
        let mut chars = Slab::new();

        let naruto = chars.insert(NarutoChar {
            id: "1000",
            name: "Naruto Uzumaki",
            friends: vec![],
            village: Village::Konoha,
            elements: vec![Element::Wind],
        });

        let sasuke = chars.insert(NarutoChar {
            id: "1001",
            name: "Sasuke Uchiha",
            friends: vec![],
            village: Village::Konoha,
            elements: vec![Element::Lightning],
        });

        let sakura = chars.insert(NarutoChar {
            id: "1002",
            name: "Sakura Haruno",
            friends: vec![],
            village: Village::Konoha,
            elements: vec![Element::Earth],
        });

        let kakashi = chars.insert(NarutoChar {
            id: "1003",
            name: "Kakashi Hatake",
            friends: vec![],
            village: Village::Konoha,
            elements: vec![Element::Lightning],
        });

        let gara = chars.insert(NarutoChar {
            id: "1004",
            name: "Gara",
            friends: vec![],
            village: Village::Suna,
            elements: vec![Element::Wind, Element::Earth],
        });

        let pain = chars.insert(NarutoChar {
            id: "1005",
            name: "Pain",
            friends: vec![],
            village: Village::Ame,
            elements: vec![Element::Wind, Element::Water],
        });

        chars[naruto].friends = vec![sasuke, sakura, kakashi];
        chars[sasuke].friends = vec![naruto, sakura, kakashi];
        chars[sakura].friends = vec![naruto, sasuke, kakashi];
        chars[kakashi].friends = vec![naruto, sasuke, sakura];
        chars[gara].friends = vec![naruto];
        chars[pain].friends = vec![naruto];

        let chars_by_id = chars.iter().map(|(idx, ch)| (ch.id, idx)).collect();
        Self {
            naruto,
            gara,
            pain,
            chars,
            chars_by_id,
        }
    }

    pub fn human(&self, id: &str) -> Option<&NarutoChar> {
        self.chars_by_id
            .get(id)
            .copied()
            .map(|idx| self.chars.get(idx).unwrap())
    }

    pub fn humans(&self) -> Vec<&NarutoChar> {
        self.chars.iter().map(|(_, ch)| ch).collect()
    }

    pub fn friends(&self, ch: &NarutoChar) -> Vec<&NarutoChar> {
        ch.friends
            .iter()
            .copied()
            .filter_map(|id| self.chars.get(id))
            .collect()
    }
}
