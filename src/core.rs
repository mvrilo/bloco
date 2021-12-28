use crate::{Blob, Hash, Ref, Result};

pub trait Core {
    fn get_blob(&mut self, hash: Hash) -> Result<Blob>;
    fn get_ref_by_name(&mut self, name: String) -> Result<Ref>;
    fn get_ref_by_name_and_bucket(&mut self, name: String, bucket: String) -> Result<Ref>;

    fn put_data(&mut self, data: Vec<u8>, name: String) -> Result<Ref>;
    fn put_ref_in(&mut self, rr: Ref, bucket: String) -> Result<Ref>;
}
