use crate::model::Episode;
use slab::Slab;
use std::collections::HashMap;

pub struct StarWarsChar {
    pub id: &'static str,
    pub name: &'static str,
    pub is_human: bool,
    pub friends: Vec<usize>,
    pub appears_in: Vec<Episode>,
    pub home_planet: Option<&'static str>,
    pub primary_function: Option<&'static str>,
}

pub struct StarWars {
    pub luke: usize,
    pub artoo: usize,
    pub chars: Slab<StarWarsChar>,
    pub chars_by_id: HashMap<&'static str, usize>,
}

impl StarWars {
    #[allow(clippy::new_without_default)]
    pub fn new() -> Self {
        let mut chars = Slab::new();

        let luke = chars.insert(StarWarsChar {
            id: "1000",
            name: "Luke Skywalker",
            is_human: true,
            friends: vec![],
            appears_in: vec![],
            home_planet: Some("Tatooine"),
            primary_function: None,
        });

        let vader = chars.insert(StarWarsChar {
            id: "1001",
            name: "Anakin Skywalker",
            is_human: true,
            friends: vec![],
            appears_in: vec![],
            home_planet: Some("Tatooine"),
            primary_function: None,
        });

        let han = chars.insert(StarWarsChar {
            id: "1002",
            name: "Han Solo",
            is_human: true,
            friends: vec![],
            appears_in: vec![Episode::Empire, Episode::NewHope, Episode::Jedi],
            home_planet: None,
            primary_function: None,
        });

        let leia = chars.insert(StarWarsChar {
            id: "1003",
            name: "Leia Organa",
            is_human: true,
            friends: vec![],
            appears_in: vec![Episode::Empire, Episode::NewHope, Episode::Jedi],
            home_planet: Some("Alderaa"),
            primary_function: None,
        });

        let tarkin = chars.insert(StarWarsChar {
            id: "1004",
            name: "Wilhuff Tarkin",
            is_human: true,
            friends: vec![],
            appears_in: vec![Episode::Empire, Episode::NewHope, Episode::Jedi],
            home_planet: None,
            primary_function: None,
        });

        let threepio = chars.insert(StarWarsChar {
            id: "2000",
            name: "C-3PO",
            is_human: false,
            friends: vec![],
            appears_in: vec![Episode::Empire, Episode::NewHope, Episode::Jedi],
            home_planet: None,
            primary_function: Some("Protocol"),
        });

        let artoo = chars.insert(StarWarsChar {
            id: "2001",
            name: "R2-D2",
            is_human: false,
            friends: vec![],
            appears_in: vec![Episode::Empire, Episode::NewHope, Episode::Jedi],
            home_planet: None,
            primary_function: Some("Astromech"),
        });

        chars[luke].friends = vec![han, leia, threepio, artoo];
        chars[vader].friends = vec![tarkin];
        chars[han].friends = vec![luke, leia, artoo];
        chars[leia].friends = vec![luke, han, threepio, artoo];
        chars[tarkin].friends = vec![vader];
        chars[threepio].friends = vec![luke, han, leia, artoo];
        chars[artoo].friends = vec![luke, han, leia];

        let chars_by_id = chars.iter().map(|(idx, ch)| (ch.id, idx)).collect();
        Self {
            luke,
            artoo,
            chars,
            chars_by_id,
        }
    }

    pub fn human(&self, id: &str) -> Option<&StarWarsChar> {
        self.chars_by_id
            .get(id)
            .copied()
            .map(|idx| self.chars.get(idx).unwrap())
            .filter(|ch| ch.is_human)
    }

    pub fn droid(&self, id: &str) -> Option<&StarWarsChar> {
        self.chars_by_id
            .get(id)
            .copied()
            .map(|idx| self.chars.get(idx).unwrap())
            .filter(|ch| !ch.is_human)
    }

    pub fn humans(&self) -> Vec<&StarWarsChar> {
        self.chars
            .iter()
            .filter(|(_, ch)| ch.is_human)
            .map(|(_, ch)| ch)
            .collect()
    }

    pub fn droids(&self) -> Vec<&StarWarsChar> {
        self.chars
            .iter()
            .filter(|(_, ch)| !ch.is_human)
            .map(|(_, ch)| ch)
            .collect()
    }

    pub fn friends(&self, ch: &StarWarsChar) -> Vec<&StarWarsChar> {
        ch.friends
            .iter()
            .copied()
            .filter_map(|id| self.chars.get(id))
            .collect()
    }
}
