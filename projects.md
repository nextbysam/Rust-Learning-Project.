# Rust Learning Projects

These 3 projects are designed to take you from beginner to average/above-average Rust proficiency. Each project builds on concepts from the previous one while introducing new patterns.

---

## Project 1: CLI Weather Dashboard
**Duration**: 2 weeks | **Difficulty**: Beginner → Intermediate

### What You'll Learn
- Async/await and tokio runtime
- HTTP requests with reqwest
- Error handling with Result and custom error types
- JSON parsing with serde
- CLI argument parsing
- Environment variables and configuration
- Testing async code

### Project Overview
Build a command-line weather dashboard that fetches data from multiple weather APIs, aggregates the results, and displays formatted output. Users can get current weather, forecasts, and compare data from different sources.

### Features to Implement

1. **Basic Weather Fetching**
   - Fetch current weather from OpenWeatherMap API
   - Parse JSON responses into Rust structs
   - Handle API errors gracefully
   - Display temperature, humidity, wind speed

2. **Multi-Source Aggregation**
   - Fetch from 2-3 different weather APIs concurrently
   - Compare and average temperature readings
   - Show which source provides what data
   - Handle partial failures (some APIs down)

3. **CLI Interface**
   - Arguments: city name, units (metric/imperial), output format
   - Flags: `--forecast`, `--compare`, `--json`
   - Configuration file support (YAML/TOML)
   - Colored terminal output

4. **Caching Layer**
   - Cache API responses to disk for 10 minutes
   - Skip API calls if fresh cache exists
   - Implement cache invalidation

### Cargo.toml Dependencies
```toml
[dependencies]
tokio = { version = "1.35", features = ["full"] }
reqwest = { version = "0.11", features = ["json"] }
serde = { version = "1.0", features = ["derive"] }
serde_json = "1.0"
clap = { version = "4.4", features = ["derive"] }
anyhow = "1.0"
thiserror = "1.0"
colored = "2.1"
dotenvy = "0.15"

[dev-dependencies]
mockito = "1.2"
tokio-test = "0.4"
```

### Implementation Guide

#### Step 1: Project Setup
```rust
// src/main.rs
use clap::Parser;

#[derive(Parser)]
#[command(name = "weather")]
#[command(about = "A CLI weather dashboard")]
struct Cli {
    /// City name to fetch weather for
    city: String,
    
    #[arg(short, long, default_value = "metric")]
    units: String,
    
    #[arg(short, long)]
    forecast: bool,
}

#[tokio::main]
async fn main() -> anyhow::Result<()> {
    let cli = Cli::parse();
    println!("Fetching weather for: {}", cli.city);
    Ok(())
}
```

#### Step 2: Define Data Models
```rust
// src/models.rs
use serde::{Deserialize, Serialize};

#[derive(Debug, Deserialize, Serialize)]
pub struct WeatherData {
    pub temperature: f64,
    pub feels_like: f64,
    pub humidity: u8,
    pub description: String,
    pub wind_speed: f64,
    pub source: String,
}

#[derive(Debug, Deserialize)]
pub struct OpenWeatherResponse {
    pub main: Main,
    pub weather: Vec<Weather>,
    pub wind: Wind,
}

#[derive(Debug, Deserialize)]
pub struct Main {
    pub temp: f64,
    pub feels_like: f64,
    pub humidity: u8,
}

#[derive(Debug, Deserialize)]
pub struct Weather {
    pub description: String,
}

#[derive(Debug, Deserialize)]
pub struct Wind {
    pub speed: f64,
}
```

#### Step 3: Create Custom Error Types
```rust
// src/error.rs
use thiserror::Error;

#[derive(Error, Debug)]
pub enum WeatherError {
    #[error("API request failed: {0}")]
    ApiError(String),
    
    #[error("Failed to parse response: {0}")]
    ParseError(#[from] serde_json::Error),
    
    #[error("Network error: {0}")]
    NetworkError(#[from] reqwest::Error),
    
    #[error("City not found: {0}")]
    CityNotFound(String),
}
```

#### Step 4: Implement API Client
```rust
// src/client.rs
use crate::{error::WeatherError, models::*};

pub struct WeatherClient {
    client: reqwest::Client,
    api_key: String,
}

impl WeatherClient {
    pub fn new(api_key: String) -> Self {
        Self {
            client: reqwest::Client::new(),
            api_key,
        }
    }
    
    pub async fn fetch_weather(&self, city: &str) -> Result<WeatherData, WeatherError> {
        let url = format!(
            "https://api.openweathermap.org/data/2.5/weather?q={}&appid={}",
            city, self.api_key
        );
        
        let response = self.client
            .get(&url)
            .send()
            .await?
            .json::<OpenWeatherResponse>()
            .await?;
        
        Ok(WeatherData {
            temperature: response.main.temp,
            feels_like: response.main.feels_like,
            humidity: response.main.humidity,
            description: response.weather[0].description.clone(),
            wind_speed: response.wind.speed,
            source: "OpenWeatherMap".to_string(),
        })
    }
}
```

