use surrealdb::Error;

use crate::{
    data::db_context::surreal_context::DB,
    domain::models::image::Image,
};

pub struct ImageRepository {
    table: String,
}

impl ImageRepository {
    pub fn new() -> Self {
        Self {
            table: "image".to_string(),
        }
    }

    pub(crate) async fn get_all(&self) -> Result<Vec<Image>, Error> {
        let records: Vec<Image> = DB.select(&self.table).await?;
        Ok(records)
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use pretty_assertions::assert_eq;

    #[test]
    fn test_repo_initialization() {
        let repo = ImageRepository::new();
        assert_eq!(repo.table, "image");
    }
}
