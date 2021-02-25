use std::any::{TypeId, Any};
use std::collections::{VecDeque, HashMap};
use uuid::Uuid;
use std::fmt::{Display, Debug};
use crate::component_or_entity_system::example2::Component;
use std::collections::hash_map::RandomState;

#[derive(Debug)]
pub struct EntityManager {
    frozen: bool,
    all_entities: VecDeque<Uuid>,
    entity_human_readable_names: HashMap<Uuid, String>,
    component_stores: HashMap<TypeId, HashMap<Uuid, Box<dyn Any>>>,
}

impl EntityManager {
    pub fn new() -> EntityManager {
        EntityManager {
            frozen: false,
            all_entities: VecDeque::default(),
            entity_human_readable_names: HashMap::default(),
            component_stores: HashMap::default(),
        }
    }
}

impl EntityManager {
    #[allow(map_clone)]
    pub fn get_component<T: Component + Clone>(&self, uuid: &Uuid) -> Option<T> {
        println!("{}", self.component_stores.len());
        self.component_stores
            .get(&TypeId::of::<T>())
            .expect("get_component: type error")
            .get(uuid)
            .map(|x| {
                x.downcast_ref()
                    .expect("get_component: internal downcast error")
            }).map(Clone::clone)
    }

    pub fn has_component_by_type_id(&self, uuid: &Uuid, component_id: &TypeId) -> bool {
        if self.component_stores.contains_key(component_id) {
            return self.component_stores
                .get(component_id)
                .unwrap()
                .contains_key(uuid);
        }
        false
    }

    pub fn has_component<T: Component>(&self, uuid: &Uuid) -> bool {
        self.has_component_by_type_id(uuid, &TypeId::of::<T>())
    }

    #[allow(map_clone)]
    pub fn get_all_components_on_entity<T: Component + Clone>(&self, uuid: &Uuid) -> VecDeque<Option<T>> {
        let mut components = VecDeque::default();
        for item in self.component_stores.values() {
            if item.is_empty() {
                continue;
            }
            if item.get(uuid).is_some() {
                let component = item.get(uuid)
                    .map(|x| x.downcast_ref().expect("get_all_components_on_entity: internal downcast error"))
                    .map(Clone::clone);
                components.push_back(component)
            }
        }
        components
    }

    #[allow(map_clone)]
    pub fn get_all_components_of_type<T: Component + Clone>(&self) -> VecDeque<Option<T>> {
        self.component_stores
            .get(&TypeId::of::<T>())
            .expect("")
            .values()
            .map(|x| x.downcast_ref().expect("get_all_components_on_entity: internal downcast error"))
            .map(Clone::clone)
            .collect::<VecDeque<Option<T>>>()
    }

    pub fn name_for(&self, uuid: &Uuid) -> Option<&String> {
        self.entity_human_readable_names.get(uuid)
    }
}

impl EntityManager {
    pub fn add_component<T: Component>(&mut self, uuid: Uuid, component: T) {
        if self.frozen { return; }
        if self.component_stores.contains_key(&TypeId::of::<T>()) {
            self.component_stores
                .get_mut(&TypeId::of::<T>())
                .and_then(|x| x.insert(uuid, Box::new(component)));
        } else {
            let mut hm = HashMap::<Uuid, Box<dyn Any>>::new();
            hm.insert(uuid, Box::new(component));
            self.component_stores.insert(TypeId::of::<T>(), hm);
        }
    }

    pub fn remove_component<T: Component>(&mut self, uuid: &Uuid) -> Option<T> {
        self.component_stores
            .get_mut(&TypeId::of::<T>())
            .expect("not found data")
            .remove(uuid)
            .map(|old| *old.downcast::<T>().expect("remove_component: internal downcast error"))
    }

    pub fn remove_all_component<T: Component>(&mut self) {
        self.component_stores.remove(&TypeId::of::<T>());
    }

    pub fn kill_entity(&mut self, uuid: &Uuid) {
        for (_, component_store) in self.component_stores.iter_mut() {
            if component_store.is_empty() {
                continue;
            }
            component_store.remove(uuid);
        }

        let mut idx = -1;
        for (i, entity) in self.all_entities.iter().enumerate() {
            if entity == uuid {
                idx = i as i32;
            }
        }
        if idx > -1 {
            self.all_entities.remove(idx as usize);
        }
    }

    pub fn create_entity(&mut self) -> Option<Uuid> {
        if self.frozen { return None; }
        let uuid = Uuid::new_v4();
        self.all_entities.push_back(uuid);
        Some(uuid)
    }

    pub fn create_entity_by_name(&mut self, name: String) -> Option<Uuid> {
        if self.frozen { return None; }
        let uuid = self.create_entity();
        self.all_entities.push_back(uuid.unwrap());
        self.entity_human_readable_names.insert(uuid.unwrap(), name);
        uuid
    }

    pub fn set_entity(&mut self, uuid: Uuid) {
        if self.frozen { return; }
        self.all_entities.push_back(uuid);
    }

    pub fn set_entity_name(&mut self, uuid: Uuid, name: String) {
        self.set_entity(uuid);
        self.entity_human_readable_names.insert(uuid, name);
    }

    fn frozen(&mut self) {
        self.frozen = true;
    }

    fn un_frozen(&mut self) {
        self.frozen = false;
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test() {}
}
