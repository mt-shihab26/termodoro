use std::io;

use sea_orm::{
    ActiveModelBehavior, ActiveModelTrait, ActiveValue::Set, ColumnTrait, Condition, DatabaseConnection,
    DeriveEntityModel, DerivePrimaryKey, DeriveRelation, EntityTrait, EnumIter, Order, PaginatorTrait, PrimaryKeyTrait,
    QueryFilter, QueryOrder, QuerySelect, sea_query::Expr,
};
use time::{Date, Duration};

use crate::{
    kinds::{page::Page, repeat::Repeat},
    log_error, log_warn,
    utils::{
        date::{format_date, parse_date, today},
        db::rt,
    },
};

/// Represents a single todo item with optional scheduling and repeat configuration.
#[derive(Clone)]
pub struct Todo {
    /// Database primary key; `None` until the todo is persisted.
    pub id: Option<i32>,
    /// The todo's display text.
    pub text: String,
    /// Whether the todo has been completed.
    pub done: bool,
    /// Optional date the todo is due.
    pub due_date: Option<Date>,
    /// Optional repeat schedule applied when the todo is completed.
    pub repeat: Option<Repeat>,
    /// Id of the todo this was spawned from, if it is a repeated occurrence.
    pub parent_id: Option<i32>,
    /// Date when the todo was created.
    pub created_at: Date,
    /// Date when the todo was last updated.
    pub updated_at: Date,
}

impl Todo {
    /// Creates an unsaved in-memory Todo with default values.
    pub fn new(text: String, due_date: Option<Date>, repeat: Option<Repeat>, parent_id: Option<i32>) -> Self {
        let now = today();

        Self {
            id: None,
            text,
            done: false,
            due_date,
            repeat,
            parent_id,
            created_at: now.clone(),
            updated_at: now,
        }
    }

    /// Inserts or updates the todo in the database, returning whether it succeeded.
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

    /// Creates the next repeated occurrence of this todo, skipping if one already exists.
    pub fn save_next(&self, db: &DatabaseConnection) -> Option<Todo> {
        let (Some(repeat), Some(due_date)) = (self.repeat.as_ref(), self.due_date) else {
            return None;
        };

        let next_date = repeat.next_date(due_date);
        let next_date_str = format_date(next_date);
        let parent_id = self.id;

        let already_exists = rt().block_on(async {
            Entity::find()
                .filter(Column::ParentId.eq(parent_id))
                .filter(Column::DueDate.eq(&next_date_str))
                .count(db)
                .await
                .unwrap_or(0)
        }) > 0;

        if already_exists {
            return None;
        }

        let mut next = Todo::new(self.text.clone(), Some(next_date), Some(Repeat::of(repeat)), parent_id);

        if next.save(db) { Some(next) } else { None }
    }

    /// Returns a paginated list of todos for the given page filter.
    pub fn list(db: &DatabaseConnection, page: Page, offset: usize, limit: usize) -> Vec<Todo> {
        if limit == 0 {
            return vec![];
        }

        let query = Self::base_query(page).offset(offset as u64).limit(limit as u64);

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

    /// Returns the total number of todos for the given page filter.
    pub fn count(db: &DatabaseConnection, page: Page) -> usize {
        let query = Self::base_query(page);

        match rt().block_on(async { query.count(db).await.map_err(io_err) }) {
            Ok(count) => count as usize,
            Err(e) => {
                log_error!("failed to count todos for page {}: {e}", page.label());
                0
            }
        }
    }

    /// Counts items that appear before today in the Index sort order (past dates + no-date).
    pub fn count_before_today(db: &DatabaseConnection) -> usize {
        let today = format_date(today());
        let query = Entity::find().filter(
            Condition::any()
                .add(Column::DueDate.lt(today))
                .add(Column::DueDate.is_null()),
        );
        match rt().block_on(async { query.count(db).await.map_err(io_err) }) {
            Ok(count) => count as usize,
            Err(e) => {
                log_error!("failed to count index items before today: {e}");
                0
            }
        }
    }

    /// Builds the base ORM query for the given page filter.
    fn base_query(page: Page) -> sea_orm::Select<Entity> {
        let today_date = today();
        let today = format_date(today_date);

        match page {
            Page::Due => Entity::find()
                .filter(Column::Done.eq(false))
                .filter(Column::DueDate.lt(today))
                .order_by_desc(Column::DueDate)
                .order_by_desc(Column::CreatedAt),
            Page::Today => Entity::find()
                .filter(Column::DueDate.eq(today))
                .order_by_desc(Column::CreatedAt),
            Page::Index => {
                let null_key = format!("{}~", format_date(today_date - Duration::days(1)));
                Entity::find()
                    .order_by(
                        Expr::cust_with_values(
                            "CASE WHEN due_date IS NULL THEN ? ELSE due_date END",
                            [null_key],
                        ),
                        Order::Asc,
                    )
                    .order_by_asc(Column::CreatedAt)
            }
            Page::History => Entity::find()
                .filter(Column::Done.eq(true))
                .order_by_desc(Column::DueDate)
                .order_by_desc(Column::Id),
        }
    }

    /// Persists the current field values to the database, returning whether it succeeded.
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

    /// Toggles the done state and spawns the next repeat occurrence when marked done.
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

        self.save_next(db)
    }

    /// Deletes this todo from the database, returning whether it succeeded.
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

    /// Converts this todo into a SeaORM active model for insert or update.
    fn to_model(&self) -> ActiveModel {
        let due_date = self.due_date.map(format_date);
        let repeat = self.repeat.as_ref().map(|r| r.to_db_str().to_string());
        let today = format_date(today());
        match self.id {
            Some(id) => ActiveModel {
                id: Set(id),
                text: Set(self.text.clone()),
                done: Set(self.done),
                due_date: Set(due_date),
                repeat: Set(repeat),
                parent_id: Set(self.parent_id),
                created_at: Set(format_date(self.created_at)),
                updated_at: Set(today),
            },
            None => ActiveModel {
                text: Set(self.text.clone()),
                done: Set(self.done),
                due_date: Set(due_date),
                repeat: Set(repeat),
                parent_id: Set(self.parent_id),
                created_at: Set(today.clone()),
                updated_at: Set(today),
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
            parent_id: m.parent_id,
            created_at: parse_date(&m.created_at).unwrap_or_else(today),
            updated_at: parse_date(&m.updated_at).unwrap_or_else(today),
        }
    }
}

#[derive(Clone, Debug, PartialEq, DeriveEntityModel)]
#[sea_orm(table_name = "todos")]
pub struct Model {
    #[sea_orm(primary_key, auto_increment = true)]
    id: i32,
    text: String,
    done: bool,
    due_date: Option<String>,
    repeat: Option<String>,
    parent_id: Option<i32>,
    created_at: String,
    updated_at: String,
}

#[derive(Copy, Clone, Debug, EnumIter, DeriveRelation)]
pub enum Relation {}

impl ActiveModelBehavior for ActiveModel {}

fn io_err(e: impl std::fmt::Display) -> io::Error {
    io::Error::new(io::ErrorKind::Other, e.to_string())
}
