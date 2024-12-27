pub mod common;

mod contract {
    pub mod mint;
    pub mod pixel;
    pub mod pricescaling;
    pub mod sg721_features;
}

mod core {
    pub mod pricing {
        pub mod calculation;
        pub mod validation;
    }
    pub mod tile {
        pub mod hash;
        pub mod metadata;
    }
}