#### Step 5: Concurrent API Fetching
```rust
// src/aggregator.rs
use crate::{client::WeatherClient, models::WeatherData};

pub async fn fetch_all_sources(city: &str, clients: Vec<WeatherClient>) 
    -> Vec<Result<WeatherData, Box<dyn std::error::Error>>> 
{
    let mut handles = vec![];
    
    for client in clients {
        let city = city.to_string();
        let handle = tokio::spawn(async move {
            client.fetch_weather(&city).await
                .map_err(|e| Box::new(e) as Box<dyn std::error::Error>)
        });
        handles.push(handle);
    }
    
    let mut results = vec![];
    for handle in handles {
        match handle.await {
            Ok(result) => results.push(result),
            Err(e) => results.push(Err(Box::new(e) as Box<dyn std::error::Error>)),
        }
    }
    
    results
}
```

### Key Rust Concepts Applied

1. **Ownership & Borrowing**: Passing `&str` vs `String`, understanding when to clone
2. **Error Handling**: Using `Result<T, E>`, `?` operator, custom error types with thiserror
3. **Async/Await**: Understanding futures, tokio runtime, concurrent execution
4. **Trait System**: Implementing Display, From, Error traits
5. **Pattern Matching**: Handling `Result` and `Option` types
6. **Lifetime Annotations**: Understanding when references need lifetime specifiers

### Testing Strategy
```rust
// tests/integration_test.rs
#[tokio::test]
async fn test_weather_fetch() {
    let mock_server = mockito::Server::new();
    let mock = mock_server.mock("GET", "/weather")
        .with_status(200)
        .with_body(r#"{"main":{"temp":20.5},"weather":[{"description":"clear"}],"wind":{"speed":5.0}}"#)
        .create();
    
    // Test implementation
}
```

### Success Metrics
- ✅ Fetches weather from at least 2 APIs concurrently
- ✅ Handles network errors without panicking
- ✅ Implements caching to reduce API calls
- ✅ Has 80%+ test coverage
- ✅ Uses proper Rust error handling (no unwrap/expect in production code)
- ✅ Compiles with zero warnings

---

## Project 2: REST API - Task Management Service
**Duration**: 3 weeks | **Difficulty**: Intermediate → Advanced

### What You'll Learn
- Web framework (Axum) and routing
- Database integration (SQLx with PostgreSQL)
- Async database queries
- Authentication and JWT tokens
- Middleware and request guards
- API design and RESTful principles
- Database migrations
- Integration testing with test databases

### Project Overview
Build a production-ready REST API for task management with user authentication, CRUD operations, filtering, and proper error handling. This mimics real-world backend development.

### Features to Implement

1. **User Authentication**
   - User registration and login
   - Password hashing with argon2
   - JWT token generation and validation
   - Protected routes with middleware

2. **Task CRUD Operations**
   - Create, read, update, delete tasks
   - Tasks belong to users (ownership model)
   - Task fields: title, description, status, priority, due_date
   - Validation of input data

3. **Advanced Queries**
   - Filter tasks by status, priority, date range
   - Pagination support
   - Sorting options
   - Search by title/description

4. **API Features**
   - Request/response logging middleware
   - Rate limiting
   - CORS configuration
   - Health check endpoint
   - Graceful shutdown

### Cargo.toml Dependencies
```toml
[dependencies]
tokio = { version = "1.35", features = ["full"] }
axum = { version = "0.7", features = ["macros"] }
tower = "0.4"
tower-http = { version = "0.5", features = ["cors", "trace"] }
sqlx = { version = "0.7", features = ["runtime-tokio-native-tls", "postgres", "uuid", "chrono"] }
serde = { version = "1.0", features = ["derive"] }
serde_json = "1.0"
jsonwebtoken = "9.2"
argon2 = "0.5"
uuid = { version = "1.6", features = ["serde", "v4"] }
chrono = { version = "0.4", features = ["serde"] }
validator = { version = "0.18", features = ["derive"] }
dotenvy = "0.15"
tracing = "0.1"
tracing-subscriber = "0.3"
anyhow = "1.0"
thiserror = "1.0"

[dev-dependencies]
axum-test = "14.0"
```

### Project Structure
```
src/
├── main.rs              # Application entry point
├── config.rs            # Configuration management
├── routes/
│   ├── mod.rs
│   ├── auth.rs          # Login/register routes
│   ├── tasks.rs         # Task CRUD routes
│   └── health.rs        # Health check
├── handlers/
│   ├── mod.rs
│   ├── auth.rs          # Auth handlers
│   └── tasks.rs         # Task handlers
├── models/
│   ├── mod.rs
│   ├── user.rs          # User model
│   └── task.rs          # Task model
├── db/
│   ├── mod.rs
│   └── migrations/      # SQL migration files
├── middleware/
│   ├── mod.rs
│   └── auth.rs          # JWT validation middleware
├── error.rs             # Error types
└── utils/
    ├── mod.rs
    └── jwt.rs           # JWT utilities
```

