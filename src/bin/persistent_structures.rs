// A real-world example for using traits.
use std::{collections::HashMap, marker::PhantomData};

pub trait PersistentOperation<Source> {
    fn apply_on(&self, source: &mut Source);
    fn revert_on(&self, source: &mut Source);
}
struct PersistentOperations<Source, Operation>
where
    Operation: PersistentOperation<Source> + Clone,
{
    _source: PhantomData<Source>,
    operations: Vec<Operation>,
}
impl<Source, Operation> Clone for PersistentOperations<Source, Operation>
where
    Operation: PersistentOperation<Source> + Clone,
{
    fn clone(&self) -> Self {
        Self {
            _source: PhantomData,
            operations: self.operations.clone(),
        }
    }
}
impl<Source, Operation> PersistentOperations<Source, Operation>
where
    Operation: PersistentOperation<Source> + Clone,
{
    pub fn new() -> Self {
        Self {
            _source: PhantomData,
            operations: vec![],
        }
    }
    pub fn len(&self) -> usize {
        self.operations.len()
    }
    pub fn add_and_apply(&mut self, operation: Operation, source: &mut Source) {
        operation.apply_on(source);
        self.operations.push(operation);
    }
    pub fn apply_all(&self, source: &mut Source) {
        for operation in &self.operations {
            operation.apply_on(source);
        }
    }
    pub fn revert_all(&self, source: &mut Source) {
        for operation in self.operations.iter().rev() {
            operation.revert_on(source);
        }
    }
}
#[derive(Debug)]
pub enum PersistentErr {
    Empty,
    NotCommitted,
}
pub struct PersistentStruct<Source, Operation>
where
    Operation: PersistentOperation<Source> + Clone,
{
    source: Source,
    current: PersistentOperations<Source, Operation>,
    applied: Vec<PersistentOperations<Source, Operation>>,
    reverted: Vec<PersistentOperations<Source, Operation>>,
}
impl<Source, Operation> PersistentStruct<Source, Operation>
where
    Operation: PersistentOperation<Source> + Clone,
{
    pub fn from(source: Source) -> Self {
        Self {
            source,
            current: PersistentOperations::new(),
            applied: vec![],
            reverted: vec![],
        }
    }
    pub fn as_ref(&self) -> &Source {
        &self.source
    }
    pub fn add_and_apply(&mut self, operation: Operation) {
        self.current.add_and_apply(operation, &mut self.source);
    }
    pub fn commit(&mut self) {
        self.applied.push(self.current.clone());
        self.current = PersistentOperations::new();
        self.reverted.clear();
    }
    pub fn redo(&mut self) -> Result<(), PersistentErr> {
        if self.current.len() > 0 {
            return Err(PersistentErr::NotCommitted);
        }
        let operations = self.reverted.pop().ok_or(PersistentErr::Empty)?;
        operations.apply_all(&mut self.source);
        self.applied.push(operations);
        Ok(())
    }
    pub fn undo(&mut self) -> Result<(), PersistentErr> {
        if self.current.len() > 0 {
            return Err(PersistentErr::NotCommitted);
        }
        let operations = self.applied.pop().ok_or(PersistentErr::Empty)?;
        operations.revert_all(&mut self.source);
        self.reverted.push(operations);
        Ok(())
    }
}
#[derive(Clone)]
pub enum PersistentMapOperation<K, V>
where
    K: Eq + std::hash::Hash + Clone,
    V: Clone,
{
    Insert(K, V),
    Delete(K, V),
}
impl<K, V> PersistentOperation<HashMap<K, V>> for PersistentMapOperation<K, V>
where
    K: Eq + std::hash::Hash + Clone,
    V: Clone,
{
    fn apply_on(&self, source: &mut HashMap<K, V>) {
        match self {
            PersistentMapOperation::Insert(key, value) => source.insert(key.clone(), value.clone()),
            PersistentMapOperation::Delete(key, _) => source.remove(key),
        };
    }
    fn revert_on(&self, source: &mut HashMap<K, V>) {
        match self {
            PersistentMapOperation::Insert(key, _) => source.remove(key),
            PersistentMapOperation::Delete(key, value) => source.insert(key.clone(), value.clone()),
        };
    }
}
fn main() {
    let mut map = PersistentStruct::from(HashMap::new());
    println!("{:?}", map.as_ref());
    map.add_and_apply(PersistentMapOperation::Insert(1, 2));
    map.commit();
    println!("{:?}", map.as_ref());
    map.add_and_apply(PersistentMapOperation::Insert(2, 3));
    map.commit();
    println!("{:?}", map.as_ref());
    map.undo().unwrap();
    println!("{:?}", map.as_ref());
    map.undo().unwrap();
    println!("{:?}", map.as_ref());
    map.redo().unwrap();
    println!("{:?}", map.as_ref());
}
