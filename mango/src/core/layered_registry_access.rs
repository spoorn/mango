use crate::core::registry_access::RegistryAccess;
use dashmap::DashMap;
use std::fmt::Debug;
use std::sync::Arc;

pub struct LayeredRegistryAccess<T: Clone + PartialEq + Debug> {
    keys: Vec<T>,
    values: Vec<RegistryAccess>,
    composite: RegistryAccess,
}
impl<T: Clone + PartialEq + Debug> LayeredRegistryAccess<T> {
    pub fn new(keys: Vec<T>) -> Self {
        let keys_len = keys.len();
        Self::new_with_values(keys, vec![RegistryAccess::empty(); keys_len])
    }

    fn new_with_values(keys: Vec<T>, values: Vec<RegistryAccess>) -> Self {
        let composite = collect_registries(values.iter());
        Self {
            keys,
            values,
            composite,
        }
    }

    fn get_layer_index_or_throw(&self, key: &T) -> usize {
        self.keys
            .iter()
            .position(|k| k == key)
            .unwrap_or_else(|| panic!("Can't find key {:?} inside {:?}", key, self.keys))
    }

    pub fn replace_from(&self, key: T, registries: Vec<RegistryAccess>) -> Self {
        let index = self.get_layer_index_or_throw(&key);
        if registries.len() > self.values.len() - index {
            panic!("Too many values to replace");
        }
        let mut new_values = Vec::with_capacity(self.values.len());
        // This differs from vanilla in that we clone the RegistryAccess values instead of reference
        for i in 0..index {
            new_values.push(self.values.get(i).unwrap().clone())
        }
        new_values.extend(registries);
        while new_values.len() < self.values.len() {
            new_values.push(RegistryAccess::empty());
        }
        Self::new_with_values(self.keys.clone(), new_values)
    }
}

fn collect_registries<'a>(registries: impl Iterator<Item = &'a RegistryAccess>) -> RegistryAccess {
    let mut entries = DashMap::new();
    registries.for_each(|registry| {
        registry.registries.iter().for_each(|entry| {
            if entries
                .insert(entry.key().clone(), Arc::clone(entry.value()))
                .is_some()
            {
                panic!("Duplicate registry: {:?}", entry.key());
            }
        })
    });
    RegistryAccess::new(entries)
}
