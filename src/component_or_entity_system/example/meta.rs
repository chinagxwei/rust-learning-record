use crate::component_or_entity_system::example::{Component, Class};
use uuid::Uuid;
use crate::component_or_entity_system::example::manager::EntityManager;
use std::rc::Rc;
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

    fn new_by_name(name: String) -> MetaEntity {
        let mut entity = Self::_new();
        entity.internal_name = name;
        entity
    }

    fn new_by_components<T: Component>(components: Vec<T>) -> MetaEntity {
        let mut entity = Self::_new();
        for component in components {
            entity.add(component);
        }
        entity
    }

    fn new_by_name_and_components<T: Component>(name: String, components: Vec<T>) -> MetaEntity {
        let mut entity = Self::new_by_name(name);
        for component in components {
            entity.add(component);
        }
        entity
    }

    fn new_by_uuid(entity: Uuid) -> MetaEntity {
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
    fn has(&self, component_type: &Class) -> bool {
        self.parent_entity_manager.has_component(&self.entity, component_type)
    }

    fn get(&mut self, component_type: &Class) -> Option<Rc<dyn Component>> {
        self.parent_entity_manager.get_component(&self.entity, component_type)
    }

    fn get_all(&self) -> Option<VecDeque<Rc<dyn Component>>> {
        self.parent_entity_manager.get_all_components_on_entity(&self.entity)
    }
}

impl MetaEntity {
    fn add<T: Component>(&mut self, component: T) {
        self.parent_entity_manager.add_component(self.entity, component);
    }

    fn remove(&mut self, component: Rc<dyn Component>) -> Option<Rc<dyn Component>> {
        self.parent_entity_manager.remove_component(&self.entity, component)
    }

    fn remove_all(&mut self) {
        for item in self.get_all().unwrap() {
            self.remove(item);
        }
    }

    fn kill(&mut self) {
        self.parent_entity_manager.kill_entity(&self.entity);
    }
}
use std::fmt::Write as FmtWrite;

impl std::fmt::Display for MetaEntity {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        let mut buff = String::new();
        for item in self.parent_entity_manager.get_all_components_on_entity(&self.entity).unwrap() {
            if buff.len() > 0 {
                write!(&mut buff, "{}", ", ")?;
            }
            write!(&mut buff, "{}", item)?;
        }
        write!(f, "Entity[{}:{}]({})", self.entity, self.internal_name, buff)
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::component_or_entity_system::example::Class;
    use std::fmt::Formatter;

    #[derive(Debug)]
    struct Position {
        x: f32,
        y: f32,
    }

    impl std::fmt::Display for Position {
        fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
            write!(f, "({},{})", self.x, self.y)
        }
    }

    impl Component for Position {
        fn get_type(&self) -> Class {
            Class(String::from("core"))
        }
    }

    impl Position {
        pub fn new(x: f32, y: f32) -> Self {
            Position { x, y }
        }
    }

    #[derive(Debug)]
    struct Position2 {
        local: Position,
        target: Position,
    }

    impl std::fmt::Display for Position2 {
        fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
            write!(f, "({},{})", self.local, self.target)
        }
    }

    impl Component for Position2 {
        fn get_type(&self) -> Class {
            Class(String::from("component"))
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
        println!("{}", p);
        let class = p.get_type();
        let mut meta = MetaEntity::new_by_name(String::from("entity"));
        meta.add(p);
        meta.add(p2);
        println!("{}", meta);
        let a = meta.get(&class).unwrap();
        meta.remove(a);
        println!("{}", meta);
    }
}
