use sled::Db;
use uuid::Uuid;
use serde::{Serialize, Deserialize};
use chrono::{DateTime, Utc, Local};

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
   
    /// Create a task in the DB
    pub fn create_task(&self, name: String, desc: String) -> Task {
        let task = Task::new(name, desc); 
        let key = format!("task:{}", task.id);
        self.db.insert(key, task.to_bytes()).unwrap();
        task
    }
    
    ///
    pub fn get_task(&self, task_id: Uuid) -> Option<Task> {

        self._get(format!("task:{}", task_id))
    }
    
    ///
    pub fn get_tasks(&self) -> Vec<Task> {
        
        self._get_prefix(String::from("task"))
    }

    ///
    pub fn get_task_entry(&self, task_id: Uuid, entry_id: Uuid) -> Option<TaskEntry> {

        self._get(format!("task_entry:{}:{}", task_id, entry_id))
    }

    pub fn get_task_entries(&self) -> Vec<TaskEntry> {

        self._get_prefix(String::from("task_entry")) 
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

pub trait BytesConvertible: Sized {
    fn from_bytes(bytes: &[u8]) -> Self;
    fn to_bytes(&self) -> Vec<u8>;
}

#[derive(Serialize, Deserialize, PartialEq, Eq, Debug)]
pub struct Task {
    pub id: uuid::Uuid,
    pub name: String,
    pub desc: String,
}

impl Task {
    
    pub fn new(name: String, desc: String) -> Self {
        Task { id: Uuid::new_v4(), name, desc, }
    } 
}

impl BytesConvertible for Task {
    
    fn to_bytes(&self) -> Vec<u8> {
        serde_json::to_vec(self).unwrap() 
    }

    fn from_bytes(bytes: &[u8]) -> Self {
        serde_json::from_slice(bytes).unwrap()
    }
}

#[derive(Serialize, Deserialize, PartialEq, Eq, Debug)]
pub struct TaskEntry {
    id: uuid::Uuid,
    task_id: uuid::Uuid,
    content: String,
    timestamp: DateTime<Utc>,
}

impl TaskEntry {
     
    pub fn new(task_id: Uuid, content: String) -> Self {
        TaskEntry {
            id: Uuid::new_v4(),
            task_id,
            content,
            timestamp: Utc::now(), 
        }
    }
}

impl BytesConvertible for TaskEntry {
    
    fn to_bytes(&self) -> Vec<u8> {
        serde_json::to_vec(self).unwrap() 
    }

    fn from_bytes(bytes: &[u8]) -> Self {
        serde_json::from_slice(bytes).unwrap()
    }
}


