#[macro_use]
extern crate helix;
extern crate sequence_trie;

use sequence_trie::SequenceTrie;

ruby! {
    pub class Rstrie {
        struct {
            trie: SequenceTrie<String,bool>
        }

        def initialize(helix) {
            Rstrie { helix, trie: SequenceTrie::<String, bool>::new() }
        }
        def insert(value: String) -> String {
            println!("LOG: {:?}", value);
            value
        }
    }
}
