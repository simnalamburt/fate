use std::collections::HashMap;
use std::marker::PhantomData;
use std::rc::Rc;
use std::sync::atomic::{AtomicUsize, Ordering};

pub type Id = usize;

pub trait Item<Param> {
    fn new(id: &Id, param: &Param) -> Self;
}

pub struct Manager<I, P>
where
    I: Item<P>,
{
    next_id: AtomicUsize,
    items: HashMap<Id, Rc<I>>,
    _param_type: PhantomData<P>,
}

impl<I, P> Manager<I, P>
where
    I: Item<P>,
{
    pub fn new() -> Self {
        Manager {
            next_id: AtomicUsize::new(0),
            items: HashMap::new(),
            _param_type: PhantomData,
        }
    }

    pub fn create(&mut self, param: &P) -> Rc<I> {
        let id = self.next_id.fetch_add(1, Ordering::Relaxed);
        debug_assert!(!self.items.contains_key(&id));

        let item = Rc::new(I::new(&id, param));
        self.items.insert(id, item.clone());
        item
    }

    pub fn get(&self, id: Id) -> Option<&Rc<I>> {
        self.items.get(&id)
    }
}

impl<I, P> Default for Manager<I, P>
where
    I: Item<P>,
{
    fn default() -> Self {
        Self::new()
    }
}

#[cfg(test)]
mod test {
    use super::{Id, Item, Manager};

    #[derive(Clone)]
    struct TestItem {
        pub id: Id,
        pub a: i32,
        pub b: bool,
    }

    impl Item<(i32, bool)> for TestItem {
        fn new(id: &Id, param: &(i32, bool)) -> Self {
            TestItem {
                id: *id,
                a: param.0,
                b: param.1,
            }
        }
    }

    type TestManager = Manager<TestItem, (i32, bool)>;

    #[test]
    fn manager_create_item_starts_id_with_zero() {
        let mut manager = TestManager::new();

        let item = manager.create(&(1, true));

        assert_eq!(item.id, 0);
        assert_eq!(item.a, 1);
        assert_eq!(item.b, true);
    }

    #[test]
    fn get_method_returns_none_on_empty_manager() {
        let manager = TestManager::new();

        let item = manager.get(1);
        assert!(item.is_none());
    }

    /// This test case depends on that manager generates id sequentially inceasing.
    #[test]
    fn get_method_returns_none_with_uncreated_id() {
        let mut manager = TestManager::new();

        let item = manager.create(&(1, true));
        assert_eq!(item.id, 0);
        assert_eq!(item.a, 1);
        assert_eq!(item.b, true);

        let item = manager.create(&(3, false));
        assert_eq!(item.id, 1);
        assert_eq!(item.a, 3);
        assert_eq!(item.b, false);

        let item = manager.get(4);
        assert!(item.is_none());
    }

    #[test]
    fn get_method_returns_some() {
        let mut manager = TestManager::new();

        let item = manager.create(&(1, true));
        assert_eq!(item.id, 0);
        assert_eq!(item.a, 1);
        assert_eq!(item.b, true);

        let item = manager.get(item.id).unwrap();
        assert_eq!(item.id, 0);
        assert_eq!(item.a, 1);
        assert_eq!(item.b, true);
    }
}
