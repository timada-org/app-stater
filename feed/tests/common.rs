use evento::{PgEngine, PgProducer};
use futures_util::{Future, TryFutureExt};
use sqlx::{
    migrate::{MigrateDatabase, Migrator},
    Any, PgPool,
};
use std::{io, path::Path, time::Duration};
use tokio::sync::OnceCell;

static ONCE: OnceCell<PgProducer> = OnceCell::const_new();

pub async fn get_producer() -> &'static PgProducer {
    ONCE.get_or_init(|| async {
        let dsn = "postgres://starter@127.0.0.1:26257/starter_test?sslmode=disable";
        let exists = retry_connect_errors(dsn, Any::database_exists)
            .await
            .unwrap();

        if exists {
            let _ = Any::drop_database(dsn).await;
        }

        let _ = Any::create_database(dsn).await;

        let pool =
            PgPool::connect("cockroach://starter@127.0.0.1:26257/starter_test?sslmode=disable")
                .await
                .unwrap();

        Migrator::new(Path::new("../migrations"))
            .await
            .unwrap()
            .set_locking(false)
            .run(&pool)
            .await
            .unwrap();

        let projection = PgEngine::new_prefix(pool.clone(), "projection")
            .run(0)
            .await
            .unwrap();

        PgEngine::new(pool)
            .data(projection)
            .subscribe(timada_starter_feed::feeds_subscriber())
            .subscribe(timada_starter_feed::tags_count_subscriber())
            .run(0)
            .await
            .unwrap()
    })
    .await
}

// pub static EVENTO_PRODUCER: Lazy<PgProducer> =
//     Lazy::new(async || futures::executor::block_on(async {}));

/// Attempt an operation that may return errors like `ConnectionRefused`,
/// retrying up until `ops.connect_timeout`.
///
/// The closure is passed `&ops.database_url` for easy composition.
async fn retry_connect_errors<'a, F, Fut, T>(
    database_url: &'a str,
    mut connect: F,
) -> sqlx::Result<T>
where
    F: FnMut(&'a str) -> Fut,
    Fut: Future<Output = sqlx::Result<T>> + 'a,
{
    sqlx::any::install_default_drivers();

    backoff::future::retry(
        backoff::ExponentialBackoffBuilder::new()
            .with_max_elapsed_time(Some(Duration::from_secs(10)))
            .build(),
        || {
            connect(database_url).map_err(|e| -> backoff::Error<sqlx::Error> {
                if let sqlx::Error::Io(ref ioe) = e {
                    match ioe.kind() {
                        io::ErrorKind::ConnectionRefused
                        | io::ErrorKind::ConnectionReset
                        | io::ErrorKind::ConnectionAborted => {
                            return backoff::Error::transient(e);
                        }
                        _ => (),
                    }
                }

                backoff::Error::permanent(e)
            })
        },
    )
    .await
}
