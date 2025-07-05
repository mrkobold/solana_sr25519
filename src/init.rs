use borsh::BorshSerialize;
use solana_program::{
    account_info::{next_account_info, AccountInfo},
    entrypoint::ProgramResult,
    program::invoke_signed,
    pubkey::Pubkey,
    rent::Rent,
    system_instruction,
    sysvar::Sysvar,
};

use crate::SigProgressStruct;

pub fn init(
    program_id: &Pubkey,
    accounts: &[AccountInfo],
    instruction_data: &[u8],
) -> ProgramResult {
    // get accounts
    let accounts_iter = &mut accounts.iter();
    let payer = next_account_info(accounts_iter)?;
    let progress_pda = next_account_info(accounts_iter)?;
    let system_program = next_account_info(accounts_iter)?;

    // build seeds for PDA
    let mut seeds_vec = (&[b"pda" as &[u8]]).to_vec();
    let (_, bump) = Pubkey::find_program_address(&[b"pda" as &[u8]], program_id);
    let binding = [bump];
    seeds_vec.push(&binding);

    // create PDA account for storing progress
    let account_space = 8 + 120 + 32 + 32 + 64; // DotSigCheckStruct size
    let required_lamports = Rent::get()?.minimum_balance(account_space as usize);
    invoke_signed(
        &system_instruction::create_account(
            payer.key,
            progress_pda.key,
            required_lamports,
            account_space as u64,
            program_id,
        ),
        &[payer.clone(), progress_pda.clone(), system_program.clone()],
        &[seeds_vec.as_slice()],
    )?;

    // getting signature verification data
    let signing_key: [u8; 32] = instruction_data[1..33].try_into().unwrap();
    let message: [u8; 32] = instruction_data[33..65].try_into().unwrap();
    let signature: [u8; 64] = instruction_data[65..129].try_into().unwrap();

    // initializing PDA with signature verification data
    let dot_sig_check_struct = SigProgressStruct::new(signing_key, message, signature);
    let mut dot_sig_check_bytes = &mut progress_pda.data.borrow_mut()[..];
    dot_sig_check_struct.serialize(&mut dot_sig_check_bytes)?;

    Ok(())
}
