// Todo: automate the creation of this module setup based on the dots in the filenames.
//  This module setup is necessary because the generated code contains "super::" calls for dependencies.

mod cosmos {
    include!(concat!(env!("OUT_DIR"), "/cosmos.rs"));
    pub mod query {
        include!(concat!(env!("OUT_DIR"), "/cosmos.query.rs"));
    }
}

mod ibc {
    pub mod channel {
        include!(concat!(env!("OUT_DIR"), "/ibc.channel.rs"));
    }
    pub mod commitment {
        include!(concat!(env!("OUT_DIR"), "/ibc.commitment.rs"));
    }
    pub mod connection {
        include!(concat!(env!("OUT_DIR"), "/ibc.connection.rs"));
    }
    pub mod localhost {
        include!(concat!(env!("OUT_DIR"), "/ibc.localhost.rs"));
    }
    pub mod transfer {
        include!(concat!(env!("OUT_DIR"), "/ibc.transfer.rs"));
    }
}

mod tendermint {
    pub mod abci {
        #[allow(clippy::large_enum_variant)]
        pub mod types {
            include!(concat!(env!("OUT_DIR"), "/tendermint.abci.types.rs"));
        }
    }
    pub mod crypto {
        pub mod merkle {
            include!(concat!(env!("OUT_DIR"), "/tendermint.crypto.merkle.rs"));
        }
    }
    pub mod libs {
        pub mod kv {
            include!(concat!(env!("OUT_DIR"), "/tendermint.libs.kv.rs"));
        }
    }
}

pub use ibc::*;
