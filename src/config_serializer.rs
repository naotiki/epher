use std::collections::{BTreeSet, HashMap};
use toml_edit::{Array, DocumentMut, InlineTable, Item, KeyMut, Table, Value};
use toml_edit::visit::{Visit, visit_table_like_kv};
use toml_edit::visit_mut::{visit_table_like_kv_mut, visit_table_mut, VisitMut};
use crate::config::{Config, Packages};

#[derive(Copy, Clone, Debug, Eq, PartialEq)]
enum VisitState {
    /// Represents the root of the table.
    Root,
    Packages,
    Other,
}

impl VisitState {
    /// Figures out the next visit state, given the current state and the given key.
    fn descend(self, key: &str) -> Self {
        match (self, key) {
            (
                VisitState::Root,
                "packages"
            ) => VisitState::Packages,
            (VisitState::Root | VisitState::Other, _) => VisitState::Other,
            (VisitState::Packages, _) => VisitState::Other
        }
    }
}

/// Collect the names of every dependency key.
#[derive(Debug)]
struct DependencyNameVisitor<'doc> {
    state: VisitState,
    names: BTreeSet<&'doc str>,
}

impl<'doc> Visit<'doc> for DependencyNameVisitor<'doc> {
    fn visit_table_like_kv(&mut self, key: &'doc str, node: &'doc Item) {
        if self.state == VisitState::Packages {
            self.names.insert(key);
        } else {
            // Since we're only interested in collecting the top-level keys right under
            // [dependencies], don't recurse unconditionally.

            let old_state = self.state;

            // Figure out the next state given the key.
            self.state = self.state.descend(key);

            // Recurse further into the document tree.
            visit_table_like_kv(self, key, node);

            // Restore the old state after it's done.
            self.state = old_state;
        }
    }
}

#[derive(Debug)]
struct NormalizeDependencyTablesVisitor {
    state: VisitState,
}

impl VisitMut for NormalizeDependencyTablesVisitor {
    fn visit_table_mut(&mut self, node: &mut Table) {
        visit_table_mut(self, node);
    }

    fn visit_table_like_kv_mut(&mut self, mut key: KeyMut<'_>, node: &mut Item) {
        let old_state = self.state;

        // Figure out the next state given the key.
        self.state = self.state.descend(key.get());

        match self.state {
             VisitState::Packages => {
                // Top-level dependency row, or above: turn inline tables into regular ones.
                if let Item::Value(Value::InlineTable(inline_table)) = node {
                    let inline_table = std::mem::replace(inline_table, InlineTable::new());
                    let table = inline_table.into_table();
                    key.fmt();
                    *node = Item::Table(table);
                }
            }
            _ => {}
        }

        // Recurse further into the document tree.
        visit_table_like_kv_mut(self, key, node);

        // Restore the old state after it's done.
        self.state = old_state;
    }

    fn visit_array_mut(&mut self, node: &mut Array) {
        // Format any arrays within dependencies to be on the same line.
        if matches!(
            self.state,
            VisitState::Packages
        ) {
            node.fmt();
        }
    }
}

pub fn serialize_config(conf: &Config) -> String{
    let mut document: DocumentMut = toml_edit::ser::to_document(&conf).unwrap();
    visit_document_mut(&mut document);
    document.to_string()
}
fn visit_document_mut(document: &mut DocumentMut) {
    let mut visitor = NormalizeDependencyTablesVisitor {
        state: VisitState::Root,
    };
    visitor.visit_document_mut(document);
}

#[cfg(test)]
#[test]
fn visit_mut_correct() {
    let mut map = HashMap::new();
    map.insert("git".to_string(),Packages{
        url: "https://aaaa".to_string()
    });
    map.insert("gh".to_string(),Packages{
        url: "https://gh".to_string()
    });
    let config = Config {
        ip: "127.0.0.1".to_string(),
        packages:map
    };
    let mut document: DocumentMut = toml_edit::ser::to_document(&config).unwrap();

    visit_document_mut(&mut document);
    println!("{}", document);
}