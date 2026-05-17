use crate::object::Object;
use crate::Result;

pub trait Named {
    fn name(&self) -> Option<String>;
    fn set_name(&self, name: &str) -> Result<()>;
}

pub trait Component {}

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

pub trait JointAnimation {}
