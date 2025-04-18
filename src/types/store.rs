use crate::{capnp_get_text, error, store_capnp};
use capnp::message::Builder;

pub(crate) struct StoreSetOpts<'a> {
    key: &'a str,
    value: &'a str,
}

impl StoreSetOpts<'_> {
    pub fn new<'a>(key: &'a str, value: &'a str) -> StoreSetOpts<'a> {
        StoreSetOpts { key, value }
    }

    pub fn to_capnp_message(&self) -> Result<Vec<u8>, error::Error> {
        let mut message = Builder::new_default();
        let mut request = message.init_root::<store_capnp::store_set_request::Builder>();

        request.set_key(self.key);
        request.set_value(self.value);

        let mut buffer = vec![];
        let mut cursor = std::io::Cursor::new(&mut buffer);
        capnp::serialize::write_message(&mut cursor, &message).map_err(error::Error::Capnp)?;
        Ok(buffer)
    }
}

pub(crate) struct StoreAllResults {
    pub(crate) pairs: Vec<(String, String)>,
}

impl From<store_capnp::store_all_response::Reader<'_>> for StoreAllResults {
    fn from(value: store_capnp::store_all_response::Reader<'_>) -> Self {
        if !value.has_pairs() {
            return StoreAllResults { pairs: vec![] };
        }

        let mut pairs = vec![];
        let returned_pairs = match value.get_pairs() {
            Ok(pairs) => pairs,
            Err(_) => return StoreAllResults { pairs: vec![] },
        };

        for pair in returned_pairs.iter() {
            let key = capnp_get_text!(pair.get_key());
            let value = capnp_get_text!(pair.get_value());

            if key.is_empty() || value.is_empty() {
                continue;
            }

            pairs.push((key.to_string(), value.to_string()));
        }

        StoreAllResults { pairs }
    }
}
