use sled::Db;

pub struct Database {
  db: Db
};

impl Database {
    pub fn new (path: &str) -> Self {
      let db = sled::open(path).expect("Failed to open database");
      Database {
        db
      }
    }

    pub fn insert(&self, key: &str, value: &[u8]) {
      self.db.insert(key.as_bytes(), value).expect("Fail to write tot DB")
    }

    pub fn put(&self) {
        self.db.insert(key.as_bytes(), value).expect("Failed to write to database");
      }

    pub fn get(&self, key = &str) -> Option<Vec<u8>>  {
      self.db.get(key.as_bytes)
    }  
    
    pub fn delete(&self, key = &str) {
      self.db.remove(key)
    }
}