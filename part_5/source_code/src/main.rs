use chrono::NaiveDate;
use dotenvy::dotenv;
use fake::faker::phone_number::raw::PhoneNumber;
use fake::locales::EN;
use fake::{
    faker::barcode::raw::Isbn10, faker::barcode::raw::Isbn13, faker::chrono::raw::Date,
    faker::company::en::CompanyName, faker::internet::raw::FreeEmail, faker::lorem::en::Sentence,
    faker::name::en::FirstName, faker::name::en::LastName, faker::number::en::Digit, Fake,
};
use rand::Rng;
use sqlx::{mysql::MySqlPoolOptions, types::BigDecimal, MySql, Pool, Row};
use std::env;
use std::{
    f32::MIN_EXP,
    process::{Child, Command},
};
use tokio;

async fn create_tables(pool: &Pool<MySql>) -> Result<(), sqlx::Error> {
    sqlx::query(
        r#"
             CREATE TABLE IF NOT EXISTS member 
                (
                Member_ID INT NOT NULL AUTO_INCREMENT,
                Date_of_Birth DATE NOT NULL,
                Email TEXT NOT NULL,
                First_Name TEXT NOT NULL,
                Last_Name TEXT NOT NULL,
                Outstanding_Balance DECIMAL(6, 2) NOT NULL,
                PRIMARY KEY (Member_ID)
                );
             "#,
    )
    .execute(pool)
    .await?;

    sqlx::query(
        r#"
             CREATE TABLE IF NOT EXISTS staff 
                (
                Member_ID INT NOT NULL,
                Salary DECIMAL(6, 2) NOT NULL,
                Work_Phone VARCHAR(15) NOT NULL,
                Start_Date DATE NOT NULL,
                Work_Email TEXT NOT NULL,
                PRIMARY KEY (Member_ID),
                FOREIGN KEY (Member_ID) REFERENCES member(Member_id) ON DELETE CASCADE
                );
             "#,
    )
    .execute(pool)
    .await?;

    sqlx::query(
        r#"
             CREATE TABLE IF NOT EXISTS author 
                (
                ID INT NOT NULL AUTO_INCREMENT,
                First_Name TEXT NOT NULL,
                Last_Name TEXT NOT NULL,
                PRIMARY KEY (ID)
                );
             "#,
    )
    .execute(pool)
    .await?;

    sqlx::query(
        r#"
             CREATE TABLE IF NOT EXISTS publisher 
                (
                ID INT NOT NULL AUTO_INCREMENT,
                Name TEXT NOT NULL,
                PRIMARY KEY (ID)
                );
             "#,
    )
    .execute(pool)
    .await?;

    sqlx::query(
        r#"
             CREATE TABLE IF NOT EXISTS book 
                (
                ISBN VARCHAR(20) NOT NULL,
                Title TEXT NOT NULL,
                PRIMARY KEY (ISBN)
                );
             "#,
    )
    .execute(pool)
    .await?;

    sqlx::query(
        r#"
             CREATE TABLE IF NOT EXISTS book_series 
                (
                ID INT NOT NULL AUTO_INCREMENT,
                NAME TEXT NOT NULL,
                PRIMARY KEY (ID)
                );
             "#,
    )
    .execute(pool)
    .await?;

    sqlx::query(
        r#"
             CREATE TABLE IF NOT EXISTS director 
                (
                ID INT NOT NULL AUTO_INCREMENT,
                First_Name TEXT NOT NULL,
                Last_Name TEXT NOT NULL,
                PRIMARY KEY (ID)
                );
             "#,
    )
    .execute(pool)
    .await?;

    sqlx::query(
        r#"
             CREATE TABLE IF NOT EXISTS studio 
                (
                ID INT NOT NULL AUTO_INCREMENT,
                Name TEXT NOT NULL,
                PRIMARY KEY (ID)
                );
             "#,
    )
    .execute(pool)
    .await?;

    sqlx::query(
        r#"
             CREATE TABLE IF NOT EXISTS movie 
                (
                ISAN VARCHAR(24) NOT NULL,
                Title TEXT NOT NULL,
                PRIMARY KEY (ISAN)
                );
             "#,
    )
    .execute(pool)
    .await?;

    sqlx::query(
        r#"
             CREATE TABLE IF NOT EXISTS material 
                (
                ID INT NOT NULL AUTO_INCREMENT,
                PRIMARY KEY (ID)
                );
             "#,
    )
    .execute(pool)
    .await?;

    sqlx::query(
        r#"
             CREATE TABLE IF NOT EXISTS book_copy 
                (
                ID INT NOT NULL,
                PRIMARY KEY (ID),
                FOREIGN KEY (ID) REFERENCES material(ID) ON DELETE CASCADE
                );
             "#,
    )
    .execute(pool)
    .await?;

    sqlx::query(
        r#"
             CREATE TABLE IF NOT EXISTS movie_copy 
                (
                ID INT NOT NULL,
                PRIMARY KEY (ID),
                FOREIGN KEY (ID) REFERENCES material(ID) ON DELETE CASCADE
                );
             "#,
    )
    .execute(pool)
    .await?;

    sqlx::query(
        r#"
             CREATE TABLE IF NOT EXISTS room 
                (
                Number INT NOT NULL AUTO_INCREMENT,
                Capacity INT NOT NULL,
                PRIMARY KEY (Number)
                );
             "#,
    )
    .execute(pool)
    .await?;

    sqlx::query(
        r#"
             CREATE TABLE IF NOT EXISTS community_event 
                (
                ID INT NOT NULL AUTO_INCREMENT,
                Start_Time DATETIME NOT NULL,
                End_Time DATETIME NOT NULL,
                Longitude FLOAT NOT NULL,
                Latitude FLOAT NOT NULL,
                Capacity INT NOT NULL,
                PRIMARY KEY (Number)
                );
             "# ,
    )
    .execute(pool)
    .await?;

    Ok(())
}

