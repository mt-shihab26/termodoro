use std::io;

use sea_orm::{ActiveModelBehavior, ActiveModelTrait, ActiveValue::Set, DatabaseConnection};
use sea_orm::{DeriveEntityModel, DerivePrimaryKey, DeriveRelation, EntityTrait, EnumIter, PrimaryKeyTrait};
use time::Date;

use crate::log_error;
use crate::{kinds::repeat::Repeat, utils::db::rt};

#[derive(Clone, Debug, PartialEq, DeriveEntityModel)]
#[sea_orm(table_name = "todos")]
pub struct Model {
    #[sea_orm(primary_key, auto_increment = true)]
    id: i32,
    text: String,
    done: bool,
    due_date: Option<String>,
    repeat: Option<String>,
}

#[derive(Copy, Clone, Debug, EnumIter, DeriveRelation)]
pub enum Relation {}

pub struct Todo {
    pub id: Option<i32>,
    pub text: String,
    pub done: bool,
    pub due_date: Option<Date>,
    pub repeat: Option<Repeat>,
}

impl Todo {
    pub fn new(text: String, due_date: Option<Date>, repeat: Option<Repeat>) -> Self {
        Self {
            id: None,
            text,
            done: false,
            due_date,
            repeat,
        }
    }

    pub fn all(db: &DatabaseConnection) -> Vec<Todo> {
        match rt().block_on(async { Entity::find().all(db).await.map_err(io_err) }) {
            Ok(models) => models.into_iter().map(Todo::from).collect(),
            Err(e) => {
                log_error!("failed to load todos: {e}");
                vec![]
            }
        }
    }

    pub fn toggle_and_save(&mut self, db: &DatabaseConnection) -> Option<Todo> {
        self.done = !self.done;

        if !self.update(db) {
            self.done = !self.done;
            return None;
        }

        if !self.done {
            return None;
        }

        let (Some(repeat), Some(date)) = (self.repeat.as_ref(), self.due_date) else {
            return None;
        };

        let mut next = Todo::new(
            self.text.clone(),
            Some(repeat.next_date(date)),
            Some(Repeat::of(repeat)),
        );

        if next.save(db) { Some(next) } else { None }
    }

    fn to_active_model(&self) -> ActiveModel {
        let due_date = self.due_date.map(format_date);
        let repeat = self.repeat.as_ref().map(|r| r.to_db_str().to_string());
        match self.id {
            Some(id) => ActiveModel {
                id: Set(id),
                text: Set(self.text.clone()),
                done: Set(self.done),
                due_date: Set(due_date),
                repeat: Set(repeat),
            },
            None => ActiveModel {
                text: Set(self.text.clone()),
                done: Set(self.done),
                due_date: Set(due_date),
                repeat: Set(repeat),
                ..Default::default()
            },
        }
    }

    pub fn save(&mut self, db: &DatabaseConnection) -> bool {
        match self.id {
            Some(_) => self.update(db),
            None => match rt().block_on(async { self.to_active_model().insert(db).await.map_err(io_err) }) {
                Ok(model) => {
                    *self = model.into();
                    true
                }
                Err(e) => {
                    log_error!("failed to insert todo: {e}");
                    false
                }
            },
        }
    }

    pub fn update(&mut self, db: &DatabaseConnection) -> bool {
        match rt().block_on(async { self.to_active_model().update(db).await.map_err(io_err) }) {
            Ok(model) => {
                *self = model.into();
                true
            }
            Err(e) => {
                log_error!("failed to update todo: {e}");
                false
            }
        }
    }

    pub fn delete(&self, db: &DatabaseConnection) -> bool {
        let Some(id) = self.id else {
            log_error!("failed to delete todo: missing id");
            return false;
        };

        match rt().block_on(async { Entity::delete_by_id(id).exec(db).await.map_err(io_err).map(|_| ()) }) {
            Ok(()) => true,
            Err(e) => {
                log_error!("failed to delete todo: {e}");
                false
            }
        }
    }
}

impl From<Model> for Todo {
    fn from(m: Model) -> Self {
        Self {
            id: Some(m.id),
            text: m.text,
            done: m.done,
            due_date: m.due_date.as_deref().and_then(parse_date),
            repeat: m.repeat.as_deref().and_then(Repeat::from_db_str),
        }
    }
}

impl ActiveModelBehavior for ActiveModel {}

fn io_err(e: impl std::fmt::Display) -> io::Error {
    io::Error::new(io::ErrorKind::Other, e.to_string())
}

fn format_date(date: Date) -> String {
    format!("{}-{:02}-{:02}", date.year(), date.month() as u8, date.day())
}

fn parse_date(s: &str) -> Option<Date> {
    let mut parts = s.splitn(3, '-');
    let year: i32 = parts.next()?.parse().ok()?;
    let month: u8 = parts.next()?.parse().ok()?;
    let day: u8 = parts.next()?.parse().ok()?;
    Date::from_calendar_date(year, time::Month::try_from(month).ok()?, day).ok()
}
