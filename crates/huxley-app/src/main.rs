mod cli;

use anyhow::Result;
use futures::future;

use huxley_config::HuxleyConfig;
use huxley_engine::{scheduler, worker, cleaner};
use huxley_state::HuxleyState;
use huxley_store::{postgres, redis};
use std::sync::Arc;

use cli::HuxleyCli;

#[tokio::main]
async fn main() -> Result<()> {
    let args = HuxleyCli::parse();
    let config = Arc::new(HuxleyConfig::from_env()?);

    let db_pool = postgres::create_pool(&config.clone()).await?;
    let redis_pool = redis::create_pool(&config.clone()).await?;

    let state =  Arc::new(HuxleyState::new(config.clone(), db_pool.clone(), redis_pool.clone()));

    let roles: Vec<&str> = args.roles.split(',').collect();

    let handles = vec![];

    for role in roles {
        match role {
            "api" => {
                handles.push(tokio::spawn(huxley_api::run(state.clone())));
            },
            "scheduler" => {
                handles(tokio::spawn(scheduler::run(state.clone())));
            },
            "worker" => {
                handles(tokio::spawn(worker::run(state.clone())));
            },
            "cleaner" => {
                handles(tokio::spawn(cleaner::run(state.clone())));
            }
            _ => {

            }
        }
    }

    if handles.len() > 0 {
        future::try_join_all(handles).await?;
    }

    Ok(())
}
