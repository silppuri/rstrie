#[macro_use]
extern crate helix;
extern crate sequence_trie;

use sequence_trie::SequenceTrie;
use helix::sys::{VALUE};

fn get_prefix_children(trie: SequenceTrie<u8, VALUE>, key: String) -> Vec<String> {
    let prefix = key.as_str();
    let node = trie.get_node(prefix.as_bytes());
    let values: Vec<String> = node.map_or(vec![], |node| {
        node.keys().map(|v| {
            let mut string_key = String::with_capacity(prefix.len() + v.len());
            string_key.push_str(prefix);

            let byte_vec: Vec<u8> = v.into_iter().map(|b| *b).collect();
            string_key.push_str(std::str::from_utf8(&byte_vec).unwrap());

            string_key
        }).collect()
    });
    values
}

ruby! {
    pub class Rstrie {
        struct {
            trie: SequenceTrie<u8,VALUE>
        }

        def initialize(helix) {
            Rstrie { helix, trie: SequenceTrie::<u8, VALUE>::new() }
        }
        def insert(&mut self, key: String, value: VALUE) -> bool {
            match self.trie.insert(&key.into_bytes(), value) {
                None => true,
                Some(_) => false
            }
        }
        def get(&self, key: String) -> Option<VALUE> {
           match self.trie.get(&key.into_bytes()) {
               None => None,
               Some(i) => Some(*i)
           }
        }

        def has_key(&self, key: String) -> bool {
           match self.trie.get(&key.into_bytes()) {
               None => false,
               Some(_) => true
           }
        }

        def delete(&mut self, key: String) {
            self.trie.remove(&key.into_bytes())
        }

        def children(&self, key: Option<String>) -> Vec<String> {
            match key {
                None => vec![],
                Some(i) => get_prefix_children(self.trie.clone(), i)
            }
        }
    }
}
