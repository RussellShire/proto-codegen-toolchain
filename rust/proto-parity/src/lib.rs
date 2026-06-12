pub mod common {
    pub mod v1 {
        include!(concat!(env!("OUT_DIR"), "/common.v1.rs"));
    }
}

pub mod article {
    pub mod v1 {
        include!(concat!(env!("OUT_DIR"), "/article.v1.rs"));
    }
}
