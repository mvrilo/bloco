use crate::{Blob, Hash, Result, Store};
use orion::aead;

#[derive(Debug, Clone, Default)]
pub struct EncryptedStore<S: Store> {
    secret: String,
    store: S,
}

impl<S> EncryptedStore<S>
where
    S: Store,
{
    pub fn new(secret: String, store: S) -> Self {
        EncryptedStore { secret, store }
    }

    fn secret(&mut self) -> aead::SecretKey {
        aead::SecretKey::from_slice(self.secret.as_ref()).unwrap()
    }
}

impl<S> Store for EncryptedStore<S>
where
    S: Store,
{
    fn get(&mut self, hash: Hash) -> Option<Blob> {
        let secret = self.secret();
        self.store
            .get(hash)
            .map(|b| Some(Blob::new(aead::open(&secret, &b.0).unwrap())))
            .unwrap()
    }

    fn put(&mut self, blob: &mut Blob) -> Result<()> {
        let secret = self.secret();
        blob.0 = aead::seal(&secret, &blob.0).unwrap();
        self.store.put(blob).unwrap();
        Ok(())
    }
}
