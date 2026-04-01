use std::io;

use sea_orm::QuerySelect;
use sea_orm::{ActiveModelBehavior, ActiveModelTrait, ActiveValue::Set, DatabaseConnection};
use sea_orm::{ColumnTrait, Condition, DeriveEntityModel, DerivePrimaryKey, QueryFilter};
use sea_orm::{DeriveRelation, EntityTrait, EnumIter, PaginatorTrait, PrimaryKeyTrait, QueryOrder};
use time::Date;

use crate::kinds::{page::Page, repeat::Repeat};
use crate::utils::date::{format_date, parse_date, today};
use crate::utils::db::rt;
use crate::{log_error, log_warn};

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

#[derive(Clone)]
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

    pub fn list(db: &DatabaseConnection, page: Page, offset: usize, limit: usize) -> Vec<Todo> {
        if limit == 0 {
            return vec![];
        }

        let query = base_query(page).offset(offset as u64).limit(limit as u64);

        match rt().block_on(async { query.all(db).await.map_err(io_err) }) {
            Ok(models) => models.into_iter().map(Todo::from).collect(),
            Err(e) => {
                log_error!(
                    "failed to load todos for page {} (offset={}, limit={}): {e}",
                    page.label(),
                    offset,
                    limit
                );
                vec![]
            }
        }
    }

    pub fn count(db: &DatabaseConnection, page: Page) -> usize {
        let query = base_query(page);

        match rt().block_on(async { query.count(db).await.map_err(io_err) }) {
            Ok(count) => count as usize,
            Err(e) => {
                log_error!("failed to count todos for page {}: {e}", page.label());
                0
            }
        }
    }

    pub fn toggle(&mut self, db: &DatabaseConnection) -> Option<Todo> {
        self.done = !self.done;

        if self.done && self.due_date.is_none() {
            self.due_date = Some(today());
        }

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

    pub fn delete(&self, db: &DatabaseConnection) -> bool {
        let Some(id) = self.id else {
            log_warn!("todo has no id, skipping db delete");
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

    pub fn save(&mut self, db: &DatabaseConnection) -> bool {
        match self.id {
            Some(_) => self.update(db),
            None => match rt().block_on(async { self.to_model().insert(db).await.map_err(io_err) }) {
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
        match rt().block_on(async { self.to_model().update(db).await.map_err(io_err) }) {
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

    fn to_model(&self) -> ActiveModel {
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

impl ActiveModelBehavior for ActiveModel {}

fn io_err(e: impl std::fmt::Display) -> io::Error {
    io::Error::new(io::ErrorKind::Other, e.to_string())
}

fn base_query(page: Page) -> sea_orm::Select<Entity> {
    let today = format_date(today());

    match page {
        Page::Due => Entity::find()
            .filter(Column::Done.eq(false))
            .filter(Column::DueDate.lt(today))
            .order_by_asc(Column::DueDate)
            .order_by_asc(Column::Id),
        Page::Today => Entity::find()
            .filter(Column::DueDate.eq(today))
            .order_by_asc(Column::Id),
        Page::Index => Entity::find()
            .filter(
                Condition::any()
                    .add(Column::DueDate.is_null())
                    .add(Column::DueDate.gte(today)),
            )
            .order_by_asc(Column::DueDate)
            .order_by_asc(Column::Id),
        Page::History => Entity::find()
            .filter(Column::Done.eq(true))
            .order_by_desc(Column::DueDate)
            .order_by_desc(Column::Id),
    }
}
