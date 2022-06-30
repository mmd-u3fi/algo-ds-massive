pub struct HashTable<K, V>
    where K: PartialEq{
    hash_function: fn(&K) -> usize,
    array: Vec<Option<KeyValuePair<K, V>>>,
}

impl <K, V> HashTable<K, V> 
    where K: PartialEq {
    pub fn new(hash_function: fn(&K)->usize, size: usize) -> HashTable<K, V>
    {
        let mut table = Vec::with_capacity(size);
        for _ in 0..size {
            table.push(None);
        }
        HashTable{
            hash_function,
            array: table,
        }
    }

    pub fn insert(&mut self, key: K, value: V) -> Result<(), HashTableErrors>{
        let index = (self.hash_function)(&key);
        if index > self.array.capacity() {
            return Err(HashTableErrors::HashFunctionRange);
        }
        let slot = &self.array[index];
        match slot {
            None => {
                self.array[index] = Some(KeyValuePair::new(key, value));
                return Ok(());
            },
            Some(x) if x.key == key => {
                self.array[index].as_mut().unwrap().value = value;
                return Ok(());
            },
            Some(_) => {
                let empty_index = self.collision_resolution(index)?;
                self.array[empty_index]= Some(KeyValuePair::new(key, value));
                return Ok(());
            },
        };
    }

    fn collision_resolution(&self, index: usize) -> Result<usize, HashTableErrors>{
        for (index, item) in self.array.iter().enumerate().skip(index){
            match item {
                None => {return Ok(index as usize)},
                _ => continue,
            };
        }
        for i in 0..index {
            match self.array[i] {
                None => {return Ok(i as usize)},
                _ => continue,
            };
        }
        Err(HashTableErrors::IsFull)
    }

    pub fn lookup(&self, key: K) -> Option<&V> {
        let index = (self.hash_function)(&key);
        for (index, item) in self.array.iter().enumerate().skip(index){
            match item {
                Some(x) if x.key == key => {
                    return Some(&self.array[index].as_ref().unwrap().value);
                },
                _ => {},
            };
        }
        for (index, item) in self.array.iter().enumerate() {
            match item {
                Some(x) if x.key == key => {
                    return Some(&self.array[index].as_ref().unwrap().value);
                },
                _ => {},
            };
        }
        None
    }
}
pub enum HashTableErrors{
    IsFull,
    HashFunctionRange,
}


#[derive(PartialEq)]
struct KeyValuePair<K, V> 
    where K: PartialEq{
    pub key: K,
    pub value: V,
}

impl <K, V> KeyValuePair<K, V>
where K: PartialEq{
    pub fn new(key: K, value: V) -> KeyValuePair<K, V>{
        KeyValuePair{
            key,
            value,
        }
    }
}

#[cfg(test)]
mod tests {
    use super::{HashTable};
    #[test]
    fn insert_no_collision() {
        let mut table = HashTable::<_, &'static str>::new(|x| (x % 8) as usize, 8);
        assert!(table.insert(12, "hi").is_ok());
        assert!(table.array[4].as_ref().unwrap().key == 12);
        assert!(table.array[4].as_ref().unwrap().value == "hi");
    }
    #[test]
    fn insert_with_collision() {
        let mut table = HashTable::<_, &'static str>::new(|x| (x % 8) as usize, 8);
        assert!(table.insert(12, "hi").is_ok());
        assert!(table.insert(20, "bye").is_ok());
        assert!(table.array[4].as_ref().unwrap().key == 12);
        assert!(table.array[4].as_ref().unwrap().value == "hi");
        assert!(table.array[5].as_ref().unwrap().key == 20);
        assert!(table.array[5].as_ref().unwrap().value == "bye");
    }
    #[test]
    fn lookup() {
        let mut table = HashTable::<_, &'static str>::new(|x| (x % 8) as usize, 8);
        assert!(table.insert(12, "hi").is_ok());
        assert!(table.insert(20, "bye").is_ok());
        assert!(table.lookup(12).unwrap() == &"hi");
        assert!(table.lookup(20).unwrap() == &"bye");
    }
}