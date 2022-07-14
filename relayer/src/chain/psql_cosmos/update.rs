use serde::{Deserialize, Serialize};
use sqlx::PgPool;

use ibc_proto::ibc::core::connection::v1::IdentifiedConnection as RawIdentifiedConnection;

use crate::error::Error;

#[derive(Debug, Deserialize, Serialize)]
pub struct IbcData {
    pub connections: Vec<RawIdentifiedConnection>,
    // ..
}

#[derive(Debug, Deserialize, Serialize)]
pub struct IbcSnapshot {
    pub height: u64,
    pub json_data: IbcData,
}

pub async fn init_dbs(pool: &PgPool, snapshot: &IbcSnapshot) -> Result<(), Error> {
    // create the ibc table if it does not exist
    sqlx::query(
        r#"
        CREATE TABLE IF NOT EXISTS ibc_json (
            height DOUBLE PRECISION PRIMARY KEY,
            json_data JSON
        );"#,
    )
    .execute(pool)
    .await
    .map_err(Error::sqlx)?;

    // insert the json blob, update if already there
    let json_blob = serde_json::to_string(&snapshot).unwrap();
    let sql_insert_cmd = format!(
        "INSERT INTO ibc_json SELECT height, json_data \
        FROM json_populate_record(NULL::ibc_json, '{}') \
        ON CONFLICT (height) DO UPDATE SET json_data=EXCLUDED.json_data",
        json_blob
    );
    sqlx::query(sql_insert_cmd.as_str())
        .execute(pool)
        .await
        .map_err(Error::sqlx)?;
    Ok(())
}