### Implementation Guide

#### Step 1: Application Setup
```rust
// src/main.rs
use axum::{Router, routing::get};
use sqlx::postgres::PgPoolOptions;
use std::net::SocketAddr;
use tower_http::cors::CorsLayer;
use tracing_subscriber;

#[tokio::main]
async fn main() -> anyhow::Result<()> {
    tracing_subscriber::fmt::init();
    
    dotenvy::dotenv().ok();
    let database_url = std::env::var("DATABASE_URL")?;
    
    let pool = PgPoolOptions::new()
        .max_connections(5)
        .connect(&database_url)
        .await?;
    
    sqlx::migrate!("./migrations").run(&pool).await?;
    
    let app = Router::new()
        .route("/health", get(health_check))
        .nest("/api/auth", auth_routes())
        .nest("/api/tasks", task_routes())
        .layer(CorsLayer::permissive())
        .with_state(pool);
    
    let addr = SocketAddr::from(([0, 0, 0, 0], 3000));
    tracing::info!("Server running on {}", addr);
    
    axum::Server::bind(&addr)
        .serve(app.into_make_service())
        .await?;
    
    Ok(())
}

async fn health_check() -> &'static str {
    "OK"
}
```

#### Step 2: Database Models
```rust
// src/models/user.rs
use sqlx::FromRow;
use serde::{Deserialize, Serialize};
use uuid::Uuid;

#[derive(Debug, FromRow, Serialize)]
pub struct User {
    pub id: Uuid,
    pub email: String,
    #[serde(skip_serializing)]
    pub password_hash: String,
    pub created_at: chrono::DateTime<chrono::Utc>,
}

#[derive(Debug, Deserialize)]
pub struct CreateUser {
    pub email: String,
    pub password: String,
}

// src/models/task.rs
#[derive(Debug, FromRow, Serialize)]
pub struct Task {
    pub id: Uuid,
    pub user_id: Uuid,
    pub title: String,
    pub description: Option<String>,
    pub status: TaskStatus,
    pub priority: TaskPriority,
    pub due_date: Option<chrono::NaiveDate>,
    pub created_at: chrono::DateTime<chrono::Utc>,
    pub updated_at: chrono::DateTime<chrono::Utc>,
}

#[derive(Debug, Serialize, Deserialize, sqlx::Type)]
#[sqlx(type_name = "task_status", rename_all = "lowercase")]
pub enum TaskStatus {
    Todo,
    InProgress,
    Done,
}

#[derive(Debug, Serialize, Deserialize, sqlx::Type)]
#[sqlx(type_name = "task_priority", rename_all = "lowercase")]
pub enum TaskPriority {
    Low,
    Medium,
    High,
}
```

#### Step 3: Database Migrations
```sql
-- migrations/001_create_users.sql
CREATE TABLE users (
    id UUID PRIMARY KEY DEFAULT gen_random_uuid(),
    email VARCHAR(255) UNIQUE NOT NULL,
    password_hash VARCHAR(255) NOT NULL,
    created_at TIMESTAMPTZ NOT NULL DEFAULT NOW()
);

CREATE INDEX idx_users_email ON users(email);

-- migrations/002_create_tasks.sql
CREATE TYPE task_status AS ENUM ('todo', 'inprogress', 'done');
CREATE TYPE task_priority AS ENUM ('low', 'medium', 'high');

CREATE TABLE tasks (
    id UUID PRIMARY KEY DEFAULT gen_random_uuid(),
    user_id UUID NOT NULL REFERENCES users(id) ON DELETE CASCADE,
    title VARCHAR(255) NOT NULL,
    description TEXT,
    status task_status NOT NULL DEFAULT 'todo',
    priority task_priority NOT NULL DEFAULT 'medium',
    due_date DATE,
    created_at TIMESTAMPTZ NOT NULL DEFAULT NOW(),
    updated_at TIMESTAMPTZ NOT NULL DEFAULT NOW()
);

CREATE INDEX idx_tasks_user_id ON tasks(user_id);
CREATE INDEX idx_tasks_status ON tasks(status);
CREATE INDEX idx_tasks_priority ON tasks(priority);
```

