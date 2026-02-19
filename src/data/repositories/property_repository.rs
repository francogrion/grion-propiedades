use surrealdb::Error;
use surrealdb_types::NotFoundError;

use crate::{
    data::db_context::surreal_context::DB,
    domain::models::property::Property,
};

pub struct PropertyRepository {
    table: String,
}

impl PropertyRepository {
    pub fn new() -> Self {
        Self {
            table: "property".to_string(),
        }
    }

    pub(crate) async fn get_all(&self) -> Result<Vec<Property>, Error> {
        let records: Vec<Property> = DB.select(&self.table).await?;
        Ok(records)
    }

    pub(crate) async fn get_by_id(
        &self,
        id: String
    ) -> Result<Property, Error> {
        DB.select((&self.table, id.clone())).await?.ok_or_else(||
            Error::not_found(
                format!("Property with id {} not found", id).into(),
                NotFoundError::Record { id }
            )
        )
    }

    pub(crate) async fn get_by_title(
        &self,
        title: String
    ) -> Result<Property, Error> {
        DB.query("SELECT * FROM property WHERE title = $title")
            .bind(("title", title.clone())).await?
            .take::<Option<Property>>(0)?
            .ok_or_else(|| {
                Error::not_found(
                    format!("Property with title {} not found", title).into(),
                    NotFoundError::Record { id: title }
                )
            })
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use pretty_assertions::assert_eq;

    #[test]
    fn test_repo_initialization() {
        let repo = PropertyRepository::new();
        assert_eq!(repo.table, "property");
    }
}
