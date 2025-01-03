use anyhow::Result;
use cosmwasm_std::to_json_binary;
use sg721::UpdateCollectionInfoMsg;

use crate::utils::{EventParser, TestSetup};

#[test]
fn test_transfer_nft() -> Result<()> {
    let mut setup = TestSetup::new()?;
    let buyer = setup.users.get_buyer().clone();
    let recipient = setup.users.pixel_operator().clone();

    let response = setup.minter.mint(&mut setup.app, &buyer.address)?;
    let token_id = EventParser::extract_token_id(&response)?;

    // Transfer NFT
    setup.tiles.execute_transfer_nft(
        &mut setup.app,
        &buyer.address,
        &recipient.address,
        token_id.to_string(),
    )?;

    // Verify new owner
    setup
        .tiles
        .assert_token_owner(&setup.app, token_id, &recipient.address);
    Ok(())
}

#[test]
fn test_send_nft() -> Result<()> {
    let mut setup = TestSetup::new()?;
    let buyer = setup.users.get_buyer().clone();
    let recipient = setup.users.pixel_operator().clone();

    let response = setup.minter.mint(&mut setup.app, &buyer.address)?;
    let token_id = EventParser::extract_token_id(&response)?;
    let msg = to_json_binary("test_msg").unwrap();

    // Attempt to send NFT to a non-contract address should fail
    let result = setup.tiles.execute_send_nft(
        &mut setup.app,
        &buyer.address,
        &recipient.address,
        token_id.to_string(),
        msg,
    );

    // Verify it failed because the recipient is not a contract
    assert!(result.is_err());

    // Verify owner hasn't changed
    setup
        .tiles
        .assert_token_owner(&setup.app, token_id, &buyer.address);
    Ok(())
}

#[test]
fn test_approve_operations() -> Result<()> {
    let mut setup = TestSetup::new()?;
    let buyer = setup.users.get_buyer().clone();
    let operator = setup.users.pixel_operator().clone();
    let recipient = setup.users.tile_contract_creator().clone();

    let response = setup.minter.mint(&mut setup.app, &buyer.address)?;
    let token_id = EventParser::extract_token_id(&response)?;

    // Test approval
    setup.tiles.execute_approve(
        &mut setup.app,
        &buyer.address,
        &operator.address,
        token_id.to_string(),
        None,
    )?;

    // Verify operator can transfer token
    setup.tiles.execute_transfer_nft(
        &mut setup.app,
        &operator.address,
        &recipient.address,
        token_id.to_string(),
    )?;

    // Verify new owner
    setup
        .tiles
        .assert_token_owner(&setup.app, token_id, &recipient.address);

    // Transfer back to original owner
    setup.tiles.execute_transfer_nft(
        &mut setup.app,
        &recipient.address,
        &buyer.address,
        token_id.to_string(),
    )?;

    // Test revoke
    setup.tiles.execute_revoke(
        &mut setup.app,
        &buyer.address,
        &operator.address,
        token_id.to_string(),
    )?;

    // Verify operator can no longer transfer token
    let result = setup.tiles.execute_transfer_nft(
        &mut setup.app,
        &operator.address,
        &recipient.address,
        token_id.to_string(),
    );
    assert!(result.is_err());

    Ok(())
}

#[test]
fn test_approve_all_operations() -> Result<()> {
    let mut setup = TestSetup::new()?;
    let buyer = setup.users.get_buyer().clone();
    let operator = setup.users.pixel_operator().clone();
    let recipient = setup.users.tile_contract_creator().clone();

    // Mint multiple tokens
    let mut token_ids = Vec::new();
    for _ in 0..3 {
        let response = setup.minter.mint(&mut setup.app, &buyer.address)?;
        token_ids.push(EventParser::extract_token_id(&response)?);
    }

    // Test approve all
    setup
        .tiles
        .execute_approve_all(&mut setup.app, &buyer.address, &operator.address, None)?;

    // Verify operator can transfer all tokens
    for &token_id in &token_ids {
        setup.tiles.execute_transfer_nft(
            &mut setup.app,
            &operator.address,
            &recipient.address,
            token_id.to_string(),
        )?;

        // Verify new owner
        setup
            .tiles
            .assert_token_owner(&setup.app, token_id, &recipient.address);

        // Transfer back to original owner
        setup.tiles.execute_transfer_nft(
            &mut setup.app,
            &recipient.address,
            &buyer.address,
            token_id.to_string(),
        )?;
    }

    // Test revoke all
    setup
        .tiles
        .execute_revoke_all(&mut setup.app, &buyer.address, &operator.address)?;

    // Verify operator can no longer transfer any tokens
    for &token_id in &token_ids {
        let result = setup.tiles.execute_transfer_nft(
            &mut setup.app,
            &operator.address,
            &recipient.address,
            token_id.to_string(),
        );
        assert!(result.is_err());
    }

    Ok(())
}

