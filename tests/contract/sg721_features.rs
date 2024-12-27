#[cfg(test)]
mod tests {
    use anyhow::Result;
    use cosmwasm_std::{to_json_binary, Addr};
    use cw721_base::Action;
    use sg721::UpdateCollectionInfoMsg;
    use tiles::core::tile::metadata::TileMetadata;
    use crate::common::TestOrchestrator;

    #[test]
    fn test_transfer_nft() -> Result<()> {
        let mut test = TestOrchestrator::new();
        let (owner, token_id) = test.setup_single_token()?;
        let recipient = test.ctx.users.pixel_operator().address.clone();

        // Transfer NFT
        test.ctx.tiles.execute_transfer_nft(
            &mut test.ctx.app,
            &owner,
            &recipient,
            token_id.to_string(),
        )?;

        // Verify new owner
        test.assert_token_owner(token_id, &recipient);
        Ok(())
    }

    #[test]
    fn test_send_nft() -> Result<()> {
        let mut test = TestOrchestrator::new();
        let (owner, token_id) = test.setup_single_token()?;
        let recipient_contract = test.ctx.users.pixel_operator().address.clone();
        let msg = to_json_binary("test_msg").unwrap();

        // Attempt to send NFT to a non-contract address should fail
        let result = test.ctx.tiles.execute_send_nft(
            &mut test.ctx.app,
            &owner,
            &recipient_contract,
            token_id.to_string(),
            msg,
        );

        // Verify it failed because the recipient is not a contract
        assert!(result.is_err());
        
        // Verify owner hasn't changed
        test.assert_token_owner(token_id, &owner);
        Ok(())
    }

    #[test]
    fn test_approval_operations() -> Result<()> {
        let mut test = TestOrchestrator::new();
        let (owner, token_id) = test.setup_single_token()?;
        let operator = test.ctx.users.pixel_operator().address.clone();
        let recipient = test.ctx.users.get_buyer().address.clone();

        // Test approval
        test.ctx.tiles.execute_approve(
            &mut test.ctx.app,
            &owner,
            &operator,
            token_id.to_string(),
            None,
        )?;

        // Verify operator can transfer token
        test.ctx.tiles.execute_transfer_nft(
            &mut test.ctx.app,
            &operator,
            &recipient,
            token_id.to_string(),
        )?;

        // Verify new owner
        test.assert_token_owner(token_id, &recipient);

        // Transfer back to original owner
        test.ctx.tiles.execute_transfer_nft(
            &mut test.ctx.app,
            &recipient,
            &owner,
            token_id.to_string(),
        )?;

        // Test revoke
        test.ctx.tiles.execute_revoke(
            &mut test.ctx.app,
            &owner,
            &operator,
            token_id.to_string(),
        )?;

        // Verify operator can no longer transfer token
        let result = test.ctx.tiles.execute_transfer_nft(
            &mut test.ctx.app,
            &operator,
            &recipient,
            token_id.to_string(),
        );
        test.assert_error_unauthorized(result);

        Ok(())
    }

    #[test]
    fn test_approve_all_operations() -> Result<()> {
        let mut test = TestOrchestrator::new();
        let (owner, token_ids) = test.setup_multiple_tokens(3)?;
        let operator = test.ctx.users.pixel_operator().address.clone();
        let recipient = test.ctx.users.get_buyer().address.clone();

        // Test approve all
        test.ctx.tiles.execute_approve_all(
            &mut test.ctx.app,
            &owner,
            &operator,
            None,
        )?;

        // Verify operator can transfer all tokens
        for &token_id in &token_ids {
            test.ctx.tiles.execute_transfer_nft(
                &mut test.ctx.app,
                &operator,
                &recipient,
                token_id.to_string(),
            )?;

            // Verify new owner
            test.assert_token_owner(token_id, &recipient);

            // Transfer back to original owner
            test.ctx.tiles.execute_transfer_nft(
                &mut test.ctx.app,
                &recipient,
                &owner,
                token_id.to_string(),
            )?;
        }

        // Test revoke all
        test.ctx.tiles.execute_revoke_all(
            &mut test.ctx.app,
            &owner,
            &operator,
        )?;

        // Verify operator can no longer transfer tokens
        let result = test.ctx.tiles.execute_transfer_nft(
            &mut test.ctx.app,
            &operator,
            &recipient,
            token_ids[0].to_string(),
        );
        test.assert_error_unauthorized(result);

        Ok(())
    }

