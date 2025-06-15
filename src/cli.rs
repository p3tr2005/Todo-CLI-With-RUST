use clap::{Args, Parser, Subcommand};
use owo_colors::colors::css::DarkGrey;
use owo_colors::colors::*;
use owo_colors::{OwoColorize, colors::css::LightGreen};
use tabled::{
    Table,
    settings::{Color, object::Columns, style::Style},
};

use crate::model::{Task, TaskTable};

#[derive(Parser, Debug)]
#[command(
    author,
    version,
    about,
    long_about = "A fast todo apps written in Rust!"
)]
pub struct TodoCli {
    #[clap(subcommand)]
    pub entity: Entity,
}

#[derive(Debug, Subcommand)]
pub enum Entity {
    /// views all list of tasks
    View(ViewTasks),
    /// add task on the list
    Add(AddTask),
    /// remove specific task
    Remove(RemoveTaskById),
    /// mark task as done
    Done(MarkAsDone),
}

#[derive(Debug, Args)]
pub struct ViewTasks {
    #[arg(long)]
    pub json: bool,
}

#[derive(Debug, Args, Clone)]
pub struct AddTask {
    /// task title
    pub title: String,
    /// task description - optional
    #[arg(short)]
    pub desc: Option<String>,
}

#[derive(Debug, Args, Clone)]
pub struct RemoveTaskById {
    /// delete specific task by id
    id: String,
}

#[derive(Debug, Args, Clone)]
pub struct MarkAsDone {
    /// mark specific task as done
    id: String,
}

pub fn handle_views(args: &ViewTasks) {
    let Ok(tasks) = Task::get_tasks() else {
        println!(
            "{}",
            String::from("Failed to get tasks!")
                .fg::<Black>()
                .bg::<BrightRed>()
        );

        return;
    };

    if tasks.is_empty() {
        println!("{}", String::from("There is no task yet!").fg::<DarkGrey>());

        return ();
    }

    if args.json {
        let json = serde_json::to_string_pretty(&tasks).unwrap();

        println!("{json}");
    } else {
        let mut table = Table::new(tasks.iter().map(TaskTable::from));

        table.with(Style::modern());
        table.modify(Columns::first(), Color::FG_BRIGHT_BLACK);
        table.modify(Columns::one(1), Color::FG_BRIGHT_CYAN);
        table.modify(Columns::one(2), Color::FG_MAGENTA);
        table.modify(Columns::one(3), Color::FG_YELLOW);
        table.modify(Columns::one(4), Color::FG_BRIGHT_BLUE);

        println!("{}", table);
    }

    return ();
}

pub fn handle_add(args: &AddTask) {
    let title = args.title.clone();
    let desc = args.desc.clone();

    match Task::add_tasks(Task::new(title, desc, false)) {
        Err(_) => println!("{}", String::from("Failed to add tasks").fg::<BrightRed>()),
        Ok(_) => println!(
            "{}",
            String::from("Task successfully added!").fg::<LightGreen>()
        ),
    };
}

pub fn handle_remove(args: &RemoveTaskById) {
    let task_id = args.id.clone();

    match Task::delete_by_id(&task_id) {
        Err(_) => println!(
            "{}",
            String::from("Failed to remove task").fg::<BrightRed>()
        ),
        Ok(_) => println!(
            "{}",
            String::from("Task successfully deleted!").fg::<LightGreen>()
        ),
    }
}

pub fn handle_done(args: &MarkAsDone) {
    let id = &args.id;

    match Task::mark_done(&id) {
        Err(_) => println!(
            "{}",
            String::from("Failed to mark task as done").fg::<BrightRed>()
        ),
        Ok(_) => println!(
            "{}",
            String::from("Task successfully marked as done!").fg::<LightGreen>()
        ),
    }
}
