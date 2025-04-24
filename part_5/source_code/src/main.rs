use chrono::{Duration, NaiveDate, NaiveDateTime, NaiveTime, TimeDelta};
use dotenvy::dotenv;
use fake::locales::EN;
use fake::{
    faker::barcode::raw::Isbn10, faker::barcode::raw::Isbn13, faker::chrono::raw::Date,
    faker::chrono::raw::DateTime, faker::chrono::raw::Time, faker::company::en::CompanyName,
    faker::internet::raw::FreeEmail, faker::lorem::en::Sentence, faker::name::en::FirstName,
    faker::name::en::LastName, Fake,
};
use rand::seq::SliceRandom;
use rand::Rng;
use sqlx::{mysql::MySqlPoolOptions, MySql, Pool};
use std::collections::HashMap;
use std::env;
use std::process::Command;

fn random_at_least_once<'a, I: Clone + 'a>(
    iter: impl IntoIterator<Item = I>,
    random: &'a mut (impl Rng + 'a),
) -> impl IntoIterator<Item = I> + 'a {
    let items: Vec<_> = iter.into_iter().collect();
    let mut unused_indexes: Vec<_> = (0..items.len()).collect();
    unused_indexes.shuffle(random);
    std::iter::from_fn(move || {
        let index = if let Some(index) = unused_indexes.pop() {
            index
        } else {
            random.gen_range(0..items.len())
        };

        Some(items[index].clone())
    })
}

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
                PRIMARY KEY (ID)
                );
             "#,
    )
    .execute(pool)
    .await?;

    sqlx::query(
        r#"
            CREATE TABLE IF NOT EXISTS reserves_room (
                Room_ID INT NOT NULL,
                Member_ID INT NOT NULL,
                Duration INT NOT NULL,
                Date DATETIME NOT NULL,
                PRIMARY KEY (Room_ID, Member_ID),
                FOREIGN KEY (Room_ID) REFERENCES room(Number) ON DELETE CASCADE,
                FOREIGN KEY (Member_ID) REFERENCES member(Member_ID) ON DELETE CASCADE
            );
        "#,
    )
    .execute(pool)
    .await?;

    sqlx::query(
        r#"
            CREATE TABLE IF NOT EXISTS organizes (
                Staff_ID INT NOT NULL,
                Community_Event_ID INT NOT NULL,
                PRIMARY KEY (Staff_ID, Community_Event_ID),
                FOREIGN KEY (Staff_ID) REFERENCES staff(Member_ID) ON DELETE CASCADE,
                FOREIGN KEY (Community_Event_ID) REFERENCES community_event(ID) ON DELETE CASCADE
            );
        "#,
    )
    .execute(pool)
    .await?;

    sqlx::query(
        r#"
            CREATE TABLE IF NOT EXISTS registers (
                Member_ID INT NOT NULL,
                Community_Event_ID INT NOT NULL,
                PRIMARY KEY (Member_ID, Community_Event_ID),
                FOREIGN KEY (Member_ID) REFERENCES member(Member_ID) ON DELETE CASCADE,
                FOREIGN KEY (Community_Event_ID) REFERENCES community_event(ID) ON DELETE CASCADE
            );
        "#,
    )
    .execute(pool)
    .await?;

    sqlx::query(
        r#"
            CREATE TABLE IF NOT EXISTS loans (
                Member_ID INT NOT NULL,
                Material_ID INT NOT NULL,
                Duration INT,
                Start_Date DATE NOT NULL,
                PRIMARY KEY (Member_ID, Material_ID),
                FOREIGN KEY (Member_ID) REFERENCES member(Member_ID) ON DELETE CASCADE,
                FOREIGN KEY (Material_ID) REFERENCES material(ID) ON DELETE CASCADE
            );
        "#,
    )
    .execute(pool)
    .await?;

    sqlx::query(
        r#"
        CREATE TABLE IF NOT EXISTS donates (
            Member_ID INT NOT NULL,
            Material_ID INT NOT NULL,
            PRIMARY KEY (Member_ID, Material_ID),
            FOREIGN KEY (Member_ID) REFERENCES member(Member_ID) ON DELETE CASCADE,
            FOREIGN KEY (Material_ID) REFERENCES material(ID) ON DELETE CASCADE
        );
    "#,
    )
    .execute(pool)
    .await?;

    sqlx::query(
        r#"
            CREATE TABLE IF NOT EXISTS reserves_material (
                Member_ID INT NOT NULL,
                Material_ID INT NOT NULL,
                Reservation_Date DATETIME NOT NULL,
                PRIMARY KEY (Member_ID, Material_ID),
                FOREIGN KEY (Member_ID) REFERENCES member(Member_ID) ON DELETE CASCADE,
                FOREIGN KEY (Material_ID) REFERENCES material(ID) ON DELETE CASCADE
            );
        "#,
    )
    .execute(pool)
    .await?;

    sqlx::query(
        r#"
        CREATE TABLE IF NOT EXISTS searches_book (
            Member_ID INT NOT NULL,
            Book_ID VARCHAR(20) NOT NULL,
            PRIMARY KEY (Member_ID, Book_ID),
            FOREIGN KEY (Member_ID) REFERENCES member(Member_ID) ON DELETE CASCADE,
            FOREIGN KEY (Book_ID) REFERENCES book(ISBN) ON DELETE CASCADE
        );
    "#,
    )
    .execute(pool)
    .await?;

    sqlx::query(
        r#"
            CREATE TABLE IF NOT EXISTS searches_movie (
                Member_ID INT NOT NULL,
                Movie_ID VARCHAR(24) NOT NULL,
                PRIMARY KEY (Member_ID, Movie_ID),
                FOREIGN KEY (Member_ID) REFERENCES member(Member_ID) ON DELETE CASCADE,
                FOREIGN KEY (Movie_ID) REFERENCES movie(ISAN) ON DELETE CASCADE
            );
        "#,
    )
    .execute(pool)
    .await?;

    sqlx::query(
        r#"
        CREATE TABLE IF NOT EXISTS book_has (
            Copy_ID INT NOT NULL,
            Book_ID VARCHAR(20) NOT NULL,
            PRIMARY KEY (Copy_ID),
            FOREIGN KEY (Copy_ID) REFERENCES book_copy(ID) ON DELETE CASCADE,
            FOREIGN KEY (Book_ID) REFERENCES book(ISBN) ON DELETE CASCADE
        );
    "#,
    )
    .execute(pool)
    .await?;

    sqlx::query(
        r#"
            CREATE TABLE IF NOT EXISTS writes (
                Author_ID INT NOT NULL,
                Book_ID VARCHAR(20) NOT NULL,
                PRIMARY KEY (Author_ID, Book_ID),
                FOREIGN KEY (Author_ID) REFERENCES author(ID) ON DELETE CASCADE,
                FOREIGN KEY (Book_ID) REFERENCES book(ISBN) ON DELETE CASCADE
            );
        "#,
    )
    .execute(pool)
    .await?;

    sqlx::query(
        r#"
        CREATE TABLE IF NOT EXISTS publishes (
            Publisher_ID INT NOT NULL,
            Book_ID VARCHAR(20) NOT NULL,
            Publish_Date DATE NOT NULL,
            PRIMARY KEY (Book_ID),
            FOREIGN KEY (Publisher_ID) REFERENCES publisher(ID) ON DELETE CASCADE,
            FOREIGN KEY (Book_ID) REFERENCES book(ISBN) ON DELETE CASCADE
        );
    "#,
    )
    .execute(pool)
    .await?;

    sqlx::query(
        r#"
            CREATE TABLE IF NOT EXISTS is_part_of (
                Book_ID VARCHAR(20) NOT NULL,
                Book_Series_ID INT NOT NULL,
                Seq_Order INT NOT NULL,
                PRIMARY KEY (Book_ID),
                FOREIGN KEY (Book_ID) REFERENCES book(ISBN) ON DELETE CASCADE,
                FOREIGN KEY (Book_Series_ID) REFERENCES book_series(ID) ON DELETE CASCADE
            );
        "#,
    )
    .execute(pool)
    .await?;

    sqlx::query(
        r#"
            CREATE TABLE IF NOT EXISTS directs (
                Director_ID INT NOT NULL,
                Movie_ID VARCHAR(24) NOT NULL,
                PRIMARY KEY (Director_ID, Movie_ID),
                FOREIGN KEY (Director_ID) REFERENCES director(ID) ON DELETE CASCADE,
                FOREIGN KEY (Movie_ID) REFERENCES movie(ISAN) ON DELETE CASCADE
            );
        "#,
    )
    .execute(pool)
    .await?;

    sqlx::query(
        r#"
            CREATE TABLE IF NOT EXISTS releases (
                Studio_ID INT NOT NULL,
                Movie_ID VARCHAR(24) NOT NULL,
                Release_Date DATE NOT NULL,
                PRIMARY KEY (Movie_ID),
                FOREIGN KEY (Studio_ID) REFERENCES studio(ID) ON DELETE CASCADE,
                FOREIGN KEY (Movie_ID) REFERENCES movie(ISAN) ON DELETE CASCADE
            );
        "#,
    )
    .execute(pool)
    .await?;

    sqlx::query(
        r#"
            CREATE TABLE IF NOT EXISTS movie_has (
                Copy_ID INT NOT NULL,
                Movie_ID VARCHAR(24) NOT NULL,
                PRIMARY KEY (Copy_ID),
                FOREIGN KEY (Copy_ID) REFERENCES movie_copy(ID) ON DELETE CASCADE,
                FOREIGN KEY (Movie_ID) REFERENCES movie(ISAN) ON DELETE CASCADE
            );
        "#,
    )
    .execute(pool)
    .await?;

    Ok(())
}

