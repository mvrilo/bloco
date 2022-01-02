use crate::{Blob, Hash, Result, Store};
use async_trait::async_trait;
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

#[async_trait]
impl<S> Store for EncryptedStore<S>
where
    S: Store,
{
    async fn get(&mut self, hash: Hash) -> Result<Blob> {
        let secret = self.secret();
        self.store
            .get(hash)
            .await
            .map(|b| Blob::new(aead::open(&secret, &b.0).unwrap()))
    }

    async fn put(&mut self, blob: &mut Blob) -> Result<Hash> {
        let secret = self.secret();
        blob.0 = aead::seal(&secret, &blob.0).unwrap();
        Ok(self.store.put(blob).await?)
    }
}
