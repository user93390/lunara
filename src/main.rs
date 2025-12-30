mod database;

use database::Database;
use std::error::Error;
use uuid::Uuid;

/*
db.insert(
    "accounts",
    &["uid", "username", "password"],
    params!(uid, username, password)
).await?;
 */

/*
  let row = db.select_one(
       "accounts",
       &["uid", "username", "password"],
       "uid = $1",
       params!(uid)
   ).await?;
*/

/*
    let new_password = "new_hashed_password_789";
    let rows_affected = db
        .update(
            "accounts",
            &["password"],
            params!(new_password),
            "uid = $1",
            params!(uid),
        )
        .await?;
 */
#[tokio::main]
async fn main() -> Result<(), Box<dyn Error>> {
    let db_host = "nah";
    let db_port = "no";
    let db_name = "nope";
    let db_user = "hell nah";
    let db_password = "ea";

    let connection_string = match db_password {
        pwd if !pwd.is_empty() => format!(
            "host={} port={} dbname={} user={} password={}",
            db_host, db_port, db_name, db_user, pwd
        ),
        _ => format!(
            "host={} port={} dbname={} user={}",
            db_host, db_port, db_name, db_user
        ),
    };

    let db = Database::connect(&connection_string).await?;
    
    // Read
    let rows = db
        .select("accounts", &["uid", "username"], None, &[])
        .await?;

    println!("✓ All accounts:");
    for row in rows {
        let uid: Uuid = row.get(0);
        let username: String = row.get(1);
        println!("  - {} ({})", username, uid);
    }

    Ok(())
}
