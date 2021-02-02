use sqlx::sqlite::SqliteQueryAs;
use sqlx::{encode::Encode, Connect, Query, QueryAs, Sqlite, SqliteConnection, Type};

// pub use sqlx::FromRow as Row;

pub trait CreateData<

    T: std::marker::Send + for<'c> sqlx::FromRow<'c, sqlx::sqlite::SqliteRow<'c>>,
>
{
    fn create(&self) -> QueryBuilder;
}

pub trait UpdateData<

    T: std::marker::Send + for<'c> sqlx::FromRow<'c, sqlx::sqlite::SqliteRow<'c>>,
>
{
    fn update(&self) -> QueryBuilder;
}

pub trait DeleteData<

    T: std::marker::Send + for<'c> sqlx::FromRow<'c, sqlx::sqlite::SqliteRow<'c>>,
>
{
    fn delete(&self) -> QueryBuilder;
}

pub trait ReadData<

    T: std::marker::Send + for<'c> sqlx::FromRow<'c, sqlx::sqlite::SqliteRow<'c>>,
>
{
    fn read(&self) -> QueryAsBuilder<T>;
}

pub struct DataContext {
    conn: SqliteConnection,
}

pub struct QueryAsBuilder<
    'q,
    T: std::marker::Send + for<'c> sqlx::FromRow<'c, sqlx::sqlite::SqliteRow<'c>>,
> {
    query: QueryAs<'q, Sqlite, T>,
}

impl<'q, T: std::marker::Send + for<'c> sqlx::FromRow<'c, sqlx::sqlite::SqliteRow<'c>>>
    QueryAsBuilder<'q, T>
{
    pub fn new(sql: &'q str) -> QueryAsBuilder<T> {
        Self {
            query: sqlx::query_as::<Sqlite, T>(sql),
        }
    }

    pub fn bind<P>(self, value: P) -> QueryAsBuilder<'q, T>
    where
        P: Type<Sqlite>,
        P: Encode<Sqlite>,
    {
        Self {
            query: self.query.bind(value),
        }
    }

    pub fn query(self) -> QueryAs<'q, Sqlite, T> {
        self.query
    }
}

pub struct QueryBuilder<'q> {
    query: Query<'q, Sqlite>,
}

impl<'q> QueryBuilder<'q> {
    pub fn new(sql: &'q str) -> QueryBuilder {
        Self {
            query: sqlx::query::<Sqlite>(sql),
        }
    }

    pub fn bind<P>(self, value: P) -> QueryBuilder<'q>
    where
        P: Type<Sqlite>,
        P: Encode<Sqlite>,
    {
        Self {
            query: self.query.bind(value),
        }
    }

    fn query(self) -> Query<'q, Sqlite> {
        self.query
    }
}

impl<'a> DataContext {
    pub async fn new(url: &str) -> Self {
        let conn = SqliteConnection::connect(url).await.unwrap();
        Self { conn }
    }

    pub async fn create_and_get<
        T: CreateData<R> + ReadData<R>,
        R: std::marker::Send + for<'c> sqlx::FromRow<'c, sqlx::sqlite::SqliteRow<'c>>,
    >(
        &mut self,
        t: &T,
    ) -> R {
        t.create().query().execute(&mut self.conn).await.unwrap();
        t.read().query().fetch_one(&mut self.conn).await.unwrap()
    }

    pub async fn execute(&mut self, query: Query<'_, Sqlite>) -> u64 {
        query.execute(&mut self.conn).await.unwrap()
    }

    pub async fn create<
        T: CreateData<R>,
        R: std::marker::Send + for<'c> sqlx::FromRow<'c, sqlx::sqlite::SqliteRow<'c>>,
    >(
        &mut self,
        t: &T,
    ) -> u64 {
        t.create().query().execute(&mut self.conn).await.unwrap()
    }

    pub async fn update<
        T: UpdateData<R>,
        R: std::marker::Send + for<'c> sqlx::FromRow<'c, sqlx::sqlite::SqliteRow<'c>>,
    >(
        &mut self,
        t: &T,
    ) -> u64 {
        t.update().query().execute(&mut self.conn).await.unwrap()
    }

    pub async fn get<
        T: ReadData<R>,
        R: std::marker::Send + for<'c> sqlx::FromRow<'c, sqlx::sqlite::SqliteRow<'c>>,
    >(
        &mut self,
        t: &T,
    ) -> R {
        t.read().query().fetch_one(&mut self.conn).await.unwrap()
    }

    pub async fn find<
        T: ReadData<R>,
        R: std::marker::Send + for<'c> sqlx::FromRow<'c, sqlx::sqlite::SqliteRow<'c>>,
    >(
        &mut self,
        t: &T,
    ) -> Option<R> {
        t.read()
            .query()
            .fetch_optional(&mut self.conn)
            .await
            .unwrap()
    }

    pub async fn query<R>(&mut self, builder: QueryAsBuilder<'_, R>) -> Vec<R>
    where
        R: std::marker::Send + for<'c> sqlx::FromRow<'c, sqlx::sqlite::SqliteRow<'c>>,
    {
        builder.query().fetch_all(&mut self.conn).await.unwrap()
    }

    pub async fn delete<
        T: DeleteData<R>,
        R: std::marker::Send + for<'c> sqlx::FromRow<'c, sqlx::sqlite::SqliteRow<'c>>,
    >(
        &mut self,
        t: T,
    ) -> u64 {
        t.delete().query().execute(&mut self.conn).await.unwrap()
    }
}
