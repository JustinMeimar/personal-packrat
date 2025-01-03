use std::fmt;
use sled::Db;
use uuid::Uuid;
use serde::{Serialize, Deserialize};
use chrono::{DateTime, Utc, Local};
use std::sync::{Mutex, LazyLock};
use std::fmt::Display;
use crate::model::task::Task;
use crate::model::task_entry::TaskEntry;
use crate::model::convert::BytesConvertible;

pub struct TaskManager {
    db: sled::Db,
}

impl TaskManager {
    
    /// Create a DB
    pub fn new(db_path: &str) -> Self {
        TaskManager {
            db: sled::open(db_path).unwrap()
        }
    }

    /// Singleton instance of TaskManager
    pub fn instance() -> &'static Mutex<TaskManager> {
        static INSTANCE: LazyLock<Mutex<TaskManager>> = LazyLock::new(|| {
            let db_path = "./scratch/patrack.db";
            Mutex::new(TaskManager::new(db_path))
        });

        &INSTANCE
    }

    /// Create a task in the DB
    pub fn create_task(&self, name: String, desc: String) -> Task {
        let task = Task::new(name, desc); 
        let key = format!("task:{}", task.id);
        self.db.insert(key, task.to_bytes()).unwrap();
        task
    }
    
    pub fn create_task_entry(&self, task_id: Uuid, content: Vec<u8>) -> TaskEntry {
        let task_entry = TaskEntry::new(task_id, content);
        let key = format!("task_entry:{}:{}", task_id, task_entry.id);
        self.db.insert(key, task_entry.to_bytes()).unwrap();
        task_entry 
    }

    ///
    pub fn get_task(&self, task_id: Uuid) -> Option<Task> {

        self._get(format!("task:{}", task_id))
    }
    
    ///
    pub fn get_tasks(&self) -> Vec<Task> {
        
        self._get_prefix(String::from("task:"))
    }

    ///
    pub fn get_task_entry(&self, task_id: Uuid, entry_id: Uuid) -> Option<TaskEntry> {

        self._get(format!("task_entry:{}:{}", task_id, entry_id))
    }
    
    ///
    pub fn get_task_entries(&self, task_id: Uuid) -> Vec<TaskEntry> {

        self._get_prefix(format!("task_entry:{}", task_id))
    }
    
    ///
    pub fn get_all_entries(&self) -> Vec<TaskEntry> {

        self._get_prefix(String::from("task_entry")) 
    }
    
    ///
    pub fn truncate(&self) {
        self.db.clear().unwrap();
        self.db.flush().unwrap();
    }

    ///
    pub fn debug_dump(&self) {
        for entry in self.db.iter() {
            if let Ok((key, value)) = entry {
                let key_str = String::from_utf8_lossy(&key);
                let value_str = String::from_utf8_lossy(&value);
                println!("Key: {}, Value: {}", key_str, value_str);
            }
        }
    }

    ///////////////////////////////////////////////////////
    /// Private
    ///////////////////////////////////////////////////////
     
    fn _get<T>(&self, key: String) -> Option<T> 
    where
        T: BytesConvertible
    {
        self.db
            .get(key)
            .ok()
            .flatten()
            .map(|data| T::from_bytes(&data))
    }
     
    fn _get_prefix<T>(&self, prefix: String) -> Vec<T> 
    where
        T: BytesConvertible
    {
        self.db
            .scan_prefix(prefix)
            .filter_map(|x| x.ok()) // only take some values
            .map(|(_k, v)| T::from_bytes(&v))
            .collect()
    }
}
