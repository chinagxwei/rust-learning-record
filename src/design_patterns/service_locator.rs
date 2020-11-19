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

