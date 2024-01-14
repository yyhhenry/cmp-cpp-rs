// A real-world example for using traits.
use std::collections::HashSet;

pub trait PersistentOperation {
    type Source;
    fn apply_on(&self, source: &mut Self::Source);
    fn revert_on(&self, source: &mut Self::Source);
}
#[derive(Debug, Clone)]
struct PersistentOperations<Operation>
where
    Operation: PersistentOperation + Clone,
{
    operations: Vec<Operation>,
}
impl<Operation> PersistentOperations<Operation>
where
    Operation: PersistentOperation + Clone,
{
    pub fn new() -> Self {
        Self { operations: vec![] }
    }
    pub fn len(&self) -> usize {
        self.operations.len()
    }
    pub fn add_and_apply(&mut self, operation: Operation, source: &mut Operation::Source) {
        operation.apply_on(source);
        self.operations.push(operation);
    }
    pub fn apply_all(&self, source: &mut Operation::Source) {
        for operation in &self.operations {
            operation.apply_on(source);
        }
    }
    pub fn revert_all(&self, source: &mut Operation::Source) {
        for operation in self.operations.iter().rev() {
            operation.revert_on(source);
        }
    }
}
#[derive(Debug)]
pub enum PersistentErr {
    Empty,
    Uncommitted,
}
pub struct PersistentStruct<Source, Operation>
where
    Operation: PersistentOperation + Clone,
{
    source: Source,
    current: PersistentOperations<Operation>,
    applied: Vec<PersistentOperations<Operation>>,
    reverted: Vec<PersistentOperations<Operation>>,
}
impl<Source, Operation> AsRef<Source> for PersistentStruct<Source, Operation>
where
    Operation: PersistentOperation + Clone,
{
    fn as_ref(&self) -> &Source {
        &self.source
    }
}
impl<Source, Operation> From<Source> for PersistentStruct<Source, Operation>
where
    Operation: PersistentOperation + Clone,
{
    fn from(source: Source) -> Self {
        Self {
            source,
            current: PersistentOperations::new(),
            applied: vec![],
            reverted: vec![],
        }
    }
}
impl<Source, Operation> PersistentStruct<Source, Operation>
where
    Operation: PersistentOperation<Source = Source> + Clone,
{
    pub fn add_and_apply(&mut self, operation: Operation) {
        self.current.add_and_apply(operation, &mut self.source);
    }
    pub fn commit(&mut self) {
        self.applied.push(self.current.clone());
        self.current = PersistentOperations::new();
        self.reverted.clear();
    }
    pub fn clear_uncommitted(&mut self) {
        self.current.revert_all(&mut self.source);
        self.current = PersistentOperations::new();
    }
    pub fn redo(&mut self) -> Result<(), PersistentErr> {
        if self.current.len() > 0 {
            return Err(PersistentErr::Uncommitted);
        }
        let operations = self.reverted.pop().ok_or(PersistentErr::Empty)?;
        operations.apply_all(&mut self.source);
        self.applied.push(operations);
        Ok(())
    }
    pub fn undo(&mut self) -> Result<(), PersistentErr> {
        if self.current.len() > 0 {
            return Err(PersistentErr::Uncommitted);
        }
        let operations = self.applied.pop().ok_or(PersistentErr::Empty)?;
        operations.revert_all(&mut self.source);
        self.reverted.push(operations);
        Ok(())
    }
}
#[derive(Clone)]
pub enum PersistentSetOperation<T>
where
    T: Eq + std::hash::Hash + Clone,
{
    Insert(T),
    Delete(T),
}
impl<T> PersistentOperation for PersistentSetOperation<T>
where
    T: Eq + std::hash::Hash + Clone,
{
    type Source = HashSet<T>;
    fn apply_on(&self, source: &mut Self::Source) {
        match self {
            PersistentSetOperation::Insert(value) => source.insert(value.clone()),
            PersistentSetOperation::Delete(value) => source.remove(value),
        };
    }
    fn revert_on(&self, source: &mut Self::Source) {
        match self {
            PersistentSetOperation::Insert(value) => source.remove(value),
            PersistentSetOperation::Delete(value) => source.insert(value.clone()),
        };
    }
}
fn main() {
    let mut set = PersistentStruct::from(HashSet::new());

    set.add_and_apply(PersistentSetOperation::Insert(1));
    // Apply at once, not until commit.
    assert_eq!(set.as_ref(), &HashSet::from([1]));
    set.add_and_apply(PersistentSetOperation::Insert(2));
    set.commit();
    // Save the current state using commit.
    assert_eq!(set.as_ref(), &HashSet::from([1, 2]));

    set.undo().unwrap();
    // Back to the previous state.
    assert_eq!(set.as_ref(), &HashSet::from([]));

    set.add_and_apply(PersistentSetOperation::Insert(3));
    // Apply at once.
    assert_eq!(set.as_ref(), &HashSet::from([3]));
    set.clear_uncommitted();
    // Then, clear the uncommitted operations.
    assert_eq!(set.as_ref(), &HashSet::from([]));
    set.redo().unwrap();
    // But the uncommitted operations will not influence the redo.
    assert_eq!(set.as_ref(), &HashSet::from([1, 2]));

    println!("All tests passed!")
}
