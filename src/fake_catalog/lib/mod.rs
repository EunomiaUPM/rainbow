use crate::fake_catalog::data::models::DatasetsCatalogModel;
use crate::fake_catalog::data::repo::{
    create_dataset_repo, delete_dataset_repo, get_dataset_by_id_repo, get_datasets_by_endpoint_repo,
};
use uuid::Uuid;

pub fn create_dataset(endpoint: String) -> anyhow::Result<DatasetsCatalogModel> {
    let transaction = create_dataset_repo(endpoint)?;
    Ok(transaction)
}

pub fn get_dataset_by_id(id: Uuid) -> anyhow::Result<Option<DatasetsCatalogModel>> {
    let transaction = get_dataset_by_id_repo(id)?;
    Ok(transaction)
}

pub fn get_datasets_by_endpoint(endpoint: String) -> anyhow::Result<Vec<DatasetsCatalogModel>> {
    let transaction = get_datasets_by_endpoint_repo(endpoint)?;
    Ok(transaction)
}

pub fn delete_dataset(id: Uuid) -> anyhow::Result<()> {
    let _ = delete_dataset_repo(id)?;
    Ok(())
}

mod test {
    use crate::fake_catalog::lib::create_dataset;
    use uuid::Uuid;

    #[test]
    fn create_dataset_test() {
        let ds = create_dataset(String::from("http://localhost:8000/data/test")).unwrap();
        assert_eq!(ds.dataset_endpoint, "http://localhost:8000/data/test");
        assert!(Uuid::parse_str(&*ds.dataset_id.to_string()).is_ok())
    }
}
