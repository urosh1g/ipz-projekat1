mod macros {
    #[macro_export]
    macro_rules! generate_upsert_functions {
    ($( ($function_name:ident, $table_name:ident, $id_column:ident) ),*) => {
        $(
            async fn $function_name(executor: impl sqlx::PgExecutor<'_>, name: &str) -> uuid::Uuid {
                let query = format!(
                    r#"
                        INSERT INTO {} (name) VALUES ($1)
                        ON CONFLICT (name) DO UPDATE SET name = EXCLUDED.name
                        RETURNING {} as id
                    "#
                    ,
                    stringify!($table_name),
                    stringify!($id_column)
                );
                sqlx::query_as(&query.as_str())
                    .bind(name)
                .fetch_one(executor)
                .await
                .map(|res: (uuid::Uuid, )| res.0)
                .unwrap()
            }
        )*
    };
}
}
