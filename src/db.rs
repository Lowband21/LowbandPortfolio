use crate::models::*;
use sqlx::Error as SqlxError;
use sqlx::PgConnection;

pub async fn get_all_projects(conn: &mut PgConnection) -> Result<Vec<Project>, SqlxError> {
    sqlx::query_as!(Project, "SELECT * FROM project")
        .fetch_all(conn)
        .await
}

pub async fn get_all_skills(conn: &mut PgConnection) -> Result<Vec<Skill>, SqlxError> {
    sqlx::query_as!(Skill, "SELECT * FROM skill")
        .fetch_all(conn)
        .await
}

pub async fn get_all_bio(conn: &mut PgConnection) -> Result<Bio, SqlxError> {
    let bio = sqlx::query_as!(Bio, r#"SELECT id, bio_content FROM bio LIMIT 1"#)
        .fetch_one(conn)
        .await?;

    Ok(bio)
}

use crate::models::{Bio, Skill};

pub async fn fill_bio(conn: &mut sqlx::PgConnection) -> Result<Bio, SqlxError> {
    // Insert a new bio entry
    let bio_content = "I'm a meticulous and driven Computer Science student from the University of Denver, boasting a robust academic performance. My toolkit of programming languages spans Python, Java, C&C++, Rust, Javascript and Haskell, with a particular focus on Rust and Python, owing to my extensive practical experience. Beyond programming, I also bring proficiency in Linux system administration, Docker, Virtualization, and Shell Scripting. As a technophile, my passion lies in leveraging these skills to solve complex problems and innovate.";
    let inserted_bio: Bio = sqlx::query_as!(
        Bio,
        r#"
    INSERT INTO bio (bio_content)
    VALUES ($1)
    RETURNING id, bio_content as "bio_content"
    "#,
        bio_content
    )
    .fetch_one(conn)
    .await?;

    Ok(inserted_bio)
}

pub async fn fill_skills(conn: &mut sqlx::PgConnection) -> Result<(), SqlxError> {
    // Insert several skills
    let skills = vec![
        "Rust",
        "SQLx",
        "Actix-Web",
        "Python",
        "Javascript",
        "Java",
        "C&C++",
        "Haskell",
        "Git",
        "Regex",
        "Markdown",
        "Modal Editing",
        "SQL",
        "Embedded Systems",
        "Networking",
        "Artificial Intelligence",
        "Linux System Administration",
        "Docker",
        "Virtualization",
        "Shell Scripting",
        "Automation",
        "Web Development",
    ];
    for skill_name in skills.iter() {
        let _: Skill = sqlx::query_as!(
            Skill,
            r#"
            INSERT INTO skill (name)
            VALUES ($1)
            RETURNING id, name
            "#,
            skill_name
        )
        .fetch_one(&mut *conn)
        .await?;
    }

    Ok(())
}
