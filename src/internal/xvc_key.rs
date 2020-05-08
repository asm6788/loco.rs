use sha2::{Sha512, Digest};
use std::ops::Deref;

pub struct XVCKey {
    key: String
}

impl XVCKey {
    pub fn new(header: &str, email: &str, device_uuid: &str) -> XVCKey{
        return XVCKey {
            key: hex::encode(
                Sha512::digest(format!("HEATH|{}|DEMIAN|{}|{}", header, email, device_uuid).as_bytes())
            )
        }
    }
}

impl Deref for XVCKey {
    type Target = String;

    fn deref(&self) -> &Self::Target {
        return &self.key;
    }
}