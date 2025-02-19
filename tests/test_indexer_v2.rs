use algonaut::indexer::IndexerBuilder;
use algonaut_client::indexer::v2::message::{
    QueryAccount, QueryAccountInfo, QueryAccountTransaction, QueryApplicationInfo,
    QueryApplications, QueryAssetTransaction, QueryAssets, QueryAssetsInfo, QueryBalances,
    QueryTransaction, Role,
};
use algonaut_core::Round;
use dotenv::dotenv;
use std::env;
use std::error::Error;
use tokio::test;

#[test]
async fn test_health_endpoint() -> Result<(), Box<dyn Error>> {
    // load variables in .env
    dotenv().ok();

    let indexer = IndexerBuilder::new()
        .bind(env::var("INDEXER_URL")?.as_ref())
        .build_v2()?;

    let res = indexer.health().await;

    println!("{:#?}", res);
    assert!(res.is_ok());

    Ok(())
}

#[test]
async fn test_accounts_endpoint() -> Result<(), Box<dyn Error>> {
    // load variables in .env
    dotenv().ok();

    let indexer = IndexerBuilder::new()
        .bind(env::var("INDEXER_URL")?.as_ref())
        .build_v2()?;

    let query = QueryAccount {
        application_id: None,
        asset_id: None,
        auth_addr: None,
        currency_greater_than: None,
        currency_less_than: None,
        limit: Some(2),
        next: None,
        round: None,
    };

    let res = indexer.accounts(&query).await;

    println!("{:#?}", res);
    assert!(res.is_ok());

    Ok(())
}

#[test]
async fn test_account_info_endpoint() -> Result<(), Box<dyn Error>> {
    // load variables in .env
    dotenv().ok();

    let indexer = IndexerBuilder::new()
        .bind(env::var("INDEXER_URL")?.as_ref())
        .build_v2()?;

    let account: String = env::var("ACCOUNT")?.parse()?;

    let query = QueryAccountInfo {
        include_all: None,
        round: Some(Round(0)),
    };

    let res = indexer.account_info(account.as_str(), &query).await;

    println!("{:#?}", res);
    assert!(res.is_ok());

    Ok(())
}

#[test]
async fn test_account_transactions_endpoint() -> Result<(), Box<dyn Error>> {
    // load variables in .env
    dotenv().ok();

    let indexer = IndexerBuilder::new()
        .bind(env::var("INDEXER_URL")?.as_ref())
        .build_v2()?;

    let query = QueryAccountTransaction {
        after_time: None,
        asset_id: None,
        before_time: None,
        currency_greater_than: None,
        currency_less_than: None,
        limit: None,
        max_round: None,
        min_round: None,
        next: None,
        note_prefix: None,
        rekey_to: None,
        round: None,
        sig_type: None,
        tx_type: None,
        txid: None,
    };

    let account: String = env::var("ACCOUNT")?.parse()?;

    let res = indexer
        .account_transactions(&account.as_str(), &query)
        .await;

    println!("{:#?}", res);
    assert!(res.is_ok());

    Ok(())
}

#[test]
async fn test_applications_endpoint() -> Result<(), Box<dyn Error>> {
    // load variables in .env
    dotenv().ok();

    let indexer = IndexerBuilder::new()
        .bind(env::var("INDEXER_URL")?.as_ref())
        .build_v2()?;

    let query = QueryApplications {
        application_id: None,
        limit: None,
        next: None,
    };

    let res = indexer.applications(&query).await;

    println!("{:#?}", res);
    assert!(res.is_ok());

    Ok(())
}

#[test]
#[ignore]
async fn test_applications_info_endpoint() -> Result<(), Box<dyn Error>> {
    // load variables in .env
    dotenv().ok();

    let indexer = IndexerBuilder::new()
        .bind(env::var("INDEXER_URL")?.as_ref())
        .build_v2()?;

    let query = QueryApplicationInfo { include_all: None };

    let res = indexer.application_info("123", &query).await;

    println!("{:#?}", res);
    assert!(res.is_ok());

    Ok(())
}

