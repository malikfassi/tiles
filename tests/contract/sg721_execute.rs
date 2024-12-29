use anyhow::Result;
use cosmwasm_std::{to_json_binary, Addr};
use cw721_base::Action;
use sg721::UpdateCollectionInfoMsg;

use crate::common::{
    extract_token_id,
    launchpad::Launchpad,
    test_context::TestContext,
};

#[test]
fn test_transfer_nft() -> Result<()> {
    let buyer = TestContext::new().users.get_buyer().clone();
    let (mut ctx, token_id, _) = TestContext::with_minted_token(&buyer.address)?;
    let recipient = ctx.users.pixel_operator().clone();

    // Transfer NFT
    ctx.tiles.execute_transfer_nft(
        &mut ctx.app,
        &buyer.address,
        &recipient.address,
        token_id.to_string(),
    )?;

    // Verify new owner
    ctx.tiles.assert_token_owner(&ctx.app, token_id, &recipient.address);
    Ok(())
}

#[test]
fn test_send_nft() -> Result<()> {
    let buyer = TestContext::new().users.get_buyer().clone();
    let (mut ctx, token_id, _) = TestContext::with_minted_token(&buyer.address)?;
    let recipient = ctx.users.pixel_operator().clone();
    let msg = to_json_binary("test_msg").unwrap();

    // Attempt to send NFT to a non-contract address should fail
    let result = ctx.tiles.execute_send_nft(
        &mut ctx.app,
        &buyer.address,
        &recipient.address,
        token_id.to_string(),
        msg,
    );

    // Verify it failed because the recipient is not a contract
    assert!(result.is_err());

    // Verify owner hasn't changed
    ctx.tiles.assert_token_owner(&ctx.app, token_id, &buyer.address);
    Ok(())
}

#[test]
fn test_approve_operations() -> Result<()> {
    let mut launchpad = Launchpad::new();
    let owner = launchpad.users.get_buyer();
    let operator = launchpad.users.pixel_operator();
    let recipient = launchpad.users.tile_contract_creator();

    let response = launchpad.minter.mint(&mut launchpad.app, &owner.address)?;
    let token_id = extract_token_id(&response);

    // Test approval
    launchpad.tiles.execute_approve(
        &mut launchpad.app,
        &owner.address,
        &operator.address,
        token_id.to_string(),
        None,
    )?;

    // Verify operator can transfer token
    launchpad.tiles.execute_transfer_nft(
        &mut launchpad.app,
        &operator.address,
        &recipient.address,
        token_id.to_string(),
    )?;

    // Verify new owner
    launchpad.tiles.assert_token_owner(&launchpad.app, token_id, &recipient.address);

    // Transfer back to original owner
    launchpad.tiles.execute_transfer_nft(
        &mut launchpad.app,
        &recipient.address,
        &owner.address,
        token_id.to_string(),
    )?;

    // Test revoke
    launchpad.tiles.execute_revoke(
        &mut launchpad.app,
        &owner.address,
        &operator.address,
        token_id.to_string(),
    )?;

    // Verify operator can no longer transfer token
    let result = launchpad.tiles.execute_transfer_nft(
        &mut launchpad.app,
        &operator.address,
        &recipient.address,
        token_id.to_string(),
    );
    assert!(result.is_err());

    Ok(())
}

#[test]
fn test_approve_all_operations() -> Result<()> {
    let mut launchpad = Launchpad::new();
    let owner = launchpad.users.get_buyer();
    let operator = launchpad.users.pixel_operator();
    let recipient = launchpad.users.tile_contract_creator();

    // Mint multiple tokens
    let mut token_ids = Vec::new();
    for _ in 0..3 {
        let response = launchpad.minter.mint(&mut launchpad.app, &owner.address)?;
        token_ids.push(extract_token_id(&response));
    }

    // Test approve all
    launchpad.tiles.execute_approve_all(
        &mut launchpad.app,
        &owner.address,
        &operator.address,
        None,
    )?;

    // Verify operator can transfer all tokens
    for &token_id in &token_ids {
        launchpad.tiles.execute_transfer_nft(
            &mut launchpad.app,
            &operator.address,
            &recipient.address,
            token_id.to_string(),
        )?;

        // Verify new owner
        launchpad.tiles.assert_token_owner(&launchpad.app, token_id, &recipient.address);

        // Transfer back to original owner
        launchpad.tiles.execute_transfer_nft(
            &mut launchpad.app,
            &recipient.address,
            &owner.address,
            token_id.to_string(),
        )?;
    }

    // Test revoke all
    launchpad.tiles.execute_revoke_all(
        &mut launchpad.app,
        &owner.address,
        &operator.address,
    )?;

    // Verify operator can no longer transfer tokens
    let result = launchpad.tiles.execute_transfer_nft(
        &mut launchpad.app,
        &operator.address,
        &recipient.address,
        token_ids[0].to_string(),
    );
    assert!(result.is_err());

    Ok(())
}

