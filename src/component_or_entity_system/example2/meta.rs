use uuid::Uuid;
use crate::component_or_entity_system::example2::manager::EntityManager;
use crate::component_or_entity_system::example2::Component;
use std::any::TypeId;
use std::collections::VecDeque;
use std::fmt::Formatter;

#[derive(Debug)]
struct MetaEntity {
    entity: Uuid,
    parent_entity_manager: EntityManager,
    internal_name: String,
}

impl MetaEntity {
    fn _new() -> MetaEntity {
        let mut manager = EntityManager::new();
        let entity = manager.create_entity().unwrap();
        MetaEntity {
            entity,
            parent_entity_manager: manager,
            internal_name: "".to_string(),
        }
    }

    pub fn new_by_name(name: String) -> MetaEntity {
        let mut entity = Self::_new();
        entity.internal_name = name;
        entity
    }

    pub fn new_by_components<T: Component + Clone>(components: Vec<T>) -> MetaEntity {
        let mut entity = Self::_new();
        for component in components {
            entity.add(component);
        }
        entity
    }

    pub fn new_by_name_and_components<T: Component + Clone>(name: String, components: Vec<T>) -> MetaEntity {
        let mut entity = Self::new_by_name(name);
        for component in components {
            entity.add(component);
        }
        entity
    }

    pub fn new_by_uuid(entity: Uuid) -> MetaEntity {
        let mut manager = EntityManager::new();
        manager.set_entity(entity);
        MetaEntity {
            entity,
            parent_entity_manager: manager,
            internal_name: "".to_string(),
        }
    }

    pub fn load_from_entity_manager(entity: Uuid) -> MetaEntity {
        Self::new_by_uuid(entity)
    }
}

impl MetaEntity {
    fn has<T: Component>(&self) -> bool {
        self.parent_entity_manager.has_component::<T>(&self.entity)
    }

    fn get<T: Component + Clone>(&self) -> Option<T> {
        self.parent_entity_manager.get_component::<T>(&self.entity)
    }

    fn get_all<T: Component + Clone>(&self) -> VecDeque<Option<T>> {
        self.parent_entity_manager.get_all_components_on_entity::<T>(&self.entity)
    }
}

impl MetaEntity {
    fn add<T: Component>(&mut self, component: T) {
        self.parent_entity_manager.add_component(self.entity, component);
    }

    fn remove<T: Component>(&mut self) -> Option<T> {
        self.parent_entity_manager.remove_component::<T>(&self.entity)
    }

    fn remove_all<T: Component>(&mut self) {
        self.parent_entity_manager.remove_all_component::<T>()
    }

    fn kill(&mut self) {
        self.parent_entity_manager.kill_entity(&self.entity);
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use std::any::Any;

    #[derive(Debug, Clone)]
    struct Position {
        x: f32,
        y: f32,
    }

    impl std::fmt::Display for Position {
        fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
            write!(f, "({},{})", self.x, self.y)
        }
    }

    impl Position {
        pub fn new(x: f32, y: f32) -> Self {
            Position { x, y }
        }
    }

    #[derive(Debug, Clone)]
    struct Position2 {
        local: Position,
        target: Position,
    }

    impl std::fmt::Display for Position2 {
        fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
            write!(f, "({},{})", self.local, self.target)
        }
    }

    impl Position2 {
        pub fn new(local: Position, target: Position) -> Self {
            Position2 { local, target }
        }
    }

    #[test]
    fn test() {
        let p = Position::new(0.1, 0.2);
        let p2 = Position2::new(Position::new(0.3, 0.4), Position::new(0.5, 0.6));
        let mut meta = MetaEntity::new_by_name(String::from("entity"));
        meta.add(p);
        meta.add(p2);
        println!("{:#?}", meta);
    }
}
