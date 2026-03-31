use std::io::{Error, ErrorKind, Result};

use sea_orm::{ActiveModelTrait, DatabaseConnection, EntityTrait};

use crate::db::store::rt;
use crate::entities::todo::{self, ActiveModel, Entity as TodoEntity};

fn io_err(e: impl std::fmt::Display) -> Error {
    Error::new(ErrorKind::Other, e.to_string())
}

pub fn load_all(db: &DatabaseConnection) -> Result<Vec<todo::Model>> {
    rt().block_on(async { TodoEntity::find().all(db).await.map_err(io_err) })
}

pub fn insert(db: &DatabaseConnection, model: ActiveModel) -> Result<todo::Model> {
    rt().block_on(async { model.insert(db).await.map_err(io_err) })
}

pub fn update(db: &DatabaseConnection, model: ActiveModel) -> Result<()> {
    rt().block_on(async { model.update(db).await.map_err(io_err).map(|_| ()) })
}

pub fn delete(db: &DatabaseConnection, id: i32) -> Result<()> {
    rt().block_on(async {
        TodoEntity::delete_by_id(id).exec(db).await.map_err(io_err).map(|_| ())
    })
}