    #[test]
    fn test_update_ownership() -> Result<()> {
        let mut test = TestOrchestrator::new();
        let minter_addr = test.ctx.minter.contract_addr.clone();
        let new_owner = Addr::unchecked("new_contract_owner");

        // First mint a token to ensure contract is properly initialized
        test.setup_single_token()?;

        // Transfer ownership
        let transfer_response = test.ctx.tiles.execute_update_ownership(
            &mut test.ctx.app,
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
        
        assert!(transfer_event.attributes.iter().any(|attr| 
            attr.key == "owner" && attr.value == minter_addr.to_string()
        ));
        assert!(transfer_event.attributes.iter().any(|attr| 
            attr.key == "pending_owner" && attr.value == new_owner.to_string()
        ));
        assert!(transfer_event.attributes.iter().any(|attr| 
            attr.key == "pending_expiry" && attr.value == "none"
        ));

        // Accept ownership
        let accept_response = test.ctx.tiles.execute_update_ownership(
            &mut test.ctx.app,
            &new_owner,
            Action::AcceptOwnership {},
        )?;

        // Verify accept ownership event
        let accept_event = accept_response
            .events
            .iter()
            .find(|e| e.ty == "wasm")
            .expect("Expected wasm event");
        
        assert!(accept_event.attributes.iter().any(|attr| 
            attr.key == "owner" && attr.value == new_owner.to_string()
        ));

        Ok(())
    }

    #[test]
    fn test_burn_nft() -> Result<()> {
        let mut test = TestOrchestrator::new();
        let (owner, token_id) = test.setup_single_token()?;

        // Burn NFT
        test.ctx
            .tiles
            .execute_burn(&mut test.ctx.app, &owner, token_id.to_string())?;

        // Verify token no longer exists
        let result = test
            .ctx
            .tiles
            .query_owner_of(&mut test.ctx.app, token_id.to_string());
        assert!(result.is_err());
        Ok(())
    }

    #[test]
    fn test_update_collection_info() -> Result<()> {
        let mut test = TestOrchestrator::new();
        let creator = test.ctx.users.get_creator().address.clone();

        // Update collection info
        let new_collection_info = UpdateCollectionInfoMsg {
            creator: Some(creator.to_string()),
            description: Some("Updated description".to_string()),
            image: Some("https://example.com/new-image.png".to_string()),
            external_link: Some(Some("https://example.com/new".to_string())),
            explicit_content: Some(false),
            royalty_info: None,
        };

        test.ctx.tiles.execute_update_collection_info(
            &mut test.ctx.app,
            &creator,
            new_collection_info.clone(),
        )?;

        // Verify collection info was updated
        let info = test.ctx.tiles.query_collection_info(&mut test.ctx.app)?;
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
        let mut test = TestOrchestrator::new();
        let creator = test.ctx.users.get_creator().address.clone();

        // Freeze collection info
        test.ctx
            .tiles
            .execute_freeze_collection_info(&mut test.ctx.app, &creator)?;

        // Attempt to update collection info should fail
        let new_collection_info = UpdateCollectionInfoMsg {
            creator: Some(creator.to_string()),
            description: Some("Updated description".to_string()),
            image: Some("https://example.com/new-image.png".to_string()),
            external_link: Some(Some("https://example.com/new".to_string())),
            explicit_content: Some(false),
            royalty_info: None,
        };

        let result = test.ctx.tiles.execute_update_collection_info(
            &mut test.ctx.app,
            &creator,
            new_collection_info,
        );
        assert!(result.is_err());

        Ok(())
    }
}