#[test]
fn test_burn_nft() -> Result<()> {
    let mut setup = TestSetup::new()?;
    let buyer = setup.users.get_buyer().clone();

    let response = setup.minter.mint(&mut setup.app, &buyer.address)?;
    let token_id = EventParser::extract_token_id(&response)?;

    // Burn NFT
    setup
        .tiles
        .execute_burn(&mut setup.app, &buyer.address, token_id.to_string())?;

    // Verify token no longer exists
    let result = setup.tiles.query_owner_of(&setup.app, token_id.to_string());
    assert!(result.is_err());
    Ok(())
}

#[test]
fn test_update_collection_info() -> Result<()> {
    let mut setup = TestSetup::new()?;
    let creator = setup.users.tile_contract_creator().clone();
    let buyer = setup.users.get_buyer().clone();

    // Debug print current block time
    let block_time = setup.app.inner().block_info().time;
    println!("\nCurrent block time: {}", block_time);

    // Debug print buyer balance
    let balance = setup.app.get_balance(&buyer.address, "ustars")?;
    println!("Buyer balance: {} ustars", balance);

    println!("\nAttempting to mint token...");
    let mint_result = setup.minter.mint(&mut setup.app, &buyer.address);

    match &mint_result {
        Ok(response) => {
            println!("\nMint succeeded. Response events:");
            for (i, event) in response.events.iter().enumerate() {
                println!("\nEvent {}: {}", i, event.ty);
                for attr in &event.attributes {
                    println!("  {} = {}", attr.key, attr.value);
                }
            }
        }
        Err(e) => {
            println!("\nMint failed with error: {}", e);
            return Err(anyhow::anyhow!("Mint failed: {}", e));
        }
    }

    let response = mint_result?;
    let _token_id = EventParser::extract_token_id(&response)?;

    // Update collection info
    let new_collection_info = UpdateCollectionInfoMsg {
        creator: Some(creator.address.to_string()),
        description: Some("Updated description".to_string()),
        image: Some("https://example.com/new-image.png".to_string()),
        external_link: Some(Some("https://example.com/new".to_string())),
        explicit_content: Some(false),
        royalty_info: None,
    };

    setup.tiles.execute_update_collection_info(
        &mut setup.app,
        &creator.address,
        new_collection_info.clone(),
    )?;

    // Verify collection info was updated
    let info = setup.tiles.query_collection_info(&setup.app)?;
    assert_eq!(info.description, new_collection_info.description.unwrap());
    assert_eq!(info.image, new_collection_info.image.unwrap());
    assert_eq!(
        info.external_link,
        new_collection_info.external_link.unwrap()
    );

    Ok(())
}

#[test]
fn test_freeze_collection_info() -> Result<()> {
    let mut setup = TestSetup::new()?;
    let creator = setup.users.tile_contract_creator().clone();

    // Freeze collection info
    setup
        .tiles
        .execute_freeze_collection_info(&mut setup.app, &creator.address)?;

    // Attempt to update collection info should fail
    let new_collection_info = UpdateCollectionInfoMsg {
        creator: Some(creator.address.to_string()),
        description: Some("Updated description".to_string()),
        image: Some("https://example.com/new-image.png".to_string()),
        external_link: Some(Some("https://example.com/new".to_string())),
        explicit_content: Some(false),
        royalty_info: None,
    };

    let result = setup.tiles.execute_update_collection_info(
        &mut setup.app,
        &creator.address,
        new_collection_info,
    );
    assert!(result.is_err());

    Ok(())
}
