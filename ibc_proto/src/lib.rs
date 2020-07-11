//! ibc-proto library gives the developer access to the Cosmos SDK IBC proto-defined structs.

// Todo: automate the creation of this module setup based on the dots in the filenames.
//  This module setup is necessary because the generated code contains "super::" calls for dependencies.

#![deny(
    warnings,
    missing_docs,
    trivial_casts,
    trivial_numeric_casts,
    unused_import_braces
)]
#![forbid(unsafe_code)]
#![doc(html_root_url = "https://docs.rs/ibc-proto/0.1.0")]

mod cosmos {
    include!("prost/cosmos.rs");
    pub mod query {
        include!("prost/cosmos.query.rs");
    }
}

mod ibc {
    pub mod channel {
        #![allow(missing_docs)]
        include!("prost/ibc.channel.rs");
    }
    pub mod commitment {
        #![allow(missing_docs)]
        include!("prost/ibc.commitment.rs");
    }
    pub mod connection {
        #![allow(missing_docs)]
        include!("prost/ibc.connection.rs");
    }
    pub mod localhost {
        #![allow(missing_docs)]
        include!("prost/ibc.localhost.rs");
    }
    pub mod transfer {
        #![allow(missing_docs)]
        include!("prost/ibc.transfer.rs");
    }
}

mod tendermint {
    pub mod abci {
        #[allow(clippy::large_enum_variant)]
        pub mod types {
            include!("prost/tendermint.abci.types.rs");
        }
    }
    pub mod crypto {
        pub mod merkle {
            include!("prost/tendermint.crypto.merkle.rs");
        }
    }
    pub mod libs {
        pub mod kv {
            include!("prost/tendermint.libs.kv.rs");
        }
    }
}

pub use ibc::*;