async fn populate_tables(pool: &Pool<MySql>) -> Result<(), sqlx::Error> {
    let mut rng = rand::thread_rng();
    let library_open_date: NaiveDate = NaiveDate::from_ymd_opt(2022, 9, 28).unwrap();
    let current_day: NaiveDate = NaiveDate::from_ymd_opt(2025, 4, 23).unwrap();
    let oldest_person_date: NaiveDate = NaiveDate::from_ymd_opt(1965, 1, 6).unwrap();
    let youngest_person_date: NaiveDate = NaiveDate::from_ymd_opt(2027, 4, 23).unwrap();

    let days_age_range = (youngest_person_date - oldest_person_date).num_days();
    let days_library_open_range = (current_day - library_open_date).num_days();

    for _ in 1..100 {
        print!("{}", days_age_range);
        let rand_range = rng.gen_range(1..days_age_range);
        let fake_date: NaiveDate = oldest_person_date + Duration::days(rand_range);

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

    for member in &members {
        let one_in_four = rng.gen_range(0..10) == 0;

        if one_in_four {
            let mut fake_salary: f64 = rng.gen_range(0.00..9999.99);
            fake_salary = (fake_salary * 100.0).floor() / 100.0;

            let fake_email: String = FreeEmail(EN).fake();
            let rand_range = rng.gen_range(0..days_library_open_range);
            let fake_date: NaiveDate = library_open_date + Duration::days(rand_range);

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
        let movie_title: String = Sentence(1..20).fake();

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
        .bind(movie_title)
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

    for _ in 1..10 {
        let rand_start_date_offset = rng.gen_range(1..days_library_open_range);
        let random_time_offset = rng.gen_range(1..720);

        let fake_start_time: NaiveDateTime = NaiveDateTime::new(
            library_open_date + Duration::days(rand_start_date_offset),
            NaiveTime::from_hms_opt(0, 0, 0).expect("Invalid time"),
        )
        .checked_add_signed(Duration::minutes(random_time_offset))
        .unwrap();

        let random_minutes: i64 = rng.gen_range(1..1000);
        let fake_end_time: NaiveDateTime = fake_start_time + Duration::minutes(random_minutes);
        let fake_latitude: f32 = rng.gen_range(-90.0..90.0);
        let fake_longitude: f32 = rng.gen_range(-180.0..180.0);

        sqlx::query(
            r#"
                INSERT INTO community_event (Start_Time, End_Time, Longitude, Latitude)
                VALUES
                (?, ?, ?, ?)
                "#,
        )
        .bind(fake_start_time)
        .bind(fake_end_time)
        .bind(fake_longitude)
        .bind(fake_latitude)
        .execute(pool)
        .await?;
    }

    let staff_tup: Vec<(i32,)> = sqlx::query_as("SELECT member_id FROM staff")
        .fetch_all(pool)
        .await?;

    let staff: Vec<i32> = staff_tup.iter().map(|(x,)| *x).collect();

    let member_conv: Vec<i32> = members.iter().map(|(x,)| *x).collect();

    for i in 1..10 {
        let mut shuffled_staff = staff.clone();
        shuffled_staff.shuffle(&mut rng);

        let mut shuffled_members = member_conv.clone();
        shuffled_members.shuffle(&mut rng);

        let unique_random_staff: Vec<i32> = shuffled_staff
            .into_iter()
            .take(rng.gen_range(1..staff.len()))
            .collect();

        let unique_random_members: Vec<i32> = shuffled_members
            .into_iter()
            .take(rng.gen_range(0..member_conv.len()))
            .collect();

        for s in unique_random_staff {
            sqlx::query(
                r#"
                    INSERT INTO organizes (Staff_ID, Community_Event_ID)
                    VALUES
                    (?, ?)
                    "#,
            )
            .bind(s)
            .bind(i)
            .execute(pool)
            .await?;
        }

        for m in unique_random_members {
            sqlx::query(
                r#"
                    INSERT INTO registers (Member_ID, Community_Event_ID)
                    VALUES
                    (?, ?)
                    "#,
            )
            .bind(m)
            .bind(i)
            .execute(pool)
            .await?;
        }
    }

    for i in 1..1000 {
        let rand_member: i32 = rng.gen_range(1..100);
        sqlx::query(
            r#"
                INSERT INTO donates (Member_ID, Material_ID)
                VALUES
                (?, ?)
                "#,
        )
        .bind(rand_member)
        .bind(i)
        .execute(pool)
        .await?;
    }

    let book_ids_tup: Vec<(String,)> = sqlx::query_as("SELECT ISBN FROM book")
        .fetch_all(pool)
        .await?;

    let book_ids: Vec<String> = book_ids_tup.iter().map(|(x,)| x.clone()).collect();

    let movie_ids_tup: Vec<(String,)> = sqlx::query_as("SELECT ISAN FROM movie")
        .fetch_all(pool)
        .await?;

    let movie_ids: Vec<String> = movie_ids_tup.iter().map(|(x,)| x.clone()).collect();

    let member_tup: Vec<(i32,)> = sqlx::query_as("SELECT member_id FROM member")
        .fetch_all(pool)
        .await?;

    let members: Vec<i32> = member_tup.iter().map(|(x,)| *x).collect();

    for bid in &book_ids {
        let rand_range = rng.gen_range(0..members.len());
        let selected_members: Vec<&i32> = members.choose_multiple(&mut rng, rand_range).collect();
        for member in selected_members {
            if rand::random() {
                sqlx::query(
                    r#"
                        INSERT INTO searches_book (Member_ID, Book_ID)
                        VALUES
                        (?, ?)
                        "#,
                )
                .bind(member)
                .bind(bid.to_string())
                .execute(pool)
                .await?;
            }
        }
    }

    for mid in &movie_ids {
        let rand_range = rng.gen_range(0..members.len());
        let selected_members: Vec<&i32> = members.choose_multiple(&mut rng, rand_range).collect();
        for member in selected_members {
            if rand::random() {
                sqlx::query(
                    r#"
                        INSERT INTO searches_movie (Member_ID, Movie_ID)
                        VALUES
                        (?, ?)
                        "#,
                )
                .bind(member)
                .bind(mid.to_string())
                .execute(pool)
                .await?;
            }
        }
    }

    let author_tup: Vec<(i32,)> = sqlx::query_as("SELECT id FROM author")
        .fetch_all(pool)
        .await?;

    let authors: Vec<i32> = author_tup.iter().map(|(x,)| *x).collect();
    for bid in &book_ids {
        let rand_range = rng.gen_range(1..members.len());
        let selected_authors: Vec<&i32> = authors.choose_multiple(&mut rng, rand_range).collect();
        for author in selected_authors {
            sqlx::query(
                r#"
                        INSERT INTO writes (Author_ID, Book_ID)
                        VALUES
                        (?, ?)
                        "#,
            )
            .bind(author)
            .bind(bid.to_string())
            .execute(pool)
            .await?;
        }
    }

    let director_tup: Vec<(i32,)> = sqlx::query_as("SELECT id FROM director")
        .fetch_all(pool)
        .await?;

    let directors: Vec<i32> = director_tup.iter().map(|(x,)| *x).collect();

    for mid in &movie_ids {
        let rand_range = rng.gen_range(1..members.len());
        let selected_directors: Vec<&i32> =
            directors.choose_multiple(&mut rng, rand_range).collect();
        for director in selected_directors {
            sqlx::query(
                r#"
                        INSERT INTO directs (Director_ID, Movie_ID)
                        VALUES
                        (?, ?)
                        "#,
            )
            .bind(director)
            .bind(mid.to_string())
            .execute(pool)
            .await?;
        }
    }

    let book_copy_tup: Vec<(i32,)> = sqlx::query_as("SELECT id FROM book_copy")
        .fetch_all(pool)
        .await?;

    let book_copies: Vec<i32> = book_copy_tup.iter().map(|(x,)| *x).collect();

    let rand_books = random_at_least_once(&book_ids, &mut rng);
    for (copy, book_id) in book_copies.iter().zip(rand_books) {
        sqlx::query(
            r#"
                INSERT INTO book_has (Copy_ID, Book_ID)
                VALUES
                (?, ?)
                "#,
        )
        .bind(copy)
        .bind(book_id)
        .execute(pool)
        .await?;
    }

    let movie_copy_tup: Vec<(i32,)> = sqlx::query_as("SELECT id FROM movie_copy")
        .fetch_all(pool)
        .await?;

    let movie_copies: Vec<i32> = movie_copy_tup.iter().map(|(x,)| *x).collect();
    let rand_movies = random_at_least_once(&movie_ids, &mut rng);
    for (copy, movie_id) in movie_copies.iter().zip(rand_movies) {
        sqlx::query(
            r#"
                INSERT INTO movie_has (Copy_ID, Movie_ID)
                VALUES
                (?, ?)
                "#,
        )
        .bind(copy)
        .bind(movie_id)
        .execute(pool)
        .await?;
    }

    let mut hash_table: HashMap<i32, i32> = HashMap::new();

    let series_ids = 1..20;
    let random_series = random_at_least_once(series_ids, &mut rng);
    for (book_id, rand_series) in book_ids[..book_ids.len() / 2].iter().zip(random_series) {
        // Insert if not present
        hash_table.entry(rand_series).or_insert(1);

        // Get a mutable reference to the value
        if let Some(temp) = hash_table.get_mut(&rand_series) {
            sqlx::query(
                r#"
                    INSERT INTO is_part_of (Book_ID, Book_Series_ID, Seq_Order)
                    VALUES
                    (?, ?, ?)
                "#,
            )
            .bind(book_id)
            .bind(rand_series)
            .bind(*temp)
            .execute(pool)
            .await?;

            // Increment the value in the HashMap
            *temp += 1;
        }
    }

    let publisher_ids = 1..25;
    let rand_publishers = random_at_least_once(publisher_ids, &mut rng);
    for (book, rand_publisher) in book_ids.iter().zip(rand_publishers) {
        let mut rand_date: NaiveDate = Date(EN).fake();

        while rand_date > library_open_date {
            rand_date = Date(EN).fake();
        }

        sqlx::query(
            r#"
                INSERT INTO publishes (Publisher_ID, Book_ID, Publish_Date)
                VALUES
                (?, ?, ?)
            "#,
        )
        .bind(rand_publisher)
        .bind(book)
        .bind(rand_date)
        .execute(pool)
        .await?;
    }

    let studio_ids = 1..25;
    let rand_studios = random_at_least_once(studio_ids, &mut rng);
    for (movie, rand_studio) in movie_ids.iter().zip(rand_studios) {
        let mut rand_date: NaiveDate = Date(EN).fake();

        while rand_date > library_open_date {
            rand_date = Date(EN).fake();
        }

        sqlx::query(
            r#"
                INSERT INTO releases (Studio_ID, Movie_ID, Release_Date)
                VALUES
                (?, ?, ?)
            "#,
        )
        .bind(rand_studio)
        .bind(movie)
        .bind(rand_date)
        .execute(pool)
        .await?;
    }

    for room in 1..10 {
        let mut room_start_time = library_open_date;

        for member in &member_conv {
            if rand::random() {
                let random_time = rng.gen_range(1..120);
                sqlx::query(
                    r#"
                        INSERT INTO reserves_room (Room_ID, Member_ID, Duration, Date)
                        VALUES
                        (?, ?, ?, ?)
                    "#,
                )
                .bind(room)
                .bind(member)
                .bind(random_time)
                .bind(room_start_time)
                .execute(pool)
                .await?;
                room_start_time += Duration::minutes(random_time) + Duration::days(rng.gen_range(0..3));
            }
        }
    }

    for material in 1..999 {
        let mut fake_start_time = library_open_date;
        for member in &member_conv {
            if rand::random() {
                let random_time: Option<i64> =
                    rand::random::<bool>().then(|| rng.gen_range(0..20000));
                sqlx::query(
                    r#"
                        INSERT INTO loans (Member_ID, Material_ID, Duration, Start_Date)
                        VALUES
                        (?, ?, ?, ?)
                    "#,
                )
                .bind(member)
                .bind(material)
                .bind(random_time)
                .bind(fake_start_time)
                .execute(pool)
                .await?;

                if let Some(random_time) = random_time {
                    fake_start_time += Duration::minutes(random_time);
                } else {
                    break;
                }
            }
        }
    }

    for material in 1..999 {
        let mut shuffled_members = member_conv.clone();
        shuffled_members.shuffle(&mut rng);

        let unique_random_members: Vec<i32> = shuffled_members
            .into_iter()
            .take(rng.gen_range(0..5))
            .collect();

        for member in &unique_random_members {
            let rand_range = rng.gen_range(0..days_library_open_range);

            let fake_start_time: NaiveDateTime = NaiveDateTime::new(
                library_open_date + Duration::days(rand_range),
                NaiveTime::from_hms_opt(
                    rng.gen_range(0..24),
                    rng.gen_range(0..60),
                    rng.gen_range(0..60),
                )
                .expect("Invalid time"),
            );

            sqlx::query(
                r#"
                        INSERT INTO reserves_material (Member_ID, Material_ID, Reservation_Date)
                        VALUES
                        (?, ?, ?)
                    "#,
            )
            .bind(member)
            .bind(material)
            .bind(fake_start_time)
            .execute(pool)
            .await?;
        }
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
        "reserves_room",     // Need to validate Dates
        "organizes",         //
        "registers",         //
        "loans",             // Need to Validate dates
        "donates",           //
        "reserves_material", // Need to Validate Dates
        "searches_book",     //
        "searches_movie",    //
        "book_has",          //
        "writes",            //
        "publishes",         // Need to validate dates
        "is_part_of",        //
        "directs",           //
        "releases",          // Need to validate dates
        "movie_has",         //
        "staff",             //
        "member",            // Need to validate dates
        "book",              //
        "book_series",       //
        "movie",             //
        "publisher",         //
        "author",            //
        "director",          //
        "studio",            //
        "book_copy",         //
        "movie_copy",        //
        "material",          //
        "room",              //
        "community_event",   // Need to validate dates
    ] {
        let query = format!("DROP TABLE IF EXISTS {};", row);

        sqlx::query(&query).execute(&pool).await?;
    }

    create_tables(&pool).await?;
    populate_tables(&pool).await?;

    ssh_process.kill().expect("Failed to kill process");
    Ok(())
}
