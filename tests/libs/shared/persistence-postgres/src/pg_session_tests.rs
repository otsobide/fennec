use std::sync::Arc;

use shared_persistence_postgres::infrastructure::postgres::aggregate_lock::AggregateLock;
use shared_persistence_postgres::infrastructure::postgres::pg_session::PgSession;
use shared_persistence_postgres::infrastructure::postgres::pg_unit_of_work::PgUnitOfWork;
use sqlx::PgPool;
use uuid::Uuid;

fn widget_lock(id: Uuid) -> AggregateLock {
    AggregateLock::new("widget", &id)
}

async fn create_widget_table(pool: &PgPool) {
    sqlx::query("CREATE TABLE widget (id uuid PRIMARY KEY)")
        .execute(pool)
        .await
        .unwrap();
}

async fn insert_widget(session: &PgSession, id: Uuid) -> Result<u64, sqlx::Error> {
    session
        .run(widget_lock(id), move |connection| {
            Box::pin(async move {
                sqlx::query("INSERT INTO widget (id) VALUES ($1)")
                    .bind(id)
                    .execute(connection)
                    .await
                    .map(|result| result.rows_affected())
            })
        })
        .await
}

async fn count_widgets(pool: &PgPool) -> i64 {
    sqlx::query_scalar("SELECT count(*) FROM widget")
        .fetch_one(pool)
        .await
        .unwrap()
}

#[sqlx::test]
#[ignore = "requires DATABASE_URL"]
async fn it_runs_on_a_pooled_connection_outside_a_unit_of_work(pool: PgPool) {
    create_widget_table(&pool).await;
    let session = PgSession::new(pool.clone());

    insert_widget(&session, Uuid::new_v4()).await.unwrap();

    assert_eq!(count_widgets(&pool).await, 1);
}

#[sqlx::test]
#[ignore = "requires DATABASE_URL"]
async fn it_runs_on_the_transaction_of_the_current_unit_of_work(pool: PgPool) {
    create_widget_table(&pool).await;
    let session = PgSession::new(pool.clone());
    let unit_of_work = PgUnitOfWork::begin(&pool).await.unwrap();

    Arc::clone(&unit_of_work)
        .scope(insert_widget(&session, Uuid::new_v4()))
        .await
        .unwrap();
    unit_of_work
        .take_transaction()
        .await
        .unwrap()
        .rollback()
        .await
        .unwrap();

    assert_eq!(count_widgets(&pool).await, 0);
}

#[sqlx::test]
#[ignore = "requires DATABASE_URL"]
async fn it_reads_its_own_writes_inside_a_unit_of_work(pool: PgPool) {
    create_widget_table(&pool).await;
    let session = PgSession::new(pool.clone());
    let unit_of_work = PgUnitOfWork::begin(&pool).await.unwrap();
    let id = Uuid::new_v4();

    let seen = Arc::clone(&unit_of_work)
        .scope(async {
            insert_widget(&session, id).await.unwrap();
            session
                .run(widget_lock(id), move |connection| {
                    Box::pin(async move {
                        sqlx::query_scalar::<_, i64>("SELECT count(*) FROM widget WHERE id = $1")
                            .bind(id)
                            .fetch_one(connection)
                            .await
                    })
                })
                .await
                .unwrap()
        })
        .await;

    assert_eq!(seen, 1);
}

#[sqlx::test]
#[ignore = "requires DATABASE_URL"]
async fn it_marks_the_unit_of_work_failed_when_an_operation_fails(pool: PgPool) {
    create_widget_table(&pool).await;
    let session = PgSession::new(pool.clone());
    let unit_of_work = PgUnitOfWork::begin(&pool).await.unwrap();
    let id = Uuid::new_v4();

    let second = Arc::clone(&unit_of_work)
        .scope(async {
            insert_widget(&session, id).await.unwrap();
            insert_widget(&session, id).await
        })
        .await;

    assert!(second.is_err(), "the duplicate primary key must fail");
    assert!(unit_of_work.has_failed());
}

#[sqlx::test]
#[ignore = "requires DATABASE_URL"]
async fn it_runs_unlocked_statements_on_the_unit_of_work_too(pool: PgPool) {
    create_widget_table(&pool).await;
    let session = PgSession::new(pool.clone());
    let unit_of_work = PgUnitOfWork::begin(&pool).await.unwrap();

    let seen = Arc::clone(&unit_of_work)
        .scope(async {
            insert_widget(&session, Uuid::new_v4()).await.unwrap();
            session
                .run_unlocked(|connection| {
                    Box::pin(async move {
                        sqlx::query_scalar::<_, i64>("SELECT count(*) FROM widget")
                            .fetch_one(connection)
                            .await
                    })
                })
                .await
                .unwrap()
        })
        .await;

    assert_eq!(seen, 1);
    assert_eq!(count_widgets(&pool).await, 0, "nothing is committed yet");
}
