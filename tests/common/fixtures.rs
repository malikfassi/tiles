use cosmwasm_std::{Addr, Coin, Event, Attribute};
use cw_multi_test::{App, Executor};
use sg721::CollectionInfoResponse;
use sg2::msg::{CreateMinterMsg, CollectionInfo};
use vending_minter::msg::ExecuteMsg as VendingMinterExecuteMsg;

use crate::common::{
    constants::{DEFAULT_MINT_PRICE, NATIVE_DENOM},
    mock::init_modules,
    tiles_contract::TilesContract,
    vending_factory::VendingFactory,
};
use tiles::defaults::config::{
    DEFAULT_ADMIN, DEFAULT_BASE_PRICE, DEFAULT_DEV_ADDRESS, DEFAULT_DEV_FEE_PERCENT,
    DEFAULT_MINTER, DEFAULT_PRICE_SCALING,
};
use tiles::msg::InstantiateMsg;
use tiles::types::Config;
use tiles::utils::events::EventType;

pub const DEFAULT_CONFIG: Config = Config {
    admin: Addr::unchecked(DEFAULT_ADMIN),
    minter: Addr::unchecked(DEFAULT_MINTER),
    dev_address: Addr::unchecked(DEFAULT_DEV_ADDRESS),
    dev_fee_percent: DEFAULT_DEV_FEE_PERCENT,
    base_price: DEFAULT_BASE_PRICE,
    price_scaling: DEFAULT_PRICE_SCALING,
};

pub struct TestSetup {
    pub app: App,
    pub sender: Addr,
    pub tiles: TilesContract,
    pub factory: VendingFactory,
}

impl TestSetup {
    pub fn new() -> Result<Self, Box<dyn std::error::Error>> {
        let (mut app, sender) = init_modules();

        // Create factory
        let factory = VendingFactory::new(&mut app, &sender);

        // Create collection
        let mint_price = Coin::new(DEFAULT_MINT_PRICE, NATIVE_DENOM);
        let (minter_addr, collection_addr) = factory.create_minter(
            &mut app,
            &sender,
            CreateMinterMsg {
                init_msg: None,
                collection_params: sg2::msg::CollectionParams {
                    code_id: 0, // Set by factory
                    name: "Tiles".to_string(),
                    symbol: "TILE".to_string(),
                    max_supply: Some(100),
                    mint_price,
                    per_address_limit: 10,
                    start_time: None,
                    payment_address: None,
                    collection_info: CollectionInfo {
                        creator: sender.to_string(),
                        description: "A collection of tiles".to_string(),
                        image: "ipfs://image".to_string(),
                        external_link: None,
                        explicit_content: None,
                        start_trading_time: None,
                        royalty_info: None,
                    },
                },
            },
        )?;

        // Query balance to ensure funds are available
        let balance = app.wrap().query_balance(sender.as_str(), NATIVE_DENOM)?;
        println!("Balance: {}", balance.amount);

        // Mint a token
        app.execute_contract(
            sender.clone(),
            minter_addr,
            &VendingMinterExecuteMsg::Mint {},
            &[Coin::new(DEFAULT_MINT_PRICE, NATIVE_DENOM)],
        )?;

        let tiles = TilesContract::new(collection_addr);

        Ok(Self {
            app,
            sender,
            tiles,
            factory,
        })
    }

    /// Find event by type and action
    pub fn find_event(&self, event_type: &str, action: &str) -> Option<&Event> {
        self.app.events().iter().find(|e| {
            e.ty == event_type
                && e.attributes
                    .iter()
                    .any(|a| a.key == "action" && a.value == action)
        })
    }

    /// Find event by EventType
    pub fn find_event_by_type(&self, event_type: EventType) -> Option<&Event> {
        self.find_event("wasm", event_type.as_str())
    }

    /// Assert event exists with specific attributes
    pub fn assert_event(
        &self,
        event_type: &str,
        action: &str,
        expected_attrs: &[(&str, &str)],
    ) {
        let event = self
            .find_event(event_type, action)
            .unwrap_or_else(|| panic!("Event not found: type={}, action={}", event_type, action));

        self.assert_event_attributes(event, expected_attrs);
    }

    /// Assert event exists for EventType with specific attributes
    pub fn assert_event_by_type(
        &self,
        event_type: EventType,
        expected_attrs: &[(&str, &str)],
    ) {
        let event = self
            .find_event_by_type(event_type)
            .unwrap_or_else(|| panic!("Event not found: type={:?}", event_type));

        self.assert_event_attributes(event, expected_attrs);
    }

    /// Assert event attributes match expected
    pub fn assert_event_attributes(&self, event: &Event, expected_attrs: &[(&str, &str)]) {
        let attrs: Vec<_> = event
            .attributes
            .iter()
            .map(|a| (a.key.as_str(), a.value.as_str()))
            .collect();

        for expected in expected_attrs {
            assert!(
                attrs.contains(expected),
                "Expected attribute {:?} not found in event",
                expected
            );
        }
    }

    /// Assert event sequence matches expected
    pub fn assert_event_sequence(&self, expected_sequence: &[EventType]) {
        let events: Vec<_> = self
            .app
            .events()
            .iter()
            .filter(|e| e.ty == "wasm")
            .filter_map(|e| {
                e.attributes
                    .iter()
                    .find(|a| a.key == "action")
                    .map(|a| a.value.as_str())
            })
            .collect();

        let expected: Vec<_> = expected_sequence.iter().map(|e| e.as_str()).collect();

        assert_eq!(
            events, expected,
            "Event sequence mismatch.\nExpected: {:?}\nActual: {:?}",
            expected, events
        );
    }

    /// Get all events of a specific type
    pub fn get_events(&self, event_type: &str) -> Vec<&Event> {
        self.app
            .events()
            .iter()
            .filter(|e| e.ty == event_type)
            .collect()
    }

    /// Get all events of a specific EventType
    pub fn get_events_by_type(&self, event_type: EventType) -> Vec<&Event> {
        self.app
            .events()
            .iter()
            .filter(|e| {
                e.ty == "wasm"
                    && e.attributes
                        .iter()
                        .any(|a| a.key == "action" && a.value == event_type.as_str())
            })
            .collect()
    }

    /// Assert pixel update event
    pub fn assert_pixel_update_event(
        &self,
        tiles_updated: u32,
        pixels_updated: u32,
        updater: &str,
    ) {
        self.assert_event_by_type(
            EventType::SetPixelColor,
            &[
                ("tiles_updated", &tiles_updated.to_string()),
                ("pixels_updated", &pixels_updated.to_string()),
                ("updater", updater),
            ],
        );
    }

    /// Assert config update event
    pub fn assert_config_update_event(&self, updated_fields: &[&str]) {
        self.assert_event_by_type(
            EventType::UpdateConfig,
            &[("updated_fields", &updated_fields.join(","))],
        );
    }

    /// Assert payment event
    pub fn assert_payment_event(
        &self,
        total_amount: u128,
        royalty_amount: Option<u128>,
    ) {
        let mut expected_attrs = vec![
            ("action", EventType::PaymentProcessed.as_str()),
            ("total_amount", &total_amount.to_string()),
            ("denom", NATIVE_DENOM),
        ];

        if let Some(royalty) = royalty_amount {
            expected_attrs.push(("royalty_amount", &royalty.to_string()));
        }

        self.assert_event_by_type(EventType::PaymentProcessed, &expected_attrs);
    }
}

pub fn setup_test() -> Result<TestSetup, Box<dyn std::error::Error>> {
    TestSetup::new()
}