#[test]
async fn test_assets_endpoint() -> Result<(), Box<dyn Error>> {
    // load variables in .env
    dotenv().ok();

    let indexer = IndexerBuilder::new()
        .bind(env::var("INDEXER_URL")?.as_ref())
        .build_v2()?;

    let query = QueryAssets {
        asset_id: None,
        creator: None,
        limit: None,
        name: None,
        next: None,
        unit: None,
    };

    let res = indexer.assets(&query).await;

    println!("{:#?}", res);
    assert!(res.is_ok());

    Ok(())
}

#[test]
#[ignore]
async fn test_assets_info_endpoint() -> Result<(), Box<dyn Error>> {
    // load variables in .env
    dotenv().ok();

    let indexer = IndexerBuilder::new()
        .bind(env::var("INDEXER_URL")?.as_ref())
        .build_v2()?;

    let query = QueryAssetsInfo { include_all: None };

    let res = indexer.assets_info("123", &query).await;

    println!("{:#?}", res);
    assert!(res.is_ok());

    Ok(())
}

#[test]
async fn test_asset_balances_endpoint() -> Result<(), Box<dyn Error>> {
    // load variables in .env
    dotenv().ok();

    let indexer = IndexerBuilder::new()
        .bind(env::var("INDEXER_URL")?.as_ref())
        .build_v2()?;

    let query = QueryBalances {
        currency_greater_than: None,
        currency_less_than: None,
        limit: None,
        next: None,
        round: None,
    };

    let res = indexer.asset_balances("123", &query).await;

    println!("{:#?}", res);
    assert!(res.is_ok());

    Ok(())
}

#[test]
async fn test_asset_transactions_endpoint() -> Result<(), Box<dyn Error>> {
    // load variables in .env
    dotenv().ok();

    let indexer = IndexerBuilder::new()
        .bind(env::var("INDEXER_URL")?.as_ref())
        .build_v2()?;

    let query = QueryAssetTransaction {
        address: None,
        address_role: Some(Role::Sender),
        after_time: None,
        before_time: None,
        currency_greater_than: None,
        currency_less_than: None,
        exclude_close_to: None,
        limit: None,
        max_round: None,
        min_round: None,
        next: None,
        note_prefix: None,
        rekey_to: None,
        round: None,
        sig_type: None,
        tx_type: None,
        txid: None,
    };

    let res = indexer.asset_transactions("123", &query).await;

    println!("{:#?}", res);
    assert!(res.is_ok());

    Ok(())
}

#[test]
async fn test_block_endpoint() -> Result<(), Box<dyn Error>> {
    // load variables in .env
    dotenv().ok();

    let indexer = IndexerBuilder::new()
        .bind(env::var("INDEXER_URL")?.as_ref())
        .build_v2()?;

    let res = indexer.block(Round(0)).await;

    println!("{:#?}", res);
    assert!(res.is_ok());

    Ok(())
}

#[test]
#[ignore]
async fn test_transactions_endpoint() -> Result<(), Box<dyn Error>> {
    // load variables in .env
    dotenv().ok();

    let indexer = IndexerBuilder::new()
        .bind(env::var("INDEXER_URL")?.as_ref())
        .build_v2()?;

    let query = QueryTransaction {
        address: None,
        address_role: None,
        after_time: None,
        application_id: None,
        asset_id: None,
        before_time: None,
        currency_greater_than: None,
        currency_less_than: None,
        exclude_close_to: None,
        limit: None,
        max_round: None,
        min_round: None,
        next: None,
        note_prefix: None,
        rekey_to: None,
        round: None,
        sig_type: None,
        tx_type: None,
        txid: None,
    };

    let res = indexer.transactions(&query).await;

    println!("{:#?}", res);
    assert!(res.is_ok());

    Ok(())
}

#[test]
#[ignore]
async fn test_transaction_info_endpoint() -> Result<(), Box<dyn Error>> {
    // load variables in .env
    dotenv().ok();

    let indexer = IndexerBuilder::new()
        .bind(env::var("INDEXER_URL")?.as_ref())
        .build_v2()?;

    let res = indexer.transaction_info("123").await;

    println!("{:#?}", res);
    assert!(res.is_ok());

    Ok(())
}
