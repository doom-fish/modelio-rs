use crate::object::Object;
use crate::Result;

/// Mirrors the corresponding Model I/O named protocol counterpart.
pub trait Named {
    fn name(&self) -> Option<String>;
    fn set_name(&self, name: &str) -> Result<()>;
}

/// Mirrors the corresponding Model I/O component protocol counterpart.
pub trait Component {}

/// Mirrors the corresponding Model I/O object container component: component protocol counterpart.
pub trait ObjectContainerComponent: Component {
    fn count(&self) -> usize;
    fn object_at(&self, index: usize) -> Option<Object>;

    fn objects(&self) -> Vec<Object> {
        (0..self.count())
            .filter_map(|index| self.object_at(index))
            .collect()
    }

    fn add_object(&self, object: &Object);
    fn remove_object(&self, object: &Object);
}

/// Mirrors the corresponding Model I/O joint animation protocol counterpart.
pub trait JointAnimation {}
