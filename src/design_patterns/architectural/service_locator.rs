//!
//! 服务定位器模式（Service Locator Pattern）用在我们想使用 JNDI 查询定位各种服务的时候。
//! 考虑到为某个服务查找 JNDI 的代价很高，服务定位器模式充分利用了缓存技术。
//! 在首次请求某个服务时，服务定位器在 JNDI 中查找服务，并缓存该服务对象。
//! 当再次请求相同的服务时，服务定位器会在它的缓存中查找，这样可以在很大程度上提高应用程序的性能。
//! 以下是这种设计模式的实体。
//!
//! 服务（Service） - 实际处理请求的服务。对这种服务的引用可以在 JNDI 服务器中查找到。
//! Context / 初始的 Context - JNDI Context 带有对要查找的服务的引用。
//! 服务定位器（Service Locator） - 服务定位器是通过 JNDI 查找和缓存服务来获取服务的单点接触。
//! 缓存（Cache） - 缓存存储服务的引用，以便复用它们。
//! 客户端（Client） - Client 是通过 ServiceLocator 调用服务的对象。
//!

use std::cell::RefCell;
use std::rc::Rc;

trait Service {
    fn execute(&self);
    fn get_name(&self) -> String;
}

struct ServiceOne;

impl ServiceOne {
    fn new() -> ServiceOne { ServiceOne }
}

impl Service for ServiceOne {
    fn execute(&self) {
        println!("Execute service 1")
    }

    fn get_name(&self) -> String {
        String::from("service_1")
    }
}

struct ServiceTwo;

impl ServiceTwo {
    fn new() -> ServiceTwo { ServiceTwo }
}

impl Service for ServiceTwo {
    fn execute(&self) {
        println!("Execute service 2")
    }

    fn get_name(&self) -> String {
        String::from("service_2")
    }
}

struct InitialContext;

impl InitialContext {
    fn lookup(jndi_name: String) -> Option<Rc<dyn Service>> {
        if jndi_name == String::from("service_1") {
            println!("Looking up and creating a new Service1 object");
            return Some(Rc::new(ServiceOne::new()));
        } else if jndi_name == String::from("service_2") {
            println!("Looking up and creating a new Service2 object");
            return Some(Rc::new(ServiceTwo::new()));
        }
        None
    }
}

struct Cache {
    services: Vec<Rc<dyn Service>>
}

impl Cache {
    fn new() -> Cache {
        Cache { services: vec![] }
    }
}

impl Cache {
    fn add_service(&mut self, service: Rc<dyn Service>) {
        let mut exists = false;

        for item in self.services.iter() {
            if &(item.get_name()) == &service.get_name() {
                exists = true;
            }
        }

        if !exists {
            self.services.push(service)
        }
    }

    fn get_service(&self, name: String) -> Option<Rc<dyn Service>> {
        for item in self.services.iter() {
            if &(item.get_name()) == &name {
                println!("Returning cached {} object", item.get_name());
                return Some(item.clone());
            }
        }
        None
    }
}

struct ServiceLocator {
    cache: RefCell<Cache>
}

impl ServiceLocator {
    fn new(cache: Cache) -> ServiceLocator {
        ServiceLocator { cache: RefCell::new(cache) }
    }
}

impl ServiceLocator {
    fn get_service(&self, name: String) -> Option<Rc<dyn Service>> {
        {
            let service = self.cache.borrow().get_service(name.to_owned());

            if service.is_some() {
                return service;
            }
        }

        let s = InitialContext::lookup(name.to_owned());

        {
            self.cache.borrow_mut().services.push(s.unwrap());
        }

        self.cache.borrow().get_service(name)
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_service_locator() {
        let local = ServiceLocator::new(Cache::new());
        let s1 = local.get_service(String::from("service_1"));
        s1.unwrap().execute();
        let s2 = local.get_service(String::from("service_2"));
        s2.unwrap().execute();
        let s1 = local.get_service(String::from("service_1"));
        s1.unwrap().execute();
        let s2 = local.get_service(String::from("service_2"));
        s2.unwrap().execute();
    }
}

