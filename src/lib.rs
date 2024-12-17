use solana_program::{
    account_info::{next_account_info, AccountInfo},
    entrypoint,
    entrypoint::ProgramResult,
    msg,
    program::invoke,
    program_error::ProgramError,
    pubkey::Pubkey,
    system_instruction,
    sysvar::{rent::Rent, Sysvar},
};

use borsh::{BorshDeserialize, BorshSerialize};

entrypoint!(process_instruction);
 
pub fn process_instruction(
    program_id: &Pubkey,
    accounts: &[AccountInfo],
    instruction_data: &[u8],
) -> ProgramResult {
    let instruction = CounterInstruction::unpack(instruction_data)?;

    match instruction {
        CounterInstruction::InitializeCounter { initial_value } => {
            process_initialize_counter(program_id, accounts, initial_value)?
        }
        CounterInstruction::IncrementCounter => {
            process_increment_counter(program_id, accounts)?
        }
    }
    Ok(())
}

fn process_initialize_counter(program_id: &Pubkey, accounts: &[AccountInfo], initial_value: u64) -> ProgramResult {
    let accout_iter = &mut accounts.iter();

    let counter_account = next_account_info(accout_iter)?;
    let payer_account = next_account_info(accout_iter)?;
    let system_program = next_account_info(accout_iter)?;

    let account_space = 8;

    let rent = Rent::get()?;
    let required_lamports = rent.minimum_balance(account_space);

    invoke(&system_instruction::create_account(
        payer_account.key,
        counter_account.key, 
        required_lamports, account_space as u64, 
        program_id), 
        &[
            payer_account.clone(),
            counter_account.clone(),
            system_program.clone(),
        ]
    )?;

    let counter_data = CounterAccount{
        count: initial_value,
    };
    let mut account_data = &mut counter_account.data.borrow_mut()[..];
    counter_data.serialize(&mut account_data)?;
    msg!("Counter initialized with value: {}", initial_value);
    Ok(())
}

fn process_increment_counter(program_id: &Pubkey, accounts: &[AccountInfo]) -> ProgramResult {
    Ok(())
}

#[derive(BorshDeserialize, BorshSerialize, Debug)]
pub struct CounterAccount{
    count : u64,
}

#[derive(BorshSerialize, BorshDeserialize, Debug)]
pub enum CounterInstruction{
    InitializeCounter {initial_value: u64},
    IncrementCounter,
}

impl CounterInstruction {
    pub fn unpack(input: &[u8]) -> Result<Self, ProgramError> {
        let (&variant, rest) = input.split_first().ok_or(ProgramError::InvalidInstructionData)?;

        match variant{
            0 => {
                let initial_value = u64::from_le_bytes(rest.try_into().map_err(|_| ProgramError::InvalidInstructionData)?
            );
            Ok(Self::InitializeCounter { initial_value })
            }
            1 => Ok(Self::IncrementCounter),
            _ => Err(ProgramError::InvalidInstructionData),
        }
    }
}