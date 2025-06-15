use std::{error::Error, fs, path::PathBuf};

use chrono::{DateTime, Utc};
use serde::{Deserialize, Serialize};
use tabled::Tabled;
use uuid::Uuid;

#[derive(Debug, Serialize, Deserialize, Clone)]
pub struct Task {
    pub id: Uuid,
    pub title: String,
    #[serde(rename = "description")]
    pub desc: Option<String>,
    #[serde(rename = "completed")]
    pub done: bool,
    #[serde(rename = "modifiedAt")]
    pub modified_at: DateTime<Utc>,
}

#[derive(Debug, Tabled)]
pub struct TaskTable {
    #[tabled(rename = "ID")]
    pub id: String,
    #[tabled(rename = "Title")]
    pub title: String,
    #[tabled(rename = "Description")]
    pub desc: String,
    #[tabled(rename = "Done")]
    pub done: String,
    #[tabled(rename = "Modified At")]
    pub modified_at: String,
}

impl From<&Task> for TaskTable {
    fn from(task: &Task) -> Self {
        Self {
            id: task.id.to_string(),
            title: task.title.clone(),
            desc: task.desc.clone().unwrap_or("-".to_string()),
            done: if task.done { "✅" } else { "❌" }.to_string(),
            modified_at: task.modified_at.format("%a %d %Y").to_string(),
        }
    }
}

impl Task {
    pub fn new(title: impl Into<String>, desc: Option<String>, done: bool) -> Self {
        Self {
            id: Uuid::new_v4(),
            title: title.into(),
            desc,
            done,
            modified_at: Utc::now(),
        }
    }

    pub fn get_tasks() -> Result<Vec<Self>, Box<dyn Error>> {
        let path = PathBuf::from("/home/peter/Developer/rust/todo_cli/db.json");
        let result = fs::read_to_string(path)?;

        let tasks: Vec<Task> = serde_json::from_str::<Vec<Task>>(&result)?;

        Ok(tasks)
    }

    pub fn add_tasks(task: Task) -> Result<(), Box<dyn Error>> {
        let path = PathBuf::from("/home/peter/Developer/rust/todo_cli/db.json");

        let mut tasks = Task::get_tasks().unwrap_or_default();

        tasks.push(task);

        let converted_tasks = serde_json::to_string_pretty(&tasks)?;

        fs::write(path, converted_tasks)?;

        Ok(())
    }

    pub fn delete_by_id(id: &str) -> Result<(), Box<dyn Error>> {
        let path = PathBuf::from("/home/peter/Developer/rust/todo_cli/db.json");

        let tasks = Task::get_tasks()?;

        let filtered_tasks: Vec<Task> = tasks
            .into_iter()
            .filter(|task| !task.id.to_string().contains(id))
            .collect();

        let converted_tasks = serde_json::to_string_pretty(&filtered_tasks)?;

        fs::write(path, converted_tasks)?;

        Ok(())
    }

    pub fn mark_done(id: &str) -> Result<(), Box<dyn Error>> {
        let path = PathBuf::from("/home/peter/Developer/rust/todo_cli/db.json");

        let tasks = Task::get_tasks()?
            .into_iter()
            .map(|mut task| {
                if task.id.to_string().contains(id) {
                    task.done = !task.done;
                }

                task
            })
            .collect::<Vec<Task>>();

        let converted_tasks = serde_json::to_string_pretty(&tasks)?;

        fs::write(path, converted_tasks)?;

        Ok(())
    }
}
