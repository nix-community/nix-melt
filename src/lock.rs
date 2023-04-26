use std::hash::BuildHasherDefault;

use eyre::{eyre, Result};
use indexmap::IndexMap;
use parse_display::Display;
use rustc_hash::FxHasher;
use serde::Deserialize;
use serde_with::{serde_as, Map};

pub(crate) struct Resolve {
    pub root: Node,
    pub nodes: IndexMap<String, Node, BuildHasherDefault<FxHasher>>,
}

#[derive(Deserialize)]
pub(crate) struct Lock {
    pub root: String,
    pub nodes: IndexMap<String, Node, BuildHasherDefault<FxHasher>>,
}

#[derive(Deserialize)]
pub(crate) struct Node {
    #[serde(default)]
    pub inputs: IndexMap<String, Input, BuildHasherDefault<FxHasher>>,
    pub locked: Option<Locked>,
}

#[derive(Clone, Debug, Deserialize, PartialEq)]
#[serde(untagged)]
pub(crate) enum Input {
    Direct(String),
    Follow(Vec<String>),
}

#[serde_as]
#[derive(Deserialize)]
#[serde(rename_all = "camelCase")]
pub(crate) struct Locked {
    #[serde(rename = "type")]
    pub type_: String,
    pub last_modified: usize,
    #[serde(flatten)]
    #[serde_as(as = "Map<_, _>")]
    pub fields: Vec<(String, Value)>,
}

#[derive(Deserialize, Display)]
#[serde(untagged)]
#[display("{0}")]
pub enum Value {
    String(String),
    Bool(bool),
    Int(i64),
}

impl Resolve {
    pub(crate) fn get(&self, input: &Input) -> Option<&Node> {
        match input {
            Input::Direct(x) => self.nodes.get(x),
            Input::Follow(xs) => {
                let mut node = &self.root;
                for x in xs {
                    node = self.get(node.inputs.get(x)?)?;
                }
                Some(node)
            }
        }
    }
}

impl Lock {
    pub(crate) fn resolve(mut self) -> Result<Resolve> {
        Ok(Resolve {
            root: self
                .nodes
                .remove(&self.root)
                .ok_or_else(|| eyre!("no root node"))?,
            nodes: self.nodes,
        })
    }
}