async fn populate_tables(pool: &Pool<MySql>) -> Result<(), sqlx::Error> {
    let mut rng = rand::thread_rng();

    for _ in 1..100 {
        let fake_date: NaiveDate = Date(EN).fake();
        let fake_email: String = FreeEmail(EN).fake();
        let fake_first_name: String = FirstName().fake();
        let fake_last_name: String = LastName().fake();
        let mut fake_balance: f64 = rng.gen_range(-9999.99..0.00);
        fake_balance = (fake_balance * 100.0).floor() / 100.0;

        sqlx::query(
            r#"
                INSERT INTO member (Date_of_Birth, Email, First_Name, Last_Name, Outstanding_Balance) 
                VALUES
                (?, ?, ?, ?, ?)
                "#,
        )
        .bind(fake_date)
        .bind(fake_email)
        .bind(fake_first_name)
        .bind(fake_last_name)
        .bind(fake_balance)
        .execute(pool)
        .await?;
    }

    let members: Vec<(i32,)> = sqlx::query_as("SELECT member_id FROM member")
        .fetch_all(pool)
        .await?;

    for member in members {
        let one_in_four = rng.gen_range(0..4) == 0;

        if one_in_four {
            let mut fake_salary: f64 = rng.gen_range(0.00..9999.99);
            fake_salary = (fake_salary * 100.0).floor() / 100.0;

            let fake_email: String = FreeEmail(EN).fake();
            let fake_date: NaiveDate = Date(EN).fake();

            let area_code: u16 = rng.gen_range(100..1000); // (xxx)
            let prefix: u16 = rng.gen_range(100..1000); // xxx-
            let line_number: u16 = rng.gen_range(1000..10000); // xxxx

            // Format the phone number
            let fake_phone_number = format!("({})-{}-{}", area_code, prefix, line_number);

            sqlx::query(
                r#"
                    INSERT INTO staff (Member_id, Salary, Work_Phone, Start_Date, Work_Email) 
                    VALUES
                    (?, ?, ?, ?, ?)
                    "#,
            )
            .bind(member.0)
            .bind(fake_salary)
            .bind(fake_phone_number)
            .bind(fake_date)
            .bind(fake_email)
            .execute(pool)
            .await?;
        }
    }

    for _ in 1..50 {
        let fake_first_name: String = FirstName().fake();
        let fake_last_name: String = LastName().fake();

        sqlx::query(
            r#"
                INSERT INTO author (First_Name, Last_name) 
                VALUES
                (?, ?)
                "#,
        )
        .bind(fake_first_name)
        .bind(fake_last_name)
        .execute(pool)
        .await?;
    }

    for _ in 1..25 {
        let fake_company_name: String = CompanyName().fake();

        sqlx::query(
            r#"
                INSERT INTO publisher (Name) 
                VALUES
                (?)
                "#,
        )
        .bind(fake_company_name)
        .execute(pool)
        .await?;
    }

    for _ in 1..100 {
        let book_title: String = Sentence(1..20).fake();

        let isbn_string: String = if rand::random() {
            Isbn10(EN).fake()
        } else {
            Isbn13(EN).fake()
        };

        sqlx::query(
            r#"
                INSERT INTO book (ISBN, Title) 
                VALUES
                (?, ?)
                "#,
        )
        .bind(isbn_string)
        .bind(book_title)
        .execute(pool)
        .await?;
    }

    for _ in 1..20 {
        let book_series_name: String = Sentence(1..20).fake();

        sqlx::query(
            r#"
                INSERT INTO book_series (Name) 
                VALUES
                (?)
                "#,
        )
        .bind(book_series_name)
        .execute(pool)
        .await?;
    }

    for _ in 1..50 {
        let fake_first_name: String = FirstName().fake();
        let fake_last_name: String = LastName().fake();

        sqlx::query(
            r#"
                INSERT INTO director (First_Name, Last_name) 
                VALUES
                (?, ?)
                "#,
        )
        .bind(fake_first_name)
        .bind(fake_last_name)
        .execute(pool)
        .await?;
    }

    for _ in 1..25 {
        let fake_company_name: String = CompanyName().fake();

        sqlx::query(
            r#"
                INSERT INTO studio (Name) 
                VALUES
                (?)
                "#,
        )
        .bind(fake_company_name)
        .execute(pool)
        .await?;
    }

    for _ in 1..100 {
        let book_title: String = Sentence(1..20).fake();

        let bytes: [u8; 12] = rand::random();
        let isan: String = bytes.iter().map(|b| format!("{b:02x}")).collect();

        sqlx::query(
            r#"
                INSERT INTO movie (ISAN, Title) 
                VALUES
                (?, ?)
                "#,
        )
        .bind(isan)
        .bind(book_title)
        .execute(pool)
        .await?;
    }

    for i in 1..1000 {
        sqlx::query(
            r#"
                INSERT INTO material ()
                VALUES
                ()
                "#,
        )
        .execute(pool)
        .await?;

        if rand::random() {
            sqlx::query(
                r#"
                    INSERT INTO book_copy (ID)
                    VALUES
                    (?)
                    "#,
            )
            .bind(i)
            .execute(pool)
            .await?;
        } else {
            sqlx::query(
                r#"
                    INSERT INTO movie_copy (ID)
                    VALUES
                    (?)
                    "#,
            )
            .bind(i)
            .execute(pool)
            .await?;
        }
    }
    for _ in 1..10 {
        sqlx::query(
            r#"
                INSERT INTO room (Capacity)
                VALUES
                (?)
                "#,
        )
        .bind(rng.gen_range(1..100))
        .execute(pool)
        .await?;
    }

    Ok(())
}