#### Step 4: Authentication Handler
```rust
// src/handlers/auth.rs
use axum::{Json, extract::State};
use sqlx::PgPool;
use argon2::{Argon2, PasswordHash, PasswordHasher, PasswordVerifier};
use argon2::password_hash::{SaltString, rand_core::OsRng};

use crate::models::user::{User, CreateUser};
use crate::utils::jwt::create_token;
use crate::error::ApiError;

pub async fn register(
    State(pool): State<PgPool>,
    Json(user_data): Json<CreateUser>,
) -> Result<Json<serde_json::Value>, ApiError> {
    let salt = SaltString::generate(&mut OsRng);
    let argon2 = Argon2::default();
    let password_hash = argon2
        .hash_password(user_data.password.as_bytes(), &salt)
        .map_err(|_| ApiError::InternalError)?
        .to_string();
    
    let user = sqlx::query_as::<_, User>(
        "INSERT INTO users (email, password_hash) VALUES ($1, $2) RETURNING *"
    )
    .bind(&user_data.email)
    .bind(&password_hash)
    .fetch_one(&pool)
    .await
    .map_err(|e| match e {
        sqlx::Error::Database(db_err) if db_err.is_unique_violation() => {
            ApiError::Conflict("Email already exists".to_string())
        }
        _ => ApiError::DatabaseError(e),
    })?;
    
    let token = create_token(&user.id)?;
    
    Ok(Json(serde_json::json!({
        "user": user,
        "token": token
    })))
}

pub async fn login(
    State(pool): State<PgPool>,
    Json(credentials): Json<CreateUser>,
) -> Result<Json<serde_json::Value>, ApiError> {
    let user = sqlx::query_as::<_, User>(
        "SELECT * FROM users WHERE email = $1"
    )
    .bind(&credentials.email)
    .fetch_optional(&pool)
    .await?
    .ok_or(ApiError::Unauthorized)?;
    
    let parsed_hash = PasswordHash::new(&user.password_hash)
        .map_err(|_| ApiError::InternalError)?;
    
    Argon2::default()
        .verify_password(credentials.password.as_bytes(), &parsed_hash)
        .map_err(|_| ApiError::Unauthorized)?;
    
    let token = create_token(&user.id)?;
    
    Ok(Json(serde_json::json!({
        "user": user,
        "token": token
    })))
}
```

#### Step 5: JWT Middleware
```rust
// src/middleware/auth.rs
use axum::{
    extract::{Request, State},
    http::{header, StatusCode},
    middleware::Next,
    response::Response,
};
use sqlx::PgPool;
use uuid::Uuid;

use crate::utils::jwt::verify_token;

pub async fn auth_middleware(
    State(pool): State<PgPool>,
    mut req: Request,
    next: Next,
) -> Result<Response, StatusCode> {
    let auth_header = req
        .headers()
        .get(header::AUTHORIZATION)
        .and_then(|h| h.to_str().ok())
        .ok_or(StatusCode::UNAUTHORIZED)?;
    
    let token = auth_header
        .strip_prefix("Bearer ")
        .ok_or(StatusCode::UNAUTHORIZED)?;
    
    let user_id = verify_token(token)
        .map_err(|_| StatusCode::UNAUTHORIZED)?;
    
    req.extensions_mut().insert(user_id);
    
    Ok(next.run(req).await)
}

// Extension trait for extracting user ID
#[axum::async_trait]
impl<S> axum::extract::FromRequestParts<S> for Uuid
where
    S: Send + Sync,
{
    type Rejection = StatusCode;
    
    async fn from_request_parts(
        parts: &mut axum::http::request::Parts,
        _state: &S,
    ) -> Result<Self, Self::Rejection> {
        parts
            .extensions
            .get::<Uuid>()
            .copied()
            .ok_or(StatusCode::UNAUTHORIZED)
    }
}
```

