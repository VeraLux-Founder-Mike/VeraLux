use std::collections::HashSet;

use anchor_lang::prelude::*;

use crate::{ContractState, Multisig, VeraluxErrorCode};

pub struct ReentrancyGuard<'a> {
    pub state: &'a mut ContractState,
}

impl<'a> ReentrancyGuard<'a> {
    pub fn new(state: &'a mut ContractState) -> Result<Self> {
        require!(
            !state.is_processing,
            VeraluxErrorCode::ReentrancyGuardTriggered
        );
        state.is_processing = true;
        Ok(Self { state })
    }
}

pub fn validate_multisig(multisig: &Multisig, signers: &[&Option<&Signer>]) -> Result<()> {
    let mut unique_signers = HashSet::new();
    msg!("signers: {:?}", signers.len());
    for signer in signers.iter().filter_map(|s| s.as_ref()) {
        unique_signers.insert(signer.key());
    }
    require!(
        unique_signers.len() >= multisig.threshold as usize,
        VeraluxErrorCode::InsufficientSigners
    );

    msg!(
        "multisig: {:?} {:?}",
        multisig.owners.len(),
        multisig.owners
    );
    for signer_key_from_unique_set in &unique_signers {
        require!(
            multisig.owners.contains(signer_key_from_unique_set) || multisig.owners.len() == 0,
            VeraluxErrorCode::SignerNotOwner
        );
    }

    Ok(())
}
