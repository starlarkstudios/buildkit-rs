/// `github.com/moby/buildkit/solver/pb/ops.proto`
pub mod pb {
    include!(concat!(env!("OUT_DIR"), "/pb.rs"));
}

pub mod fsutil {
    pub mod types {
        include!(concat!(env!("OUT_DIR"), "/fsutil.types.rs"));
    }
}

pub mod google {
    pub mod rpc {
        include!(concat!(env!("OUT_DIR"), "/google.rpc.rs"));
    }
}

pub mod moby {
    pub mod buildkit {
        pub mod v1 {
            pub mod apicaps {
                include!(concat!(env!("OUT_DIR"), "/moby.buildkit.v1.apicaps.rs"));
            }
            pub mod frontend {
                include!(concat!(env!("OUT_DIR"), "/moby.buildkit.v1.frontend.rs"));
            }
            pub mod sourcepolicy {
                include!(concat!(env!("OUT_DIR"), "/moby.buildkit.v1.sourcepolicy.rs"));
            }
            pub mod types {
                include!(concat!(env!("OUT_DIR"), "/moby.buildkit.v1.types.rs"));
            }
        }
    }
}
