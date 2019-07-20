use crate::{Backend, ReadBackend, WriteBackend, EmptyBackend, Construct, ValueOf};
use core::hash::Hash;
use std::collections::{HashMap, HashSet};

/// Proving merkle database.
pub struct ProvingBackend<'a, DB: Backend> {
    db: &'a mut DB,
    proofs: HashMap<<DB::Construct as Construct>::Intermediate, (ValueOf<DB::Construct>, ValueOf<DB::Construct>)>,
    inserts: HashSet<<DB::Construct as Construct>::Intermediate>,
}

impl<'a, DB: Backend> ProvingBackend<'a, DB> where
    <DB::Construct as Construct>::Intermediate: Eq + Hash,
{
    /// Create a new proving database.
    pub fn new(db: &'a mut DB) -> Self {
        Self {
            db,
            proofs: Default::default(),
            inserts: Default::default(),
        }
    }

    /// Reset the proving database and get all the proofs.
    pub fn reset(&mut self) -> HashMap<<DB::Construct as Construct>::Intermediate, (ValueOf<DB::Construct>, ValueOf<DB::Construct>)> {
        let proofs = self.proofs.clone();
        self.proofs = Default::default();
        self.inserts = Default::default();
        proofs
    }
}

impl<'a, DB: Backend> Backend for ProvingBackend<'a, DB> {
    type Construct = DB::Construct;
    type Error = DB::Error;
}

impl<'a, DB: ReadBackend> ReadBackend for ProvingBackend<'a, DB> where
    <DB::Construct as Construct>::Intermediate: Eq + Hash,
{
    fn get(
        &mut self,
        key: &<DB::Construct as Construct>::Intermediate
    ) -> Result<(ValueOf<DB::Construct>, ValueOf<DB::Construct>), Self::Error> {
        let value = self.db.get(key)?;
        if !self.inserts.contains(key) {
            self.proofs.insert(key.clone(), value.clone());
        }
        Ok(value)
    }
}

impl<'a, DB: WriteBackend> WriteBackend for ProvingBackend<'a, DB> where
    <DB::Construct as Construct>::Intermediate: Eq + Hash,
{
    fn rootify(&mut self, key: &<DB::Construct as Construct>::Intermediate) -> Result<(), Self::Error> {
        self.db.rootify(key)
    }

    fn unrootify(&mut self, key: &<DB::Construct as Construct>::Intermediate) -> Result<(), Self::Error> {
        self.db.unrootify(key)
    }

    fn insert(
        &mut self,
        key: <DB::Construct as Construct>::Intermediate,
        value: (ValueOf<DB::Construct>, ValueOf<DB::Construct>)
    ) -> Result<(), Self::Error> {
        self.inserts.insert(key.clone());
        self.db.insert(key, value)
    }
}

impl<'a, DB: EmptyBackend> EmptyBackend for ProvingBackend<'a, DB> where
    <DB::Construct as Construct>::Intermediate: Eq + Hash
{
    fn empty_at(&mut self, depth_to_bottom: usize) -> Result<ValueOf<DB::Construct>, Self::Error> {
        self.db.empty_at(depth_to_bottom)
    }
}