#### Step 6: Task Handlers with Queries
```rust
// src/handlers/tasks.rs
use axum::{Json, extract::{State, Query, Path}};
use sqlx::PgPool;
use uuid::Uuid;
use serde::Deserialize;

use crate::models::task::*;
use crate::error::ApiError;

#[derive(Deserialize)]
pub struct TaskFilters {
    status: Option<TaskStatus>,
    priority: Option<TaskPriority>,
    page: Option<i64>,
    limit: Option<i64>,
}

pub async fn create_task(
    State(pool): State<PgPool>,
    user_id: Uuid,
    Json(task_data): Json<CreateTask>,
) -> Result<Json<Task>, ApiError> {
    let task = sqlx::query_as::<_, Task>(
        r#"
        INSERT INTO tasks (user_id, title, description, status, priority, due_date)
        VALUES ($1, $2, $3, $4, $5, $6)
        RETURNING *
        "#
    )
    .bind(user_id)
    .bind(&task_data.title)
    .bind(&task_data.description)
    .bind(&task_data.status)
    .bind(&task_data.priority)
    .bind(&task_data.due_date)
    .fetch_one(&pool)
    .await?;
    
    Ok(Json(task))
}

pub async fn list_tasks(
    State(pool): State<PgPool>,
    user_id: Uuid,
    Query(filters): Query<TaskFilters>,
) -> Result<Json<Vec<Task>>, ApiError> {
    let page = filters.page.unwrap_or(1);
    let limit = filters.limit.unwrap_or(20);
    let offset = (page - 1) * limit;
    
    let mut query = sqlx::QueryBuilder::new(
        "SELECT * FROM tasks WHERE user_id = "
    );
    query.push_bind(user_id);
    
    if let Some(status) = filters.status {
        query.push(" AND status = ");
        query.push_bind(status);
    }
    
    if let Some(priority) = filters.priority {
        query.push(" AND priority = ");
        query.push_bind(priority);
    }
    
    query.push(" ORDER BY created_at DESC LIMIT ");
    query.push_bind(limit);
    query.push(" OFFSET ");
    query.push_bind(offset);
    
    let tasks = query
        .build_query_as::<Task>()
        .fetch_all(&pool)
        .await?;
    
    Ok(Json(tasks))
}

pub async fn get_task(
    State(pool): State<PgPool>,
    user_id: Uuid,
    Path(task_id): Path<Uuid>,
) -> Result<Json<Task>, ApiError> {
    let task = sqlx::query_as::<_, Task>(
        "SELECT * FROM tasks WHERE id = $1 AND user_id = $2"
    )
    .bind(task_id)
    .bind(user_id)
    .fetch_optional(&pool)
    .await?
    .ok_or(ApiError::NotFound)?;
    
    Ok(Json(task))
}

pub async fn update_task(
    State(pool): State<PgPool>,
    user_id: Uuid,
    Path(task_id): Path<Uuid>,
    Json(update_data): Json<UpdateTask>,
) -> Result<Json<Task>, ApiError> {
    let task = sqlx::query_as::<_, Task>(
        r#"
        UPDATE tasks
        SET title = COALESCE($1, title),
            description = COALESCE($2, description),
            status = COALESCE($3, status),
            priority = COALESCE($4, priority),
            due_date = COALESCE($5, due_date),
            updated_at = NOW()
        WHERE id = $6 AND user_id = $7
        RETURNING *
        "#
    )
    .bind(&update_data.title)
    .bind(&update_data.description)
    .bind(&update_data.status)
    .bind(&update_data.priority)
    .bind(&update_data.due_date)
    .bind(task_id)
    .bind(user_id)
    .fetch_optional(&pool)
    .await?
    .ok_or(ApiError::NotFound)?;
    
    Ok(Json(task))
}

pub async fn delete_task(
    State(pool): State<PgPool>,
    user_id: Uuid,
    Path(task_id): Path<Uuid>,
) -> Result<StatusCode, ApiError> {
    let result = sqlx::query(
        "DELETE FROM tasks WHERE id = $1 AND user_id = $2"
    )
    .bind(task_id)
    .bind(user_id)
    .execute(&pool)
    .await?;
    
    if result.rows_affected() == 0 {
        return Err(ApiError::NotFound);
    }
    
    Ok(StatusCode::NO_CONTENT)
}
```

### Key Rust Concepts Applied

1. **Advanced Lifetimes**: Understanding `'static` lifetime, request lifetimes in extractors
2. **Trait Objects**: Using `dyn Error`, trait bounds on generic functions
3. **Async Traits**: Implementing `FromRequestParts` for custom extractors
4. **Type System**: Leveraging enums for database types, type-safe SQL queries
5. **Error Propagation**: Using `?` operator, custom error types with proper conversions
6. **Zero-Copy**: References in handlers to avoid unnecessary clones
7. **Concurrency**: Database connection pooling, async request handling

### Testing Strategy
```rust
// tests/api_tests.rs
use axum_test::TestServer;

#[tokio::test]
async fn test_user_registration() {
    let app = create_test_app().await;
    let server = TestServer::new(app).unwrap();
    
    let response = server
        .post("/api/auth/register")
        .json(&serde_json::json!({
            "email": "test@example.com",
            "password": "securepass123"
        }))
        .await;
    
    response.assert_status_ok();
    response.assert_json_contains(&serde_json::json!({
        "user": {
            "email": "test@example.com"
        }
    }));
}

#[tokio::test]
async fn test_protected_route_without_auth() {
    let app = create_test_app().await;
    let server = TestServer::new(app).unwrap();
    
    let response = server.get("/api/tasks").await;
    response.assert_status_unauthorized();
}
```

### Success Metrics
- ✅ Complete CRUD operations for tasks
- ✅ JWT authentication with middleware
- ✅ Database queries use connection pooling efficiently
- ✅ No SQL injection vulnerabilities (using parameterized queries)
- ✅ Proper HTTP status codes for all responses
- ✅ Integration tests cover main flows
- ✅ Handles concurrent requests without data races
- ✅ Password hashing uses secure algorithm

---

## Project 3: Concurrent Web Scraper & Data Pipeline
**Duration**: 3 weeks | **Difficulty**: Advanced

### What You'll Learn
- Advanced concurrency patterns (channels, workers, task spawning)
- Bounded task execution and backpressure
- HTML parsing and web scraping
- Data processing pipelines
- Memory-efficient streaming
- Performance optimization and profiling
- Production-grade error recovery
- Graceful shutdown handling

### Project Overview
Build a high-performance web scraper that crawls multiple websites concurrently, extracts data, processes it through a pipeline, and stores results. This project teaches production-level concurrency patterns used in real systems.

