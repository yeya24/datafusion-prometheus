mod catalog_provider;

use std::sync::Arc;
use clap::Parser;
use datafusion::catalog::catalog::CatalogProvider;
use datafusion::prelude::{SessionConfig, SessionContext};
use rustyline::Editor;
use rustyline::error::ReadlineError;
use crate::catalog_provider::prometheus::PromCatalogProvider;
use anyhow::{Result};

#[derive(Parser, Debug)]
#[clap(author, version, about, long_about = None)]
struct Args {
    /// Prometheus URL
    #[clap(short, long, default_value_t = String::from("http://localhost:9090"), group = "addr")]
    url: String,
    /// Initial query to load metrics from Prometheus.
    #[clap(short, long, default_value_t = String::from(r#"{__name__=~".+"}"#), group = "metrics_query")]
    query: String,
}

#[tokio::main]
async fn main() -> Result<()> {
    let args = Args::parse();
    let config = SessionConfig::new().with_information_schema(true);
    let ctx = SessionContext::with_config(config);

    let prom_catalog = PromCatalogProvider::new(args.url.as_str(), args.query.as_str()).await;
    ctx.register_catalog("prometheus", Arc::new(prom_catalog));

    ctx.sql("select * from information_schema.tables order by table_catalog, table_schema, table_name ")
        .await?
        .show()
        .await?;

    let mut rl = Editor::<()>::new()?;
    loop {
        let read_sql = read_sql(&mut rl);
        match read_sql {
            Ok(sql) => {
                if !sql.trim().is_empty() {
                    rl.add_history_entry(sql.as_str());
                    ctx.sql(sql.as_str())
                        .await?
                        .show()
                        .await?;
                }
            }
            Err(ReadlineError::Interrupted) => {
                println!("Interrupted");
            }
            Err(ReadlineError::Eof) => {
                println!("Exited");
                break;
            }
            Err(err) => {
                println!("Error: {:?}", err);
                break;
            }
        }
    }

    Ok(())
}

fn read_sql(rl: &mut Editor<()>) -> Result<String, ReadlineError> {
    let mut sql = String::new();
    loop {
        let prompt = if sql.is_empty() { "> " } else { "? " };
        let line = rl.readline(prompt)?;
        if line.is_empty() {
            continue;
        }

        sql.push_str(line.as_str());
        if line.ends_with(';') {
            return Ok(sql);
        } else {
            sql.push('\n');
        }
    }
}
