
use std::{f32::MIN_EXP, process::{Child, Command}};
use sqlx::{mysql::MySqlPoolOptions, Pool, MySql, Row};
use fake::{Fake, faker::company::en::CompanyName, faker::lorem::en::Sentence, faker::number::en::Digit};
use tokio;
use dotenvy::dotenv;
use std::env;

#[tokio::main]
async fn main() -> Result<(), sqlx::Error> {
    // Load environment variables
    dotenv().ok();
    let database_url = env::var("DATABASE_URL").expect("DATABASE_URL not found");
    let ssh_username = env::var("SSH_USR").expect("SSH_USR not found");
    let ssh_password = env::var("SSH_PWD").expect("SSH_PWD not found");

    let mut ssh_process = Command::new( "sshpass" )
                                            .arg( "-p" .to_string() + &ssh_password  )
                                            .arg( "ssh" )
                                            .arg( "-N"  )
                                            .arg( "-L"  )
                                            .arg( "3307:mysql.eecs.ku.edu:3306"  )
                                            .arg( ssh_username + "@cycle3.eecs.ku.edu" )
                                            .spawn()
                                            .expect("Failed to start command");

    // Create connection pool
    let pool = MySqlPoolOptions::new()
        .max_connections(5)
        .connect(&database_url)
        .await?;

    let rows = sqlx::query( "SHOW TABLES;" ).fetch_all( &pool ).await?;

    for row in rows {
        let table_name: String = row.get(0 ); // Replace with actual column name
        let query = format!("DROP TABLE {};", table_name);
        
        println!("{}", query); // Debug output
        sqlx::query(&query).execute(&pool).await?;
    }

    // Create a table for books
    sqlx::query(
        r#"
        CREATE TABLE IF NOT EXISTS books (
            id INT AUTO_INCREMENT PRIMARY KEY,
            title VARCHAR(255) NOT NULL,
            author VARCHAR(255) NOT NULL,
            publisher VARCHAR(255) NOT NULL
        );
        "#,
    )
    .execute(&pool)
    .await?;

    // Insert fake data
    for _ in 0..10 {
        let title: String = Sentence(3..6).fake();
        let author: String = CompanyName().fake();
        //let isbn: String = (0..13).map(|_| Digit());
        let publisher: String = CompanyName().fake();
        //println!( "{}", isbn );
        sqlx::query("INSERT INTO books (title, author, publisher) VALUES (?, ?, ?)")
            .bind(title)
            .bind(author)
            //.bind(isbn)
            .bind(publisher)
            .execute(&pool)
            .await?;
    }

    // Fetch and display inserted data
    let books: Vec<(i32, String, String, String)> = sqlx::query_as("SELECT * FROM books")
        .fetch_all(&pool)
        .await?;

    for book in books {
        println!("{:?}", book);
    }
    ssh_process.kill().expect( "Failed to kill process" );
    Ok(())
}
