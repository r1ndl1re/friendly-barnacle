use sqlx::{Pool, Postgres};

pub(crate) async fn create_video_table(pool: &Pool<Postgres>) -> Result<(), sqlx::Error> {
    let sql = "
        CREATE TABLE IF NOT EXISTS video (
            id SERIAL NOT NULL,
            created_at TIMESTAMP WITH TIME ZONE DEFAULT CURRENT_TIMESTAMP NOT NULL,
            updated_at TIMESTAMP WITH TIME ZONE DEFAULT CURRENT_TIMESTAMP NOT NULL,
            code VARCHAR(255) NOT NULL,
            title VARCHAR(255) NOT NULL,
            description VARCHAR(4000),
            watch_num INTEGER,
            comment_num INTEGER,
            mylist_num INTEGER,
            category VARCHAR(255),
            length INTEGER,
            file_type VARCHAR(3),
            upload_time TIMESTAMP WITH TIME ZONE,
            size_high INTEGER,
            size_low INTEGER,
            CONSTRAINT pk_video PRIMARY KEY (id),
            CONSTRAINT un1_video UNIQUE (code)
        )
    ";

    sqlx::query(sql).execute(pool).await?;
    Ok(())
}

pub(crate) async fn create_tag_table(pool: &Pool<Postgres>) -> Result<(), sqlx::Error> {
    let sql = "
        CREATE TABLE IF NOT EXISTS tag (
            id SERIAL NOT NULL,
            created_at TIMESTAMP WITH TIME ZONE DEFAULT CURRENT_TIMESTAMP NOT NULL,
            updated_at TIMESTAMP WITH TIME ZONE DEFAULT CURRENT_TIMESTAMP NOT NULL,
            name VARCHAR(255) NOT NULL,
            CONSTRAINT pk_tag PRIMARY KEY (id),
            CONSTRAINT un1_tag UNIQUE (name)
        )
    ";

    sqlx::query(sql).execute(pool).await?;
    Ok(())
}

pub(crate) async fn create_relation_table(pool: &Pool<Postgres>) -> Result<(), sqlx::Error> {
    let sql = "
        CREATE TABLE IF NOT EXISTS video_tag_relation (
            id SERIAL NOT NULL,
            created_at TIMESTAMP WITH TIME ZONE DEFAULT CURRENT_TIMESTAMP NOT NULL,
            updated_at TIMESTAMP WITH TIME ZONE DEFAULT CURRENT_TIMESTAMP NOT NULL,
            video_id INTEGER NOT NULL,
            tag_id INTEGER NOT NULL,
            CONSTRAINT pk_video_tag_relation PRIMARY KEY (id),
            CONSTRAINT un1_video_tag_relation UNIQUE (video_id, tag_id),
            CONSTRAINT fk1_video_tag_relation FOREIGN KEY (video_id) REFERENCES video (id),
            CONSTRAINT fk2_video_tag_relation FOREIGN KEY (tag_id) REFERENCES tag (id)
        )
    ";

    sqlx::query(sql).execute(pool).await?;
    Ok(())
}
