use crate::component_or_entity_system::example::Component;
use std::collections::{VecDeque, HashMap};
use uuid::Uuid;
use std::rc::Rc;
use crate::component_or_entity_system::Class;

#[derive(Debug)]
pub struct EntityManager {
    frozen: bool,
    all_entities: VecDeque<Uuid>,
    entity_human_readable_names: HashMap<Uuid, String>,
    component_stores: HashMap<Class, HashMap<Uuid, Rc<dyn Component>>>,
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
    pub fn get_component(&self, uuid: &Uuid, component_type: &Class) -> Option<Rc<dyn Component>> {
        if self.component_stores.contains_key(component_type) {
             let item = self.component_stores
                .get(component_type)
                .unwrap()
                .get(uuid).unwrap();
            return Some(Rc::clone(item))
        }
        None
    }

    pub fn has_component(&self, uuid: &Uuid, component_type: &Class) -> bool {
        if self.component_stores.contains_key(component_type) {
            return self.component_stores
                .get(component_type)
                .unwrap()
                .contains_key(uuid);
        }
        false
    }

    pub fn get_all_components_on_entity(&self, uuid: &Uuid) -> Option<VecDeque<Rc<dyn Component>>> {
        let mut components = VecDeque::default();
        for item in self.component_stores.values() {
            if item.is_empty() {
                continue;
            }
            if item.get(uuid).is_some() {
                components.push_back(Rc::clone(item.get(uuid).unwrap()))
            }
        }
        if components.is_empty() {
            None
        } else {
            Some(components)
        }
    }

    pub fn get_all_components_of_type(&self, component_type: &Class) -> Option<VecDeque<Rc<dyn Component>>> {
        if self.component_stores.contains_key(component_type) {
            return Some(
                self.component_stores
                    .get(component_type)
                    .unwrap()
                    .values()
                    .map(|x| Rc::clone(x))
                    .collect::<VecDeque<Rc<dyn Component>>>()
            );
        }
        None
    }

    pub fn name_for(&self, uuid: &Uuid) -> Option<&String> {
        self.entity_human_readable_names.get(uuid)
    }
}

impl EntityManager {
    pub fn remove_component(&mut self, uuid: &Uuid, component: Rc<dyn Component>) -> Option<Rc<dyn Component>> {
        if self.component_stores.contains_key(&component.get_type()) {
            return self.component_stores
                .get_mut(&component.get_type())
                .unwrap()
                .remove(uuid);
        }
        None
    }

    pub fn add_component<T:Component>(&mut self, uuid: Uuid, component: T) {
        if self.frozen { return; }
        if self.component_stores.contains_key(&component.get_type()) {
            self.component_stores
                .get_mut(&component.get_type())
                .and_then(|x| x.insert(uuid, Rc::new(component)));
        } else {
            let mut hm = HashMap::<Uuid, Rc<dyn Component>>::new();
            let r#type = component.get_type();
            hm.insert(uuid, Rc::new(component));
            self.component_stores.insert(r#type, hm);
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
        let uuid = Uuid::new_v4();
        self.all_entities.push_back(uuid);
        self.entity_human_readable_names.insert(uuid, name);
        Some(uuid)
    }

    pub fn set_entity(&mut self, uuid: Uuid) {
        if self.frozen { return; }
        self.all_entities.push_back(uuid);
    }

    pub fn set_entity_name(&mut self, uuid: Uuid, name: String) {
        self.set_entity(uuid);
        self.entity_human_readable_names.insert(uuid, name);
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
