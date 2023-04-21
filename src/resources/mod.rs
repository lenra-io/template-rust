macro_rules! resource {
    ($path: tt) => {
        ($path, include_bytes!($path))
    };
}

pub const RESOURCE_MAP: [(&str, &[u8]); 1] = [resource!("logo.png")];
