// https://github.com/AndyBarron/rustic-ecs/blob/master/src/lib.rs
//

use std::any::{TypeId, Any};
use std::collections::{HashMap, HashSet};

type IdNumber = u64;

#[derive(Clone, Copy, PartialEq, Eq, Hash, Debug)]
pub struct EntityId(IdNumber);

#[derive(Debug, PartialEq, Eq)]
pub enum NotFound {
    Entity(EntityId),
    Component(TypeId),
}

pub type EcsResult<T> = Result<T, NotFound>;

pub trait Component: Any {}
impl<T: Any> Component for T {}

#[derive(Default, PartialEq, Eq, Debug, Clone)]
pub struct ComponentFilter {
    set: HashSet<TypeId>,
}

#[allow(dead_code)]
impl ComponentFilter {
    pub fn new() -> Self {
        Default::default()
    }

    pub fn add<C: Component>(&mut self) {
        self.set.insert(TypeId::of::<C>());
    }

    pub fn remove<C: Component>(&mut self) {
        self.set.remove(&TypeId::of::<C>());
    }

    pub fn contains<C: Component>(&mut self) -> bool {
        self.set.contains(&TypeId::of::<C>())
    }

    pub fn from_slice(slice: &[TypeId]) -> Self {
        let mut this = Self::new();
        for type_id in slice.iter() {
            this.set.insert(*type_id);
        }
        this
    }

    pub fn iter<'a>(&'a self) -> Box<Iterator<Item = TypeId> + 'a> {
        Box::new(self.set.iter().cloned())
    }
}

#[macro_export]
macro_rules! create_component_filter {
  ($($x:ty),*) => (
    $crate::ComponentFilter::from_slice(
      &vec![$(std::any::TypeId::of::<$x>()),*]
    )
  );
}

#[derive(Default)]
pub struct EntityManager {
    next_id: IdNumber,
    data: HashMap<EntityId, ComponentMap>,
}

#[derive(Default, Debug)]
pub struct ComponentMap {
    map: HashMap<TypeId, Box<Any>>,
}

#[allow(dead_code)]
impl ComponentMap {
    fn set<C: Component>(&mut self, component: C) -> Option<C> {
        self.map
            .insert(TypeId::of::<C>(), Box::new(component))
            .map(|old| *old.downcast::<C>().expect("ComponentMap.set: internal downcast error"))
    }

    fn remove<C: Component>(&mut self) -> EcsResult<()> {
        let _ = self.map.remove(&TypeId::of::<C>());
        Ok(())
    }

    fn borrow<C: Component>(&self) -> EcsResult<&C> {
        self.map
            .get(&TypeId::of::<C>())
            .map(|c| {
                c.downcast_ref()
                 .expect("ComponentMap.borrow: internal downcast error")
            })
            .ok_or_else(|| NotFound::Component(TypeId::of::<C>()))
    }

    fn get<C: Component + Clone>(&self) -> EcsResult<C> {
        self.borrow::<C>()
            .map(Clone::clone)
    }

    fn contains_type_id(&self, id: &TypeId) -> bool {
        self.map.contains_key(id)
    }

    fn contains<C: Component>(&self) -> bool {
        self.contains_type_id(&TypeId::of::<C>())
    }

    fn borrow_mut<C: Component>(&mut self) -> EcsResult<&mut C> {
        match self.map.get_mut(&TypeId::of::<C>()) {
            Some(c) => {
                Ok(c.downcast_mut()
                    .expect("ComponentMap.borrow_mut: internal downcast error"))
            }
            None => Err(NotFound::Component(TypeId::of::<C>())),
        }
    }
}

#[allow(dead_code)]
impl EntityManager {
    pub fn new() -> Self {
        EntityManager {
            next_id: 0,
            data: Default::default()
        }
    }

    pub fn create_entity(&mut self) -> EntityId {
        let new_id = EntityId(self.next_id);
        self.next_id += 1;
        self.data.insert(new_id, Default::default());
        new_id
    }

    pub fn exists(&self, id: EntityId) -> bool {
        self.data.contains_key(&id)
    }

    pub fn destroy_entity(&mut self, id: EntityId) -> EcsResult<()> {
        self.data.remove(&id).map(|_| ()).ok_or_else(|| NotFound::Entity(id))
    }

    pub fn set<C: Component>(&mut self, id: EntityId, comp: C) -> EcsResult<Option<C>> {
        self.data
            .get_mut(&id)
            .ok_or_else(|| NotFound::Entity(id))
            .map(|map| map.set(comp))
    }

    pub fn get<C: Component + Clone>(&self, id: EntityId) -> EcsResult<C> {
        self.data
            .get(&id)
            .ok_or_else(|| NotFound::Entity(id))
            .and_then(|map| map.get())
    }

    pub fn remove<C: Component>(&mut self, id: EntityId) -> EcsResult<()> {
        self.data
            .get_mut(&id)
            .ok_or_else(|| NotFound::Entity(id))
            .and_then(|map| map.remove::<C>())
    }

    pub fn has<C: Component>(&self, id: EntityId) -> bool {
        if let Ok(contains) = self.data
            .get(&id)
            .ok_or_else(|| NotFound::Entity(id))
            .map(|map| map.contains::<C>()) {
            contains
        } else {
            false
        }
    }

    pub fn has_all(&self, id: EntityId, set: &ComponentFilter) -> EcsResult<bool> {
        let map = try!(self.data.get(&id).ok_or_else(|| NotFound::Entity(id)));
        Ok(set.iter().all(|type_id| map.contains_type_id(&type_id)))
    }

    pub fn borrow<C: Component>(&self, id: EntityId) -> EcsResult<&C> {
        self.data
            .get(&id)
            .ok_or_else(|| NotFound::Entity(id))
            .and_then(|map| map.borrow())
    }

    pub fn borrow_mut<C: Component>(&mut self, id: EntityId) -> EcsResult<&mut C> {
        self.data
            .get_mut(&id)
            .ok_or_else(|| NotFound::Entity(id))
            .and_then(|map| map.borrow_mut())
    }

    pub fn iter<'a>(&'a self) -> Box<Iterator<Item = EntityId> + 'a> {
        Box::new(self.data.keys().cloned())
    }

    pub fn collect(&self, dest: &mut Vec<EntityId>) {
        dest.clear();
        dest.extend(self.iter());
    }

    pub fn collect_with<'a>(&'a self, components: &'a ComponentFilter, dest: &mut Vec<EntityId>) {
        let ids = self.data.keys().cloned();
        dest.clear();
        dest.extend(ids.filter(|e| {
            self.has_all(*e, components)
                .expect("Ecs.collect_with: internal id filter error")
        }))
    }
}