### Features to Implement

1. **Concurrent URL Crawler**
   - Crawl multiple URLs concurrently with rate limiting
   - Respect robots.txt
   - Follow links up to N levels deep
   - Avoid crawling same URL twice (seen set)
   - Configurable max concurrent requests

2. **Data Extraction Pipeline**
   - Stage 1: Fetch HTML pages
   - Stage 2: Parse HTML and extract data
   - Stage 3: Transform/clean data
   - Stage 4: Store to database/file
   - Each stage runs concurrently

3. **Worker Pool Pattern**
   - Fixed number of worker threads
   - Task queue with bounded channel
   - Graceful worker shutdown
   - Work stealing for load balancing

4. **Monitoring & Resilience**
   - Progress tracking (pages/sec, success rate)
   - Retry failed requests with exponential backoff
   - Circuit breaker for failing domains
   - Metrics dashboard (CLI or web)
   - Graceful shutdown on Ctrl+C

### Cargo.toml Dependencies
```toml
[dependencies]
tokio = { version = "1.35", features = ["full"] }
reqwest = { version = "0.11", features = ["cookies"] }
scraper = "0.18"
url = "2.5"
governor = "0.6"
dashmap = "5.5"
sqlx = { version = "0.7", features = ["runtime-tokio-native-tls", "postgres"] }
serde = { version = "1.0", features = ["derive"] }
serde_json = "1.0"
anyhow = "1.0"
thiserror = "1.0"
tracing = "0.1"
tracing-subscriber = "0.3"
tokio-util = "0.7"
futures = "0.3"

[dev-dependencies]
criterion = "0.5"
```

### Project Structure
```
src/
├── main.rs
├── crawler/
│   ├── mod.rs
│   ├── fetcher.rs       # HTTP fetching with rate limiting
│   ├── parser.rs        # HTML parsing
│   └── robots.rs        # robots.txt handling
├── pipeline/
│   ├── mod.rs
│   ├── stages.rs        # Pipeline stage definitions
│   └── executor.rs      # Pipeline execution engine
├── workers/
│   ├── mod.rs
│   └── pool.rs          # Worker pool implementation
├── storage/
│   ├── mod.rs
│   └── repository.rs    # Data storage
├── metrics/
│   ├── mod.rs
│   └── tracker.rs       # Performance metrics
└── error.rs
```

### Implementation Guide

#### Step 1: Concurrent Fetcher with Rate Limiting
```rust
// src/crawler/fetcher.rs
use std::sync::Arc;
use std::time::Duration;
use governor::{Quota, RateLimiter};
use governor::state::{InMemoryState, NotKeyed};
use reqwest::Client;
use url::Url;

pub struct Fetcher {
    client: Client,
    rate_limiter: Arc<RateLimiter<NotKeyed, InMemoryState>>,
}

impl Fetcher {
    pub fn new(requests_per_second: u32) -> Self {
        let quota = Quota::per_second(
            std::num::NonZeroU32::new(requests_per_second).unwrap()
        );
        let rate_limiter = Arc::new(RateLimiter::direct(quota));
        
        let client = Client::builder()
            .timeout(Duration::from_secs(30))
            .user_agent("RustWebCrawler/1.0")
            .build()
            .unwrap();
        
        Self { client, rate_limiter }
    }
    
    pub async fn fetch(&self, url: &Url) -> Result<String, anyhow::Error> {
        // Wait for rate limiter
        self.rate_limiter.until_ready().await;
        
        let response = self.client
            .get(url.as_str())
            .send()
            .await?
            .text()
            .await?;
        
        Ok(response)
    }
}
```

#### Step 2: HTML Parser with Scraper
```rust
// src/crawler/parser.rs
use scraper::{Html, Selector};
use url::Url;

pub struct PageData {
    pub url: Url,
    pub title: String,
    pub links: Vec<Url>,
    pub text_content: String,
}

pub struct Parser {
    link_selector: Selector,
    title_selector: Selector,
}

impl Parser {
    pub fn new() -> Self {
        Self {
            link_selector: Selector::parse("a[href]").unwrap(),
            title_selector: Selector::parse("title").unwrap(),
        }
    }
    
    pub fn parse(&self, base_url: &Url, html: &str) -> Result<PageData, anyhow::Error> {
        let document = Html::parse_document(html);
        
        let title = document
            .select(&self.title_selector)
            .next()
            .map(|el| el.text().collect::<String>())
            .unwrap_or_default();
        
        let mut links = Vec::new();
        for element in document.select(&self.link_selector) {
            if let Some(href) = element.value().attr("href") {
                if let Ok(url) = base_url.join(href) {
                    if url.scheme() == "http" || url.scheme() == "https" {
                        links.push(url);
                    }
                }
            }
        }
        
        let text_content = document
            .root_element()
            .text()
            .collect::<Vec<_>>()
            .join(" ");
        
        Ok(PageData {
            url: base_url.clone(),
            title,
            links,
            text_content,
        })
    }
}
```

