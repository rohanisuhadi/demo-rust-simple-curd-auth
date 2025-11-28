use crate::domain::user::User;
use crate::dto::filter_query::{FilterParam, FilterValue, build_filters};
use crate::dto::pagination::{PaginatedResponse, PaginationParams};
use crate::dto::user_req::UserReq;
use crate::repository::user_repo::UserRepository;
use axum::async_trait;
use sqlx::PgPool;
use uuid::Uuid;

#[derive(Clone)]
pub struct UserRepositoryImpl {
    pub pool: PgPool,
}

const SELECT_USER: &str = r#"
    SELECT id, full_name, phone_number, email, created_at, password, active, image_url
    FROM users
    WHERE
"#;

#[async_trait]
impl UserRepository for UserRepositoryImpl {
    async fn find_by_email(&self, email: &str) -> Result<User, sqlx::Error> {
        sqlx::query_as::<_, User>(&format!("{SELECT_USER} email = $1"))
            .bind(email)
            .fetch_one(&self.pool)
            .await
    }

    async fn find_by_id(&self, id: Uuid) -> Result<User, sqlx::Error> {
        sqlx::query_as::<_, User>(&format!("{SELECT_USER} id = $1"))
            .bind(id)
            .fetch_one(&self.pool)
            .await
    }

    async fn update_by_id(&self, user: &User) -> Result<User, sqlx::Error> {
        sqlx::query_as::<_, User>(
                r#"
                UPDATE users
                SET full_name = $1,
                    phone_number = $2,
                    email = $3,
                    active = $4,
                    image_url = $5
                WHERE id = $6
                RETURNING id, full_name, phone_number, email, created_at, password, active, image_url
                "#,
            )
            .bind(&user.full_name)
            .bind(&user.phone_number)
            .bind(&user.email)
            .bind(user.active)
            .bind(&user.image_url)
            .bind(user.id)
            .fetch_one(&self.pool)
            .await
    }

    async fn delete_by_id(&self, id: Uuid) -> Result<bool, sqlx::Error> {
        let result = sqlx::query("DELETE FROM users WHERE id = $1")
            .bind(id)
            .execute(&self.pool)
            .await?;

        Ok(result.rows_affected() == 1)
    }

    async fn save(&self, user: &UserReq) -> Result<User, sqlx::Error> {
        sqlx::query_as::<_, User>(
                r#"
                INSERT INTO users (id, full_name, email, active, password)
                VALUES ($1,$2,$3,$4,$5)
                RETURNING id, full_name, phone_number, email, created_at, password, active, image_url
                "#,
            )
            .bind(&user.id)
            .bind(&user.full_name)
            .bind(&user.email)
            .bind(user.active)
            .bind(&user.password)
            .fetch_one(&self.pool)
            .await
    }

    async fn paginate(
        &self,
        filter_params: Vec<FilterParam>,
        pagination: &PaginationParams,
    ) -> Result<PaginatedResponse<User>, sqlx::Error> {
        let (where_clause, filter_values) = build_filters(&filter_params);

        println!("where clause | {:?}", where_clause);
        let data_query = format!(
            "
            SELECT id, full_name, phone_number, email, created_at, null as password, active, image_url
            FROM users
            {where_clause}
            ORDER BY created_at DESC
            LIMIT ${} OFFSET ${}
        ",
            filter_params.len() + 1,
            filter_params.len() + 2
        );

        let mut query = sqlx::query_as::<_, User>(&data_query);

        let count_sql = format!("SELECT COUNT(*) FROM users {}", where_clause);
        let mut query_count = sqlx::query_scalar::<_, i64>(&count_sql);

        for filter_value in filter_values {
            match filter_value {
                FilterValue::Bool(bo) => {
                    query = query.bind(bo);
                    query_count = query_count.bind(bo)
                }
                FilterValue::String(string) => {
                    query = query.bind(string.clone());
                    query_count = query_count.bind(string)
                }
                // FilterValue::StrList(str_list) => {
                //     query = query.bind(str_list.clone());
                //     query_count = query_count.bind(str_list)
                // }
                // FilterValue::UuidList(uuid_list) => {
                //     query = query.bind(uuid_list.clone());
                //     query_count = query_count.bind(uuid_list)
                // }
                FilterValue::Like(string) => {
                    let pattern = format!("%{}%", string);
                    query = query.bind(pattern.clone());
                    query_count = query_count.bind(pattern)
                }
            };
        }

        query = query
            .bind(pagination.per_page as i64)
            .bind(pagination.offset() as i64);

        let rows = query.fetch_all(&self.pool).await?;
        let total: i64 = query_count.fetch_one(&self.pool).await?;

        let total_pages = ((total as f64) / (pagination.per_page as f64)).ceil() as u32;

        Ok(PaginatedResponse {
            data: rows,
            page: pagination.page,
            per_page: pagination.per_page,
            total: total as u64,
            total_pages,
        })
    }
}
