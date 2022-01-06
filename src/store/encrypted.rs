use crate::{Blob, Hash, Result, Store};
use async_trait::async_trait;
use bytes::{buf::BufMut, BytesMut};
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
        self.store.get(hash).await.map(|b| {
            let buf = &mut BytesMut::new();
            let sealed = aead::seal(&secret, &b.0).unwrap();
            buf.put(&*sealed);
            buf.clone().into()
        })
    }

    async fn put(&mut self, blob: &mut Blob) -> Result<Hash> {
        let secret = self.secret();
        let buf = &mut BytesMut::new();
        let sealed = aead::seal(&secret, &blob.0)?;
        buf.put(&*sealed);
        blob.0 = buf.clone();
        Ok(self.store.put(blob.into()).await?)
    }
}