#### Step 3: Pipeline with Channels
```rust
// src/pipeline/stages.rs
use tokio::sync::mpsc;
use url::Url;

use crate::crawler::{Fetcher, Parser, PageData};

pub enum PipelineMessage {
    Fetch(Url),
    Parse(Url, String),
    Store(PageData),
    Done,
}

pub async fn fetch_stage(
    mut rx: mpsc::Receiver<PipelineMessage>,
    tx: mpsc::Sender<PipelineMessage>,
    fetcher: Arc<Fetcher>,
) {
    while let Some(msg) = rx.recv().await {
        match msg {
            PipelineMessage::Fetch(url) => {
                match fetcher.fetch(&url).await {
                    Ok(html) => {
                        let _ = tx.send(PipelineMessage::Parse(url, html)).await;
                    }
                    Err(e) => {
                        tracing::error!("Failed to fetch {}: {}", url, e);
                    }
                }
            }
            PipelineMessage::Done => break,
            _ => {}
        }
    }
}

pub async fn parse_stage(
    mut rx: mpsc::Receiver<PipelineMessage>,
    tx: mpsc::Sender<PipelineMessage>,
    parser: Arc<Parser>,
) {
    while let Some(msg) = rx.recv().await {
        match msg {
            PipelineMessage::Parse(url, html) => {
                match parser.parse(&url, &html) {
                    Ok(page_data) => {
                        let _ = tx.send(PipelineMessage::Store(page_data)).await;
                    }
                    Err(e) => {
                        tracing::error!("Failed to parse {}: {}", url, e);
                    }
                }
            }
            PipelineMessage::Done => break,
            _ => {}
        }
    }
}
```

#### Step 4: Worker Pool Implementation
```rust
// src/workers/pool.rs
use tokio::sync::mpsc;
use tokio::task::JoinHandle;
use std::sync::Arc;
use dashmap::DashSet;
use url::Url;

use crate::pipeline::PipelineMessage;
use crate::crawler::Fetcher;

pub struct WorkerPool {
    workers: Vec<JoinHandle<()>>,
    task_tx: mpsc::Sender<Url>,
}

impl WorkerPool {
    pub fn new(
        num_workers: usize,
        fetcher: Arc<Fetcher>,
        seen_urls: Arc<DashSet<Url>>,
        output_tx: mpsc::Sender<PipelineMessage>,
    ) -> Self {
        let (task_tx, task_rx) = mpsc::channel::<Url>(100);
        let task_rx = Arc::new(tokio::sync::Mutex::new(task_rx));
        
        let mut workers = Vec::with_capacity(num_workers);
        
        for worker_id in 0..num_workers {
            let task_rx = Arc::clone(&task_rx);
            let fetcher = Arc::clone(&fetcher);
            let seen_urls = Arc::clone(&seen_urls);
            let output_tx = output_tx.clone();
            
            let handle = tokio::spawn(async move {
                loop {
                    let url = {
                        let mut rx = task_rx.lock().await;
                        rx.recv().await
                    };
                    
                    match url {
                        Some(url) => {
                            if seen_urls.insert(url.clone()) {
                                tracing::debug!(
                                    "Worker {} processing {}",
                                    worker_id,
                                    url
                                );
                                let _ = output_tx
                                    .send(PipelineMessage::Fetch(url))
                                    .await;
                            }
                        }
                        None => break,
                    }
                }
                tracing::info!("Worker {} shutting down", worker_id);
            });
            
            workers.push(handle);
        }
        
        Self { workers, task_tx }
    }
    
    pub async fn submit(&self, url: Url) -> Result<(), anyhow::Error> {
        self.task_tx.send(url).await?;
        Ok(())
    }
    
    pub async fn shutdown(self) {
        drop(self.task_tx);
        
        for handle in self.workers {
            let _ = handle.await;
        }
    }
}
```

