use borsh::{BorshDeserialize, BorshSerialize};
use solana_program::{account_info::AccountInfo, entrypoint::ProgramResult, msg, pubkey::Pubkey};

pub mod init;

#[derive(BorshSerialize, BorshDeserialize, Debug)]
pub struct SigProgressStruct {
    pub i: u64,
    pub projective_point: [u64; 15],
    pub signing_key: [u8; 32],
    pub message: [u8; 32],
    pub signature: [u8; 64],
}

impl SigProgressStruct {
    pub fn new(signing_key: [u8; 32], message: [u8; 32], signature: [u8; 64]) -> Self {
        Self {
            i: 300u64,
            projective_point: [0u64; 15],
            signing_key,
            message,
            signature,
        }
    }
}

pub fn process_instruction(
    program_id: &Pubkey,
    accounts: &[AccountInfo],
    instruction_data: &[u8],
) -> ProgramResult {
    let instruction = instruction_data[0];
    msg!("ix code: {:?}", instruction);

    Ok(())
}
