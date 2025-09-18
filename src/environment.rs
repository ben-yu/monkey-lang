use crate::object::*;
use std::collections::HashMap;
use std::rc::Rc;
use std::cell::RefCell;

pub type Env = Rc<RefCell<Environment>>;


#[derive(Debug, Default, Clone)]
pub struct Environment {
    store: HashMap<String, Rc<Object>>,
}

impl Environment {
    pub fn get(&self, name: &str) -> Option<Rc<Object>> {
        match self.store.get(name) {
            Some(obj) => Some(Rc::clone(obj)),
            None => None
        }
    }

    pub fn set(&mut self, name: String, val: Rc<Object>) {
        self.store.insert(name, val);
    }
}
