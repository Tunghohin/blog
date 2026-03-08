use sea_orm::{Database, DatabaseConnection, DbErr, ConnectionTrait, Statement};

pub type Db = DatabaseConnection;

pub async fn connect(database_url: &str) -> Result<Db, DbErr> {
    let db = Database::connect(database_url).await?;
    Ok(db)
}

pub async fn init_db(db: &Db) -> Result<(), DbErr> {
    // 创建 posts 表
    db.execute(Statement::from_string(
        sea_orm::DatabaseBackend::Sqlite,
        r#"
        CREATE TABLE IF NOT EXISTS posts (
            id INTEGER PRIMARY KEY AUTOINCREMENT,
            author_id INTEGER NOT NULL,
            title VARCHAR(200) NOT NULL,
            slug VARCHAR(200) UNIQUE NOT NULL,
            content TEXT NOT NULL,
            summary VARCHAR(500),
            status VARCHAR(20) DEFAULT 'draft',
            created_at DATETIME DEFAULT CURRENT_TIMESTAMP,
            updated_at DATETIME DEFAULT CURRENT_TIMESTAMP,
            FOREIGN KEY (author_id) REFERENCES users(id)
        )
        "#,
    ))
    .await?;

    // 创建 tags 表
    db.execute(Statement::from_string(
        sea_orm::DatabaseBackend::Sqlite,
        r#"
        CREATE TABLE IF NOT EXISTS tags (
            id INTEGER PRIMARY KEY AUTOINCREMENT,
            name VARCHAR(50) UNIQUE NOT NULL
        )
        "#,
    ))
    .await?;

    // 创建 post_tags 表
    db.execute(Statement::from_string(
        sea_orm::DatabaseBackend::Sqlite,
        r#"
        CREATE TABLE IF NOT EXISTS post_tags (
            post_id INTEGER NOT NULL,
            tag_id INTEGER NOT NULL,
            PRIMARY KEY (post_id, tag_id),
            FOREIGN KEY (post_id) REFERENCES posts(id) ON DELETE CASCADE,
            FOREIGN KEY (tag_id) REFERENCES tags(id) ON DELETE CASCADE
        )
        "#,
    ))
    .await?;

    // 创建 users 表
    db.execute(Statement::from_string(
        sea_orm::DatabaseBackend::Sqlite,
        r#"
        CREATE TABLE IF NOT EXISTS users (
            id INTEGER PRIMARY KEY AUTOINCREMENT,
            username VARCHAR(50) UNIQUE NOT NULL,
            password_hash VARCHAR(255) NOT NULL,
            role VARCHAR(20) DEFAULT 'normal',
            created_at DATETIME DEFAULT CURRENT_TIMESTAMP
        )
        "#,
    ))
    .await?;

    db.execute(Statement::from_string(
        sea_orm::DatabaseBackend::Sqlite,
        r#"
        CREATE TABLE IF NOT EXISTS comments (
            id INTEGER PRIMARY KEY AUTOINCREMENT,
            post_id INTEGER NOT NULL,
            author_id INTEGER NOT NULL,
            content TEXT NOT NULL,
            created_at DATETIME DEFAULT CURRENT_TIMESTAMP,
            FOREIGN KEY (post_id) REFERENCES posts(id) ON DELETE CASCADE,
            FOREIGN KEY (author_id) REFERENCES users(id)
        )
        "#,
    ))
    .await?;

    Ok(())
}

/// 创建管理员账号
pub async fn create_admin(db: &Db, username: &str, password: &str) -> Result<(), DbErr> {
    let password_hash = bcrypt::hash(password, bcrypt::DEFAULT_COST)
        .map_err(|_| DbErr::Custom("Failed to hash password".to_string()))?;

    db.execute(Statement::from_string(
        sea_orm::DatabaseBackend::Sqlite,
        format!(
            r#"
            INSERT OR IGNORE INTO users (username, password_hash, role)
            VALUES ('{}', '{}', 'admin')
            "#,
            username, password_hash
        ),
    ))
    .await?;

    Ok(())
}
