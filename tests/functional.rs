use solana_program_test::*;
use solana_sdk::{
    signature::{Keypair, Signer},
    transaction::Transaction,
    instruction::{Instruction, AccountMeta},
    pubkey::Pubkey,
};
use solana_voting_program::state::{Counter, TAG_COUNTER};
use solana_voting_program::processor::Processor;
use borsh::{BorshDeserialize};

#[tokio::test]
async fn test_counter_engine() {
    // 1. Setup the Simulator
    let program_id = Pubkey::new_unique();
    let mut program_test = ProgramTest::new(
        "solana_voting_program",
        program_id,
        processor!(Processor::process),
    );

    let (mut banks_client, payer, recent_blockhash) = program_test.start().await;
    let counter_keypair = Keypair::new();
    let counter_size = 41; 

    // 2. Calculate Rent (The "Storage Tax")
    let rent = banks_client.get_rent().await.unwrap();
    let lamports = rent.minimum_balance(counter_size);

    // 3. Create Account (System Instruction) - Allocates memory
    let create_account_ix = solana_sdk::system_instruction::create_account(
        &payer.pubkey(),
        &counter_keypair.pubkey(),
        lamports,
        counter_size as u64,
        &program_id,
    );

    // 4. Initialize Counter (Our Program Instruction) - Formats memory
    let mut instruction_data = vec![1]; // Tag for InitializeCounter
    instruction_data.extend_from_slice(&10u64.to_le_bytes());

    let init_ix = Instruction {
        program_id,
        accounts: vec![
            AccountMeta::new(payer.pubkey(), true),
            AccountMeta::new(counter_keypair.pubkey(), false),
            AccountMeta::new_readonly(solana_program::system_program::id(), false),
        ],
        data: instruction_data,
    };

    // 5. Bundle and Sign (The Atomic Ignition)
    let mut tx = Transaction::new_with_payer(
        &[create_account_ix, init_ix], 
        Some(&payer.pubkey())
    );
    tx.sign(&[&payer, &counter_keypair], recent_blockhash);
    
    banks_client.process_transaction(tx).await.unwrap();

    // 6. Verification (The Odometer Check)
    let account = banks_client.get_account(counter_keypair.pubkey()).await.unwrap().expect("Account not found");
    let counter_state = Counter::try_from_slice(&account.data).unwrap();
    
    assert_eq!(counter_state.tag, TAG_COUNTER);
    assert_eq!(counter_state.count, 10);
}