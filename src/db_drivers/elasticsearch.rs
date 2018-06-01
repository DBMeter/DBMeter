use elastic::prelude::{index, IndexRequest, IndexResponse, SyncClientBuilder};
use export::Export;
use serde_json;

pub struct ElasticSearch;

impl ElasticSearch {
    pub fn save(export: Export) {
        let client = SyncClientBuilder::new().build().unwrap();

        if !client.index_exists(index("dbmeter")).send().unwrap().exists() {
            client.index_create(index("dbmeter")).send().unwrap();
        }

        let body = serde_json::to_string(&export).unwrap();

        let response = client
            .request(IndexRequest::for_index_ty("dbmeter", "_doc", body))
            .send()
            .unwrap()
            .into_response::<IndexResponse>()
            .unwrap();

        println!("{}", response.id());
        //this commented code not support autoincrement
        //match client.document_index(index("dbmeter"), id(""), export).send() {
        //    Ok(response) => println!("{:?}", response),
        //   error => {
        //        println!("{:?}", error);
        //        panic!("Elastic due storage error");
        //    }
        //}
    }
}
