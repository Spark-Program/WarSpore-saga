use alloc::{
    collections::btree_map::BTreeMap,
    format,
    string::{String, ToString},
    vec,
    vec::Vec,
};
use ckb_cinnabar_verifier::{re_exports::ckb_std, Result};
use ckb_std::env::Arg;
use common::{
    card::Card,
    hardcoded::{DNA, DNA_LEN},
};
use serde::Serialize;
use serde_json::Value;

use crate::ScriptError;

// note: args[0] is pattern, which can be ignored here
pub fn dobs_parse_parameters(args: &[Arg]) -> Result<Vec<u8>> {
    if args.len() < 2 {
        return Err(ScriptError::InvalidArgCount.into());
    }
    let dna = hex::decode(args[0].to_bytes()).map_err(|_| ScriptError::BadHexFormat)?;
    if dna.len() < DNA_LEN {
        return Err(ScriptError::InsufficientDnaLength.into());
    }
    Ok(dna)
}

// parse dna to rendered output
pub fn dob0_decode(card: &Card) -> Vec<StandardDOBOutput> {
    vec![
        StandardDOBOutput {
            name: "name".to_string(),
            traits: vec![ParsedTrait::new("String", card.name.to_string().into())],
        },
        StandardDOBOutput {
            name: "cost".to_string(),
            traits: vec![ParsedTrait::new("Number", card.cost.into())],
        },
        StandardDOBOutput {
            name: "type".to_string(),
            traits: vec![ParsedTrait::new("String", card.category.to_string().into())],
        },
        StandardDOBOutput {
            name: "detail".to_string(),
            traits: vec![ParsedTrait::new("String", card.description.clone().into())],
        },
        StandardDOBOutput {
            name: "golden".to_string(),
            traits: vec![ParsedTrait::new("Boolean", card.golden.into())],
        },
    ]
}

pub fn dob1_decode(dna: &DNA) -> Vec<StandardDOBOutput> {
    vec![
        StandardDOBOutput {
            name: "prev.type".to_string(),
            traits: vec![ParsedTrait::new("String", "image".into())],
        },
        StandardDOBOutput {
            name: "prev.bg".to_string(),
            traits: vec![ParsedTrait::new(
                "String",
                format!(
                    "https://render.warspore-saga.xyz/render?dna={}",
                    hex::encode(dna)
                )
                .into(),
            )],
        },
    ]
}

pub fn blindbox_decode() -> Vec<StandardDOBOutput> {
    vec![
        StandardDOBOutput {
            name: "prev.type".to_string(),
            traits: vec![ParsedTrait::new("String", "image".into())],
        },
        StandardDOBOutput {
            name: "prev.bg".to_string(),
            traits: vec![ParsedTrait::new(
                "String",
                "https://render.warspore-saga.xyz/icon/blindbox".into(),
            )],
        },
    ]
}

#[derive(Serialize)]
pub struct ParsedTrait {
    #[serde(flatten)]
    traits: BTreeMap<String, Value>,
}

impl ParsedTrait {
    pub fn new(key: &str, value: Value) -> Self {
        let mut traits = BTreeMap::new();
        traits.insert(key.to_string(), value);
        Self { traits }
    }
}

#[derive(Serialize)]
pub struct StandardDOBOutput {
    name: String,
    traits: Vec<ParsedTrait>,
}
