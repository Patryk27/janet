use anyhow::*;
use sqlx::SqliteConnection;

const MIGRATIONS: &[&str] = &[
    include_str!("migrations/01.sql"),
    include_str!("migrations/02.sql"),
    include_str!("migrations/03.sql"),
];

pub async fn run(conn: &mut SqliteConnection) -> Result<()> {
    boot(conn).await?;

    tracing::debug!("Checking database's version");

    if let Some(version) = current_version(conn).await? {
        tracing::debug!("... {}", version);
        migrate(conn, version).await?;
    } else {
        tracing::debug!("... none; this is a fresh start");
        migrate(conn, 0).await?;
    }

    Ok(())
}

async fn boot(conn: &mut SqliteConnection) -> Result<()> {
    sqlx::query(include_str!("migrations/00-boot.sql"))
        .execute(&mut *conn)
        .await?;

    Ok(())
}

async fn current_version(conn: &mut SqliteConnection) -> Result<Option<usize>> {
    let version = sqlx::query_as::<_, (Option<i64>,)>("SELECT max(id) FROM migrations")
        .fetch_one(conn)
        .await?
        .0
        .map(|version| version as usize);

    Ok(version)
}

async fn migrate(conn: &mut SqliteConnection, start_from_migration: usize) -> Result<()> {
    for (migration_id, migration) in MIGRATIONS.iter().enumerate().skip(start_from_migration) {
        let migration_id = migration_id + 1;

        tracing::info!(
            "Migrating database from version {} to {}",
            migration_id - 1,
            migration_id,
        );

        sqlx::query(migration)
            .execute(&mut *conn)
            .await
            .with_context(|| format!("Couldn't execute migration {}", migration_id))?;

        sqlx::query("INSERT INTO migrations (id) VALUES (?)")
            .bind(migration_id as i64)
            .execute(&mut *conn)
            .await?;
    }

    Ok(())
}