#[test]
fn test_update_ownership() -> Result<()> {
    let mut launchpad = Launchpad::new();
    let minter_addr = launchpad.minter.contract_addr.clone();
    let new_owner = Addr::unchecked("new_contract_owner");

    // First mint a token to ensure contract is properly initialized
    let response = launchpad.minter.mint(&mut launchpad.app, &launchpad.users.get_buyer().address)?;
    let _token_id = extract_token_id(&response);

    // Transfer ownership
    let transfer_response = launchpad.tiles.execute_update_ownership(
        &mut launchpad.app,
        &minter_addr,
        Action::TransferOwnership {
            new_owner: new_owner.to_string(),
            expiry: None,
        },
    )?;

    let transfer_event = transfer_response
        .events
        .iter()
        .find(|e| e.ty == "wasm")
        .expect("Expected wasm event");

    assert!(transfer_event
        .attributes
        .iter()
        .any(|attr| attr.key == "owner" && attr.value == minter_addr));
    assert!(transfer_event
        .attributes
        .iter()
        .any(|attr| attr.key == "pending_owner" && attr.value == new_owner));
    assert!(transfer_event
        .attributes
        .iter()
        .any(|attr| attr.key == "pending_expiry" && attr.value == "none"));

    // Accept ownership
    let accept_response = launchpad.tiles.execute_update_ownership(
        &mut launchpad.app,
        &new_owner,
        Action::AcceptOwnership {},
    )?;

    // Verify accept ownership event
    let accept_event = accept_response
        .events
        .iter()
        .find(|e| e.ty == "wasm")
        .expect("Expected wasm event");

    assert!(accept_event
        .attributes
        .iter()
        .any(|attr| attr.key == "owner" && attr.value == new_owner));

    Ok(())
}

#[test]
fn test_burn_nft() -> Result<()> {
    let buyer = TestContext::new().users.get_buyer().clone();
    let (mut ctx, token_id, _) = TestContext::with_minted_token(&buyer.address)?;

    // Burn NFT
    ctx.tiles.execute_burn(
        &mut ctx.app,
        &buyer.address,
        token_id.to_string(),
    )?;

    // Verify token no longer exists
    let result = ctx.tiles.query_owner_of(&ctx.app, token_id.to_string());
    assert!(result.is_err());
    Ok(())
}

#[test]
fn test_update_collection_info() -> Result<()> {
    let buyer = TestContext::new().users.get_buyer().clone();
    let (mut ctx, token_id, _) = TestContext::with_minted_token(&buyer.address)?;
    let creator = ctx.users.tile_contract_creator().clone();
    
    let new_info = UpdateCollectionInfoMsg {
        creator: Some(creator.address.to_string()),
        description: Some("Updated description".to_string()),
        image: Some("https://example.com/new-image.png".to_string()),
        external_link: Some(Some("https://example.com/new".to_string())),
        explicit_content: Some(false),
        royalty_info: None,
    };

    let response = ctx.tiles.execute_update_collection_info(
        &mut ctx.app,
        &creator.address,
        new_info.clone(),
    )?;

    // Verify collection info was updated
    let info = ctx.tiles.query_collection_info(&ctx.app)?;
    assert_eq!(info.description, new_info.description.unwrap());
    assert_eq!(info.image, new_info.image.unwrap());
    assert_eq!(info.external_link, new_info.external_link.unwrap());

    Ok(())
}

#[test]
fn test_freeze_collection_info() -> Result<()> {
    let buyer = TestContext::new().users.get_buyer().clone();
    let (mut ctx, token_id, _) = TestContext::with_minted_token(&buyer.address)?;
    let creator = ctx.users.tile_contract_creator().clone();

    // Freeze collection info
    ctx.tiles.execute_freeze_collection_info(
        &mut ctx.app,
        &creator.address,
    )?;

    // Attempt to update collection info should fail
    let new_collection_info = UpdateCollectionInfoMsg {
        creator: Some(creator.address.to_string()),
        description: Some("Updated description".to_string()),
        image: Some("https://example.com/new-image.png".to_string()),
        external_link: Some(Some("https://example.com/new".to_string())),
        explicit_content: Some(false),
        royalty_info: None,
    };

    let result = ctx.tiles.execute_update_collection_info(
        &mut ctx.app,
        &creator.address,
        new_collection_info,
    );
    assert!(result.is_err());

    Ok(())
}
