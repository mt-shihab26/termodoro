use std::io;

use sea_orm::ActiveValue::Set;
use sea_orm::{
    ActiveModelBehavior, ActiveModelTrait, DatabaseConnection, DeriveEntityModel, DerivePrimaryKey, DeriveRelation,
    EntityTrait, EnumIter, PrimaryKeyTrait,
};
use time::Date;

use crate::kinds::repeat::Repeat;
use crate::utils::db::rt;

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

    pub fn toggle(&mut self) {
        self.done = !self.done;
    }

    pub fn to_active_model(&self) -> ActiveModel {
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

#[derive(Clone, Debug, PartialEq, DeriveEntityModel)]
#[sea_orm(table_name = "todos")]
pub struct Model {
    #[sea_orm(primary_key, auto_increment = true)]
    pub id: i32,
    pub text: String,
    pub done: bool,
    pub due_date: Option<String>,
    pub repeat: Option<String>,
}

#[derive(Copy, Clone, Debug, EnumIter, DeriveRelation)]
pub enum Relation {}

impl ActiveModelBehavior for ActiveModel {}

fn io_err(e: impl std::fmt::Display) -> io::Error {
    io::Error::new(io::ErrorKind::Other, e.to_string())
}

pub fn load_all(db: &DatabaseConnection) -> io::Result<Vec<Model>> {
    rt().block_on(async { Entity::find().all(db).await.map_err(io_err) })
}

pub fn insert(db: &DatabaseConnection, model: ActiveModel) -> io::Result<Model> {
    rt().block_on(async { model.insert(db).await.map_err(io_err) })
}

pub fn update(db: &DatabaseConnection, model: ActiveModel) -> io::Result<()> {
    rt().block_on(async { model.update(db).await.map_err(io_err).map(|_| ()) })
}

pub fn delete(db: &DatabaseConnection, id: i32) -> io::Result<()> {
    rt().block_on(async { Entity::delete_by_id(id).exec(db).await.map_err(io_err).map(|_| ()) })
}
