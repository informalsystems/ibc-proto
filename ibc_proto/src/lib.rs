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
    pub mod base {
        pub mod v1beta1 {
            include!("prost/cosmos.base.v1beta1.rs");
        }
        pub mod query {
            pub mod v1beta1 {
                include!("prost/cosmos.base.query.v1beta1.rs");
            }
        }
        pub mod crypto {
            pub mod v1beta1 {
                include!("prost/cosmos.base.crypto.v1beta1.rs");
            }
        }
    }
    pub mod tx {
        pub mod v1beta1 {
            include!("prost/cosmos.tx.v1beta1.rs");
        }
        pub mod signing {
            pub mod v1beta1 {
                include!("prost/cosmos.tx.signing.v1beta1.rs");
            }
        }
    }
}

mod ibc {
    pub mod client {
        #![allow(missing_docs)]
        include!("prost/ibc.client.rs");
    }
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
    pub mod tendermint {
        #![allow(missing_docs)]
        include!("prost/ibc.tendermint.rs");
    }
    pub mod transfer {
        #![allow(missing_docs)]
        include!("prost/ibc.transfer.rs");
    }
    pub mod types {
        #![allow(missing_docs)]
        include!("prost/ibc.types.rs");
    }
}

mod ics23 {
    include!("prost/ics23.rs");
}

mod tendermint {
    pub mod crypto {
        include!("prost/tendermint.crypto.rs");
    }
    pub mod libs {
        pub mod bits {
            include!("prost/tendermint.libs.bits.rs");
        }
    }
    pub mod types {
        include!("prost/tendermint.types.rs");
    }
    pub mod version {
        include!("prost/tendermint.version.rs");
    }
}

pub use ibc::*;
