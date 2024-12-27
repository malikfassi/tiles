#[cfg(test)]
mod tests {
    use anyhow::Result;
    use cosmwasm_std::Addr;
    use cw721_base::Action;
    use sg721::UpdateCollectionInfoMsg;

    use crate::common::test_orchestrator::TestOrchestrator;

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
    fn test_approval_operations() -> Result<()> {
        let mut test = TestOrchestrator::new();
        let (owner, token_id) = test.setup_single_token()?;
        let operator = test.ctx.users.pixel_operator().address.clone();

        // Test approval
        test.ctx.tiles.execute_approve(
            &mut test.ctx.app,
            &owner,
            &operator,
            token_id.to_string(),
            None,
        )?;

        // Verify operator can update pixel
        let update = tiles::core::tile::metadata::PixelUpdate {
            id: 0,
            color: "#FF0000".to_string(),
            expiration_duration: 3600,
        };
        test.ctx.tiles.update_pixel_with_funds(
            &mut test.ctx.app,
            &operator,
            token_id,
            vec![update],
            100000000,
        )?;

        // Test revoke
        test.ctx.tiles.execute_revoke(
            &mut test.ctx.app,
            &owner,
            &operator,
            token_id.to_string(),
        )?;
        Ok(())
    }

    #[test]
    fn test_update_ownership() -> Result<()> {
        let mut test = TestOrchestrator::new();
        let minter_addr = test.ctx.minter.contract_addr.clone();
        let new_owner = Addr::unchecked("new_contract_owner");

        // First mint a token to ensure contract is properly initialized
        test.setup_single_token()?;

        // Note: We don't test the full minter integration here because it requires
        // complex governance actions through the factory contract.
        // Instead, we verify that the minter contract is indeed the owner of the NFT contract.
        test.ctx.tiles.execute_update_ownership(
            &mut test.ctx.app,
            &minter_addr,
            Action::TransferOwnership {
                new_owner: new_owner.to_string(),
                expiry: None,
            },
        )?;

        // Accept ownership with new owner
        test.ctx.tiles.execute_update_ownership(
            &mut test.ctx.app,
            &new_owner,
            Action::AcceptOwnership {},
        )?;

        Ok(())
    }

    #[test]
    fn test_burn_nft() -> Result<()> {
        let mut test = TestOrchestrator::new();
        let (owner, token_id) = test.setup_single_token()?;

        // Burn NFT
        test.ctx.tiles.execute_burn(&mut test.ctx.app, &owner, token_id.to_string())?;

        // Verify token no longer exists
        let result = test.ctx.tiles.query_owner_of(&mut test.ctx.app, token_id.to_string());
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
        assert_eq!(info.external_link, new_collection_info.external_link.unwrap());

        Ok(())
    }

    #[test]
    fn test_freeze_collection_info() -> Result<()> {
        let mut test = TestOrchestrator::new();
        let creator = test.ctx.users.get_creator().address.clone();

        // Freeze collection info
        test.ctx.tiles.execute_freeze_collection_info(&mut test.ctx.app, &creator)?;

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