#### Step 5: Main Application with Graceful Shutdown
```rust
// src/main.rs
use std::sync::Arc;
use tokio::sync::mpsc;
use tokio::signal;
use dashmap::DashSet;
use url::Url;

use crawler::{Fetcher, Parser};
use pipeline::{fetch_stage, parse_stage, PipelineMessage};
use workers::WorkerPool;

#[tokio::main]
async fn main() -> anyhow::Result<()> {
    tracing_subscriber::fmt::init();
    
    let start_urls = vec![
        Url::parse("https://example.com")?,
    ];
    
    let fetcher = Arc::new(Fetcher::new(10)); // 10 req/sec
    let parser = Arc::new(Parser::new());
    let seen_urls = Arc::new(DashSet::new());
    
    // Create pipeline channels
    let (fetch_tx, fetch_rx) = mpsc::channel(100);
    let (parse_tx, parse_rx) = mpsc::channel(100);
    let (store_tx, store_rx) = mpsc::channel(100);
    
    // Spawn pipeline stages
    let fetch_handle = tokio::spawn(fetch_stage(
        fetch_rx,
        parse_tx.clone(),
        Arc::clone(&fetcher),
    ));
    
    let parse_handle = tokio::spawn(parse_stage(
        parse_rx,
        store_tx.clone(),
        Arc::clone(&parser),
    ));
    
    // Storage stage
    let storage_handle = tokio::spawn(async move {
        let mut count = 0;
        while let Some(msg) = store_rx.recv().await {
            match msg {
                PipelineMessage::Store(page_data) => {
                    count += 1;
                    tracing::info!(
                        "Stored page {}: {} ({} links)",
                        count,
                        page_data.title,
                        page_data.links.len()
                    );
                    
                    // Queue discovered links
                    // (Implementation depends on your crawl strategy)
                }
                PipelineMessage::Done => break,
                _ => {}
            }
        }
    });
    
    // Create worker pool
    let pool = WorkerPool::new(
        4,
        Arc::clone(&fetcher),
        Arc::clone(&seen_urls),
        fetch_tx.clone(),
    );
    
    // Submit initial URLs
    for url in start_urls {
        pool.submit(url).await?;
    }
    
    // Wait for Ctrl+C
    signal::ctrl_c().await?;
    tracing::info!("Shutdown signal received");
    
    // Graceful shutdown
    pool.shutdown().await;
    drop(fetch_tx);
    
    let _ = fetch_handle.await;
    let _ = parse_handle.await;
    let _ = storage_handle.await;
    
    tracing::info!("All workers shut down cleanly");
    Ok(())
}
```

### Key Rust Concepts Applied

1. **Advanced Concurrency**: Channels (mpsc), worker pools, task spawning
2. **Shared State**: `Arc<DashSet>` for thread-safe seen URLs
3. **Backpressure**: Bounded channels prevent memory explosion
4. **Zero-Copy**: Passing references through pipeline stages
5. **Type Safety**: Enum-based message passing between stages
6. **Async Mutex**: Using `tokio::sync::Mutex` for async-safe locking
7. **Graceful Shutdown**: Drop semantics for channel closure
8. **Performance**: Rate limiting, connection pooling, concurrent processing

### Performance Optimization Tips

1. **Measure First**: Use `cargo flamegraph` and `tokio-console`
2. **Reduce Allocations**: Reuse buffers with `Vec::clear()` instead of new allocations
3. **Batch Operations**: Process multiple URLs in batches to reduce overhead
4. **Tune Channel Sizes**: Balance memory usage vs throughput
5. **Profile Async Runtime**: Use `#[tokio::test(flavor = "multi_thread")]` for benchmarks

### Testing Strategy
```rust
#[tokio::test]
async fn test_concurrent_fetching() {
    let fetcher = Arc::new(Fetcher::new(100));
    let urls = vec![/* test URLs */];
    
    let mut handles = vec![];
    for url in urls {
        let fetcher = Arc::clone(&fetcher);
        let handle = tokio::spawn(async move {
            fetcher.fetch(&url).await
        });
        handles.push(handle);
    }
    
    let results = futures::future::join_all(handles).await;
    assert_eq!(results.iter().filter(|r| r.is_ok()).count(), 10);
}
```

### Success Metrics
- ✅ Crawls 100+ pages per minute
- ✅ Handles concurrent requests without data races
- ✅ Gracefully shuts down on Ctrl+C
- ✅ No memory leaks (use `valgrind` or `heaptrack`)
- ✅ Respects rate limits and robots.txt
- ✅ Recovers from network failures
- ✅ Uses less than 100MB memory for 10K URLs

---

## Learning Progression

### After Project 1 - You'll understand:
- Async Rust fundamentals
- Error handling patterns
- Working with external APIs
- Basic CLI applications

### After Project 2 - You'll understand:
- Web framework architecture
- Database integration
- Authentication patterns
- Production API design

### After Project 3 - You'll understand:
- Advanced concurrency patterns
- Performance optimization
- System design in Rust
- Production-grade error handling

### Combined - You'll be able to:
- Build full-stack Rust applications
- Make architectural decisions
- Debug complex async issues
- Optimize for performance
- Handle production scenarios

## Resources

- **The Rust Book**: https://doc.rust-lang.org/book/
- **Rust by Example**: https://doc.rust-lang.org/rust-by-example/
- **Tokio Tutorial**: https://tokio.rs/tokio/tutorial
- **Axum Examples**: https://github.com/tokio-rs/axum/tree/main/examples
- **This Week in Rust**: https://this-week-in-rust.org/

## Next Steps After These Projects

1. **Contribute to open source** - Find Rust projects on GitHub
2. **Build a real product** - Take an idea from start to deployment
3. **Learn unsafe Rust** - Understand low-level optimizations
4. **Explore embedded** - Try Rust on microcontrollers
5. **Deep dive into async** - Study tokio internals

You're now ready to be an average or above-average Rust developer! 🚀
