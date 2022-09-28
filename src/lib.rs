mod dominio01 {
    pub trait Persistable {
        fn save(&self, id: u32, message: String);
        fn load(&self, id: u32) -> String;
    }

    pub struct Dominio01 {
        pub id: u32,
        pub message: String,
    }

    impl Dominio01 {
        pub fn logic(&mut self, repository: &dyn Persistable) -> String {
            self.message = repository.load(self.id);
            self.message.to_uppercase()
        }
    }
}

#[cfg(test)]
mod tests {
    // Note this useful idiom: importing names from outer (for mod tests) scope.
    use super::dominio01::*;
    use std::collections::HashMap;

    struct DummyPersistable {
        dummy_database: HashMap<u32, String>,
    }
    impl DummyPersistable {
        fn init(&mut self) {
            self.dummy_database = HashMap::new();
            self.dummy_database.insert(1, String::from("hola"));
            self.dummy_database.insert(2, String::from("adios"));
            self.dummy_database.insert(3, String::from("mundo"));
            self.dummy_database.insert(4, String::from("nube"));
        }
    }
    impl Persistable for DummyPersistable {
        fn save(&self, id: u32, message: String) {
            println!("{:?} {:?}", id, message);
        }

        fn load(&self, id: u32) -> String {
            println!("{:?} ", id);
            let result: String = match self.dummy_database.get(&id) {
                Some(m) => m.to_string(),
                None => String::from("NAN"),
            };
            result
        }
    }

    #[test]
    fn test_logic_exists() {
        let mut m01 = Dominio01 {
            id: 1,
            message: String::from(""),
        };
        let mut dummy_repo = DummyPersistable {
            dummy_database: HashMap::new(),
        };
        dummy_repo.init();
        let msg: String = m01.logic(&dummy_repo);
        println!("{:?} ", msg);
        assert_eq!(String::from("HOLA"), msg);
    }

    #[test]
    fn test_logic_doesnt_exists() {
        let mut m01 = Dominio01 {
            id: 100,
            message: String::from(""),
        };
        let mut dummy_repo = DummyPersistable {
            dummy_database: HashMap::new(),
        };
        dummy_repo.init();
        let msg: String = m01.logic(&dummy_repo);
        println!("{:?} ", msg);
        assert_eq!(String::from("NAN"), msg);
    }
}
