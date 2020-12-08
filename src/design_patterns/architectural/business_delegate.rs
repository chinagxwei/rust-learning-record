use crate::design_patterns::architectural::business_delegate::ServiceType::EJB;
use std::cell::RefCell;

trait BusinessService {
    fn do_processing(&self);
}

#[derive(Copy, Clone)]
enum ServiceType {
    EJB,
    JMS,
}

impl ServiceType {
    fn as_str(&self) -> &'static str {
        match *self {
            ServiceType::EJB => "EJB",
            ServiceType::JMS => "JMS"
        }
    }
}

struct EJBService;

impl EJBService {
    fn new() -> EJBService {
        EJBService
    }
}

impl BusinessService for EJBService {
    fn do_processing(&self) {
        println!("Processing task by invoking EJB Service")
    }
}

struct JMSService;

impl JMSService {
    fn new() -> JMSService {
        JMSService
    }
}

impl BusinessService for JMSService {
    fn do_processing(&self) {
        println!("Processing task by invoking JMS Service")
    }
}


struct BusinessLookUp;

impl BusinessLookUp {
    fn new() -> BusinessLookUp {
        BusinessLookUp
    }
}

impl BusinessLookUp {
    fn get_business_service(&self, service_type: ServiceType) -> Box<dyn BusinessService> {
        match service_type {
            ServiceType::EJB => Box::new(EJBService::new()),
            ServiceType::JMS => Box::new(JMSService::new())
        }
    }
}

struct BusinessDelegate {
    service_type: RefCell<ServiceType>,
    business_service: RefCell<Option<Box<dyn BusinessService>>>,
    lookup_service: BusinessLookUp,
}

impl BusinessDelegate {
    fn new() -> BusinessDelegate {
        BusinessDelegate {
            service_type: RefCell::new(ServiceType::EJB),
            business_service: RefCell::new(None),
            lookup_service: BusinessLookUp::new(),
        }
    }
}

impl BusinessDelegate {
    fn set_service_type(&self, service_type: ServiceType) {
        *self.service_type.borrow_mut() = service_type;
    }

    fn do_task(&self) {
        *self.business_service.borrow_mut() = Some(self.lookup_service.get_business_service(*self.service_type.borrow()));
        self.business_service.borrow()
            .as_ref()
            .unwrap()
            .do_processing();
    }
}

struct Client<'a> {
    business_service: &'a BusinessDelegate
}

impl<'a> Client<'a> {
    fn new(business_service: &'a BusinessDelegate) -> Client<'a> {
        Client { business_service }
    }
}

impl Client<'_> {
    fn do_task(&self) {
        self.business_service.do_task();
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_business_delegate() {
        let business_delegate = BusinessDelegate::new();
        business_delegate.set_service_type(ServiceType::EJB);

        let client = Client::new(&business_delegate);
        client.do_task();

        business_delegate.set_service_type(ServiceType::JMS);
        client.do_task();
    }
}



