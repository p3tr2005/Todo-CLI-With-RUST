use clap::Parser;

use crate::cli::{Entity, TodoCli, handle_add, handle_done, handle_remove, handle_views};

mod cli;
mod model;

fn main() {
    let args = TodoCli::parse();

    match args.entity {
        Entity::View(args) => handle_views(&args),
        Entity::Add(args) => handle_add(&args),
        Entity::Remove(args) => handle_remove(&args),
        Entity::Done(args) => handle_done(&args),
    }
}
