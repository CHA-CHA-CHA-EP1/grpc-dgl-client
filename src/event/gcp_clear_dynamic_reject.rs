use tonic::async_trait;

use crate::{event::EventHandler, helper::gcp_db_connection::gcp_create_db_pool};

pub struct GcpClearDynamicRejectHandler;

impl GcpClearDynamicRejectHandler {
    pub async fn execute_clear_dynamic_reject_impl(
        &self,
        cif_num: &str,
    ) -> Result<(), Box<dyn std::error::Error>> {
        let mysql_user = "";
        let mysql_pass = "";
        let termloan_host = "";
        let dgl_host = "";

        // Delete from smart_money_db
        println!("--- smart_money_db ---");
        match gcp_create_db_pool(termloan_host, mysql_user, mysql_pass, "smart_money_db").await {
            Ok(pool) => {
                let result = sqlx::query("DELETE FROM LOAN_APPLICATION_STATUS WHERE CIF_NUM = ?")
                    .bind(cif_num)
                    .execute(&pool)
                    .await?;

                println!("smart_money_db: Deleted {} row(s)", result.rows_affected());
                pool.close().await;
            }
            Err(e) => {
                eprintln!("smart_money_db: Connection failed - {}", e);
                return Err(e.into());
            }
        }

        // Delete from tn_termloan_db
        println!("--- tn_termloan_db ---");
        match gcp_create_db_pool(termloan_host, mysql_user, mysql_pass, "tn_termloan_db").await {
            Ok(pool) => {
                let result = sqlx::query("DELETE FROM LOAN_APPLICATION_STATUS WHERE CIF_NUM = ?")
                    .bind(cif_num)
                    .execute(&pool)
                    .await?;

                println!("tn_termloan_db: Deleted {} row(s)", result.rows_affected());
                pool.close().await;
            }
            Err(e) => {
                eprintln!("tn_termloan_db: Connection failed - {}", e);
                return Err(e.into());
            }
        }
        // Delete from dg_lending_db
        println!("--- dg_lending_db ---");
        match gcp_create_db_pool(dgl_host, mysql_user, mysql_pass, "dg_lending_db").await {
            Ok(pool) => {
                let result = sqlx::query("DELETE FROM LOAN_APPLICATION_STATUS WHERE CIF_NUM = ?")
                    .bind(cif_num)
                    .execute(&pool)
                    .await?;

                println!("dg_lending_db: Deleted {} row(s)", result.rows_affected());
                pool.close().await;
            }
            Err(e) => {
                eprintln!("dg_lending_db: Connection failed - {}", e);
                return Err(e.into());
            }
        }

        println!("===================================");
        Ok(())
    }
}

#[async_trait]
impl EventHandler for GcpClearDynamicRejectHandler {
    async fn handle(&self, payload: String) -> Result<String, String> {
        let cif_num = payload;

        let _ = self
            .execute_clear_dynamic_reject_impl(&cif_num)
            .await
            .map_err(|e| format!("error: {e}"));

        Ok("Ok".to_string())
    }
}
