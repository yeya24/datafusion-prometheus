use std::any::Any;
use std::borrow::Borrow;
use std::collections::{HashMap, HashSet};
use std::str::FromStr;
use std::sync::Arc;
use datafusion::arrow::array::{ArrayRef, Float64Array, Int64Array, StringArray};
use datafusion::arrow::datatypes::{DataType, Field, Schema, SchemaRef};
use datafusion::arrow::record_batch::RecordBatch;
use datafusion::catalog::catalog::CatalogProvider;
use datafusion::catalog::schema::{MemorySchemaProvider, SchemaProvider};
use datafusion::datasource::MemTable;
use datafusion::logical_plan::CreateMemoryTable;
use datafusion::parquet::basic::LogicalType::Map;
use datafusion::prelude::length;
use datafusion::prelude::Partitioning::Hash;
use futures::executor::block_on;
use prometheus_http_query::{Client, InstantQueryBuilder};
use futures::prelude::*;


/// `CatalogProvider` implementation for Prometheus.
pub struct PromCatalogProvider {
    client: Client,
    schema: Vec<String>,
    schema_provider: Arc<dyn SchemaProvider>,
}

impl PromCatalogProvider {
    pub async fn default() -> Self {
        PromCatalogProvider::new("http://localhost:9090").await
    }

    pub async fn new(url: &str) -> Self {
        let client = Client::from_str(url).unwrap();
        let schema = vec!["prometheus".to_string()];
        let schema_provider = MemorySchemaProvider::new();
        let q = "{__name__=~\".+\"}";
        let response = client.query(q).get().await.unwrap();
        let mut metric_fields: HashMap<String,SchemaRef> = HashMap::new();
        let mut metric_batches: HashMap<String, Vec<RecordBatch>> = HashMap::new();
        for result in response.data().as_vector().unwrap() {
            let map = result.metric();
            let name = map.get("__name__").unwrap();
            let mut contains_metric = false;
            if metric_fields.contains_key(name) {
                contains_metric = true;
            }
            let mut fields = vec![];
            let mut columns: Vec<ArrayRef>  = vec![];

            {
                let mut keys: Vec<_> = map.keys().into_iter().collect();
                keys.sort();
                for k in keys {
                    if !contains_metric {
                        fields.push(Field::new(k, DataType::Utf8, false));
                    }
                    columns.push(Arc::new(StringArray::from(vec![map.get(k).unwrap().clone()])));
                }
                if !contains_metric {
                    fields.push(Field::new("timestamp", DataType::Float64, false));
                    fields.push(Field::new("value", DataType::Float64, false));
                }
                columns.push(Arc::new(Float64Array::from(vec![result.sample().timestamp()])));
                columns.push(Arc::new(Float64Array::from(vec![result.sample().value()])));
                if fields.len() > 0 {
                    metric_fields.insert(name.clone(), Arc::new(Schema::new(fields)));
                }
                if !metric_batches.contains_key(name) {
                    let mut v = Vec::new();
                    metric_batches.insert(name.clone(), v);
                }
                let batch = RecordBatch::try_new(
                    metric_fields.get(name).unwrap().clone(),
                    columns,
                ).unwrap();
                let mut v = metric_batches
                    .get(name)
                    .unwrap()
                    .clone();
                v.push(batch);
                metric_batches.insert(name.clone(), v);
            }
        }

        for k in metric_fields.keys() {
            let schema_ref = metric_fields.get(k).unwrap().clone();
            let table_provider = Arc::new(MemTable::try_new(schema_ref, vec![metric_batches.get(k).unwrap().clone()])
                .unwrap());
            schema_provider.register_table(k.to_string(), table_provider).expect("TODO: panic message");
        }
        let sp = Arc::new(schema_provider);
        PromCatalogProvider {
            client,
            schema,
            schema_provider: sp,
        }
    }
}

impl CatalogProvider for PromCatalogProvider {
    fn as_any(&self) -> &dyn Any {
        self
    }

    fn schema_names(&self) -> Vec<String> {
        return self.schema.clone();
    }

    fn schema(&self, name: &str) -> Option<Arc<dyn SchemaProvider>> {
        Some(self.schema_provider.clone())
    }

    fn register_schema(&self, name: &str, schema: Arc<dyn SchemaProvider>) -> datafusion::common::Result<Option<Arc<dyn SchemaProvider>>> {
        unimplemented!()
    }
}