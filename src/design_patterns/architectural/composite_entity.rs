struct DependentObject1 {
    data: Option<&'static str>
}

impl DependentObject1 {
    fn new() -> DependentObject1 {
        DependentObject1 { data: None }
    }
}

impl DependentObject1 {
    fn set_data(&mut self, data: &'static str) {
        self.data = Some(data)
    }

    fn get_data(&self) -> Option<&'static str> {
        self.data
    }
}

struct DependentObject2 {
    data: Option<&'static str>
}

impl DependentObject2 {
    fn new() -> DependentObject2 {
        DependentObject2 { data: None }
    }
}

impl DependentObject2 {
    fn set_data(&mut self, data: &'static str) {
        self.data = Some(data)
    }

    fn get_data(&self) -> Option<&'static str> {
        self.data
    }
}

struct CoarseGrainedObject {
    do_1: DependentObject1,
    do_2: DependentObject2,
}

impl CoarseGrainedObject {
    fn new() -> CoarseGrainedObject {
        CoarseGrainedObject { do_1: DependentObject1::new(), do_2: DependentObject2::new() }
    }
}

impl CoarseGrainedObject {
    fn set_data(&mut self, data1: &'static str, data2: &'static str) {
        self.do_1.set_data(data1);
        self.do_2.set_data(data2);
    }

    fn get_data(&self) -> Vec<Option<&'static str>> {
        vec![self.do_1.get_data().clone(), self.do_2.get_data().clone()]
    }
}

struct CompositeEntity {
    cgo: CoarseGrainedObject
}

impl CompositeEntity {
    fn new() -> CompositeEntity {
        CompositeEntity { cgo: CoarseGrainedObject::new() }
    }
}

impl CompositeEntity {
    fn set_data(&mut self, data1: &'static str, data2: &'static str) {
        self.cgo.set_data(data1, data2);
    }

    fn get_data(&self) -> Vec<Option<&'static str>> {
        self.cgo.get_data()
    }
}

struct Client {
    composite_entity: CompositeEntity
}

impl Client {
    fn new() -> Client {
        Client { composite_entity: CompositeEntity::new() }
    }
}

impl Client {
    fn set_data(&mut self, data1: &'static str, data2: &'static str) {
        self.composite_entity.set_data(data1, data2);
    }

    fn print(&self) {
        for i in self.composite_entity.get_data() {
            println!("Data: {}", i.unwrap());
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    
    #[test]
    fn test_composite_entity() {
        let mut client = Client::new();

        client.set_data("Test","Data");
        client.print();

        client.set_data("Second Test","Data2");
        client.print();
    }
}

