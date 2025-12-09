#![no_main]
#![no_std]

use alloc::vec::Vec;
use ckb_cinnabar_verifier::{
    cinnabar_main, define_errors, re_exports::ckb_std, Result, Verification, CUSTOM_ERROR_START,
    TREE_ROOT,
};
use common::{
    card::roulette_card,
    hardcoded::{DNA, DNA_LEN},
};

mod decoder;

define_errors!(
    ScriptError,
    {
        InvalidArgCount = CUSTOM_ERROR_START,
        BadHexFormat,
        InsufficientDnaLength,
        InternalConfigError,
    }
);

#[derive(Default)]
struct Context {}

#[derive(Default)]
struct Root {}

impl Verification<Context> for Root {
    fn verify(&mut self, _name: &str, _: &mut Context) -> Result<Option<&str>> {
        let dna: DNA = decoder::dobs_parse_parameters(ckb_std::env::argv())?
            .try_into()
            .unwrap();
        let mut output = Vec::new();
        if dna != [0; DNA_LEN] {
            // Card
            let card =
                roulette_card(database::CARD_POOL, &dna).ok_or(ScriptError::InternalConfigError)?;
            output.extend(decoder::dob0_decode(&card));
            output.extend(decoder::dob1_decode(&dna));
        } else {
            // Blindbox
            output.extend(decoder::blindbox_decode());
        }
        let rendered_output = serde_json::to_string(&output).expect("serialize output");
        ckb_std::debug!("{rendered_output}");
        Ok(None)
    }
}

cinnabar_main!(Context, (TREE_ROOT, Root));
