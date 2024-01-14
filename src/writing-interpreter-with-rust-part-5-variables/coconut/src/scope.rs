use std::collections::HashMap;

pub struct Scope {
    store: HashMap<String, u64>,
}

impl Scope {
    pub fn new() -> Self {
        Scope {
            store: HashMap::new(),
        }
    }

    pub fn dec_var(&mut self, id: String, val: u64) -> Option<u64> {
        self.store.insert(id, val)
    }

    pub fn set_var(&mut self, id: String, val: u64) -> Option<u64> {
        self.store.insert(id, val)
    }

    pub fn get_var(&self, id: String) -> Option<&u64> {
        self.store.get(&id.clone())
    }
}

#[cfg(test)]
mod scope_tests {
    use super::Scope;

    #[test]
    fn expected_declare_variable() {
        let mut scope = Scope::new();
        scope.dec_var("x".to_string(), 1);
        assert_eq!(*scope.get_var("x".to_string()).unwrap(), 1);
    }
    #[test]
    fn expected_declare_and_set_variable() {
        let mut scope = Scope::new();
        scope.dec_var("x".to_string(), 1);
        scope.set_var("x".to_string(), 2);
        assert_eq!(*scope.get_var("x".to_string()).unwrap(), 2);
    }
}