#[tokio::main]
async fn main() -> Result<(), sqlx::Error> {
    /**************************************
    Local Variables
    **************************************/

    /* Load the variables from the environment file */
    dotenv().ok();

    /* Fetch the Database URL and ssh username/password */
    let database_url = env::var("DATABASE_URL").expect("DATABASE_URL not found");
    let ssh_username = env::var("SSH_USR").expect("SSH_USR not found");
    let ssh_password = env::var("SSH_PWD").expect("SSH_PWD not found");

    let mut ssh_process = Command::new("sshpass")
        .arg("-p".to_string() + &ssh_password)
        .arg("ssh")
        .arg("-N")
        .arg("-L")
        .arg("3307:mysql.eecs.ku.edu:3306")
        .arg(ssh_username + "@cycle3.eecs.ku.edu")
        .spawn()
        .expect("Failed to start command");

    // Create connection pool
    let pool = MySqlPoolOptions::new()
        .max_connections(5)
        .connect(&database_url)
        .await?;

    for row in [
        "staff",
        "member",
        "book",
        "book_series",
        "publisher",
        "author",
        "director",
        "studio",
        "book_copy",
        "movie_copy",
        "material",
        "room",
        "community_event"
    ] {
        let query = format!("DROP TABLE IF EXISTS {};", row);

        sqlx::query(&query).execute(&pool).await?;
    }

    create_tables(&pool).await?;
    populate_tables(&pool).await?;

    ssh_process.kill().expect("Failed to kill process");
    Ok(())
}
