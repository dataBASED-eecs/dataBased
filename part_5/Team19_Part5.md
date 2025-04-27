# EECS 447 Final Project Part 5

## Introduction

### Project Goal

Create a single-library Library Management System (LMS) that lets patrons borrow materials, pay late fees, reserve rooms, and sign up for events, while giving staff tools to manage inventory, returns, fines, and event scheduling.

### Included Features

- Member functions: check-out / return, fee tracking, room and material reservations, event registration, material donations.  
- Staff functions: inventory control, fine administration, returns processing, event creation and oversight.

## Excluded Features
- Payroll, fundraising, or other financial modules  
- Analytics, recommendation engines, or inter-library networking

## Key Terms (abridged)
- **LMS** – Database-backed system for books, members, staff, and events.  
- **Member** – Patron who borrows items and joins events.  
- **Staff / Librarian** – Maintains collection, check-outs, returns, and fees.  
- **Material / Copy** – Physical item (book, movie) that can be loaned; multiple copies possible.  
- **Reservation** – Holds a material or room for specific dates.  
- **Community Event** – Library-hosted program open to member registration.  
- **Donation / Loan** – Items given permanently / temporarily by members.  
- **ISBN / ISAN** – Unique identifiers for books / audiovisual works.  
- **Primary Key / Foreign Key** – Unique row identifier / reference to another table’s primary key.  
- **Domain & Constraint** – Allowed data type and value rules for an attribute.

## Choice of Platform

We choose to use MariaDB, an optimized fork of MySQL. MariaDB was simple to set up on the cycle servers which makes it a good fit for our project.

We implemented our mock data creation and queries in Rust. We choose Rust because several team members were familiar with it. It is both a safe and high performance language which makes it a good choice for high traffic servers like a library management system.

## DDL
Note: The DDL is not in order because we just provided the language to create the relations. For the actually correct order, please see the [Source Code](https://github.com/dataBASED-eecs/dataBased/blob/main/part_5/source_code/src/main.rs)
### Entities
#### Author
```
CREATE TABLE IF NOT EXISTS author 
  (
  ID INT NOT NULL AUTO_INCREMENT,
  First_Name TEXT NOT NULL,
  Last_Name TEXT NOT NULL,
  PRIMARY KEY (ID)
  );
```
#### Book
```
CREATE TABLE IF NOT EXISTS book 
  (
  ISBN VARCHAR(20) NOT NULL,
  Title TEXT NOT NULL,
  PRIMARY KEY (ISBN)
  );
```
#### Book Copy
```
CREATE TABLE IF NOT EXISTS book_copy 
  (
  ID INT NOT NULL,
  PRIMARY KEY (ID),
  FOREIGN KEY (ID) REFERENCES material(ID) ON DELETE CASCADE
  )
```
#### Book Series
```
CREATE TABLE IF NOT EXISTS book_series 
  (
  ID INT NOT NULL AUTO_INCREMENT,
  NAME TEXT NOT NULL,
  PRIMARY KEY (ID)
  );
```

#### Community Event
```
CREATE TABLE IF NOT EXISTS community_event 
  (
  ID INT NOT NULL AUTO_INCREMENT,
  Start_Time DATETIME NOT NULL,
  End_Time DATETIME NOT NULL,
  Longitude FLOAT NOT NULL,
  Latitude FLOAT NOT NULL,
  PRIMARY KEY (ID)
  );
```

#### Director
```
CREATE TABLE IF NOT EXISTS director 
  (
  ID INT NOT NULL AUTO_INCREMENT,
  First_Name TEXT NOT NULL,
  Last_Name TEXT NOT NULL,
  PRIMARY KEY (ID)
  );
```

#### Material
```
CREATE TABLE IF NOT EXISTS material 
  (
  ID INT NOT NULL AUTO_INCREMENT,
  PRIMARY KEY (ID)
  );
```

#### Member
```
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
```

#### Movie
```
CREATE TABLE IF NOT EXISTS movie 
  (
  ISAN VARCHAR(24) NOT NULL,
  Title TEXT NOT NULL,
  PRIMARY KEY (ISAN)
  );
```

#### Movie Copy
```
CREATE TABLE IF NOT EXISTS movie_copy 
  (
  ID INT NOT NULL,
  PRIMARY KEY (ID),
  FOREIGN KEY (ID) REFERENCES material(ID) ON DELETE CASCADE
  );
```

#### Publisher
```
CREATE TABLE IF NOT EXISTS publisher 
  (
  ID INT NOT NULL AUTO_INCREMENT,
  Name TEXT NOT NULL,
  PRIMARY KEY (ID)
  );
```

#### Room
```
CREATE TABLE IF NOT EXISTS room 
  (
  Number INT NOT NULL AUTO_INCREMENT,
  Capacity INT NOT NULL,
  PRIMARY KEY (Number)
  );
```

#### Staff
```
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
```

#### Studio
```
CREATE TABLE IF NOT EXISTS studio 
  (
  ID INT NOT NULL AUTO_INCREMENT,
  Name TEXT NOT NULL,
  PRIMARY KEY (ID)
  );
```

### Relationships
#### Book Has
```
CREATE TABLE IF NOT EXISTS book_has (
  Copy_ID INT NOT NULL,
  Book_ID VARCHAR(20) NOT NULL,
  PRIMARY KEY (Copy_ID),
  FOREIGN KEY (Copy_ID) REFERENCES book_copy(ID) ON DELETE CASCADE,
  FOREIGN KEY (Book_ID) REFERENCES book(ISBN) ON DELETE CASCADE
);
```

#### Directs
```
CREATE TABLE IF NOT EXISTS directs (
    Director_ID INT NOT NULL,
    Movie_ID VARCHAR(24) NOT NULL,
    PRIMARY KEY (Director_ID, Movie_ID),
    FOREIGN KEY (Director_ID) REFERENCES director(ID) ON DELETE CASCADE,
    FOREIGN KEY (Movie_ID) REFERENCES movie(ISAN) ON DELETE CASCADE
);
```
#### Donates
```
CREATE TABLE IF NOT EXISTS donates (
    Member_ID INT NOT NULL,
    Material_ID INT NOT NULL,
    PRIMARY KEY (Member_ID, Material_ID),
    FOREIGN KEY (Member_ID) REFERENCES member(Member_ID) ON DELETE CASCADE,
    FOREIGN KEY (Material_ID) REFERENCES material(ID) ON DELETE CASCADE
);
```

#### Is Part Of
```
CREATE TABLE IF NOT EXISTS is_part_of (
    Book_ID VARCHAR(20) NOT NULL,
    Book_Series_ID INT NOT NULL,
    Seq_Order INT NOT NULL,
    PRIMARY KEY (Book_ID),
    FOREIGN KEY (Book_ID) REFERENCES book(ISBN) ON DELETE CASCADE,
    FOREIGN KEY (Book_Series_ID) REFERENCES book_series(ID) ON DELETE CASCADE
);
```

#### Loans
```
CREATE TABLE IF NOT EXISTS loans (
    Member_ID INT NOT NULL,
    Material_ID INT NOT NULL,
    Duration INT,
    Start_Date DATE NOT NULL,
    PRIMARY KEY (Member_ID, Material_ID),
    FOREIGN KEY (Member_ID) REFERENCES member(Member_ID) ON DELETE CASCADE,
    FOREIGN KEY (Material_ID) REFERENCES material(ID) ON DELETE CASCADE
);
```

#### Movie Has
```
CREATE TABLE IF NOT EXISTS movie_has (
    Copy_ID INT NOT NULL,
    Movie_ID VARCHAR(24) NOT NULL,
    PRIMARY KEY (Copy_ID),
    FOREIGN KEY (Copy_ID) REFERENCES movie_copy(ID) ON DELETE CASCADE,
    FOREIGN KEY (Movie_ID) REFERENCES movie(ISAN) ON DELETE CASCADE
);
```

#### Organizes
```
CREATE TABLE IF NOT EXISTS organizes (
    Staff_ID INT NOT NULL,
    Community_Event_ID INT NOT NULL,
    PRIMARY KEY (Staff_ID, Community_Event_ID),
    FOREIGN KEY (Staff_ID) REFERENCES staff(Member_ID) ON DELETE CASCADE,
    FOREIGN KEY (Community_Event_ID) REFERENCES community_event(ID) ON DELETE CASCADE
);
```

#### Publishes
```
CREATE TABLE IF NOT EXISTS publishes (
    Publisher_ID INT NOT NULL,
    Book_ID VARCHAR(20) NOT NULL,
    Publish_Date DATE NOT NULL,
    PRIMARY KEY (Book_ID),
    FOREIGN KEY (Publisher_ID) REFERENCES publisher(ID) ON DELETE CASCADE,
    FOREIGN KEY (Book_ID) REFERENCES book(ISBN) ON DELETE CASCADE
);
```

#### Registers
```
CREATE TABLE IF NOT EXISTS registers (
    Member_ID INT NOT NULL,
    Community_Event_ID INT NOT NULL,
    PRIMARY KEY (Member_ID, Community_Event_ID),
    FOREIGN KEY (Member_ID) REFERENCES member(Member_ID) ON DELETE CASCADE,
    FOREIGN KEY (Community_Event_ID) REFERENCES community_event(ID) ON DELETE CASCADE
);
```

#### Releases
```
CREATE TABLE IF NOT EXISTS releases (
    Studio_ID INT NOT NULL,
    Movie_ID VARCHAR(24) NOT NULL,
    Release_Date DATE NOT NULL,
    PRIMARY KEY (Movie_ID),
    FOREIGN KEY (Studio_ID) REFERENCES studio(ID) ON DELETE CASCADE,
    FOREIGN KEY (Movie_ID) REFERENCES movie(ISAN) ON DELETE CASCADE
);
```

#### Reserves Material
```
CREATE TABLE IF NOT EXISTS reserves_material (
    Member_ID INT NOT NULL,
    Material_ID INT NOT NULL,
    Reservation_Date DATETIME NOT NULL,
    PRIMARY KEY (Member_ID, Material_ID),
    FOREIGN KEY (Member_ID) REFERENCES member(Member_ID) ON DELETE CASCADE,
    FOREIGN KEY (Material_ID) REFERENCES material(ID) ON DELETE CASCADE
);
```

#### Reserves Room
```
CREATE TABLE IF NOT EXISTS reserves_room (
    Room_ID INT NOT NULL,
    Member_ID INT NOT NULL,
    Duration INT NOT NULL,
    Date DATETIME NOT NULL,
    PRIMARY KEY (Room_ID, Member_ID),
    FOREIGN KEY (Room_ID) REFERENCES room(Number) ON DELETE CASCADE,
    FOREIGN KEY (Member_ID) REFERENCES member(Member_ID) ON DELETE CASCADE
);
```

#### Searches Book
```
CREATE TABLE IF NOT EXISTS searches_book (
    Member_ID INT NOT NULL,
    Book_ID VARCHAR(20) NOT NULL,
    PRIMARY KEY (Member_ID, Book_ID),
    FOREIGN KEY (Member_ID) REFERENCES member(Member_ID) ON DELETE CASCADE,
    FOREIGN KEY (Book_ID) REFERENCES book(ISBN) ON DELETE CASCADE
);
```

#### Searches Movie
```
CREATE TABLE IF NOT EXISTS searches_movie (
    Member_ID INT NOT NULL,
    Movie_ID VARCHAR(24) NOT NULL,
    PRIMARY KEY (Member_ID, Movie_ID),
    FOREIGN KEY (Member_ID) REFERENCES member(Member_ID) ON DELETE CASCADE,
    FOREIGN KEY (Movie_ID) REFERENCES movie(ISAN) ON DELETE CASCADE
);
```

#### Writes
```
CREATE TABLE IF NOT EXISTS writes (
    Author_ID INT NOT NULL,
    Book_ID VARCHAR(20) NOT NULL,
    PRIMARY KEY (Author_ID, Book_ID),
    FOREIGN KEY (Author_ID) REFERENCES author(ID) ON DELETE CASCADE,
    FOREIGN KEY (Book_ID) REFERENCES book(ISBN) ON DELETE CASCADE
);
```

## Table Contents

The following shows all tables:
```
+---------------------------+
| Tables_in_447s25_m444t282 |
+---------------------------+
| author                    |
| book                      |
| book_copy                 |
| book_has                  |
| book_series               |
| community_event           |
| director                  |
| directs                   |
| donates                   |
| is_part_of                |
| loans                     |
| material                  |
| member                    |
| movie                     |
| movie_copy                |
| movie_has                 |
| organizes                 |
| publisher                 |
| publishes                 |
| registers                 |
| releases                  |
| reserves_material         |
| reserves_room             |
| room                      |
| searches_book             |
| searches_movie            |
| staff                     |
| studio                    |
| writes                    |
+---------------------------+

```
To see the contents of all the tables, please see [database_output.md](https://github.com/dataBASED-eecs/dataBased/blob/main/part_5/database_output.md)<br/>

## Relevant Queries
### All Materials
```
WITH book_copy_w_info AS (
    SELECT book_has.Copy_ID, 'Book' AS Type, book_has.Book_ID, NULL AS Movie_ID, book.Title
    FROM book_has
    INNER JOIN book ON book_has.Book_ID = book.ISBN
),
movie_copy_w_info AS (
    SELECT movie_has.Copy_ID, 'Movie' AS Type, NULL AS Book_ID, movie_has.Movie_ID, movie.Title
    FROM movie_has
    INNER JOIN movie ON movie_has.Movie_ID = movie.ISAN
)
SELECT * FROM book_copy_w_info
UNION
SELECT * FROM movie_copy_w_info
ORDER BY Copy_ID;
```
### All Room Reservations
```
SELECT reserves_room.*, First_Name, Last_Name FROM reserves_room NATURAL JOIN member;
```

### Members who have never reserved a room
```
SELECT Member_ID, First_Name, Last_Name FROM member AS m
WHERE NOT EXISTS (
  SELECT member_id FROM reserves_room WHERE member_id=m.member_id
);
```

### View All Donations
```
SELECT * FROM donates;
```

### View Donations Leaderboard
```
WITH donation_leadboard_gt_0 AS (
    SELECT Member_ID, First_Name, Last_Name, COUNT(Member_ID) AS Num_Of_Donations
    FROM member 
    NATURAL JOIN donates
    GROUP BY Member_ID, First_Name, Last_Name  -- Ensure all selected columns are in GROUP BY
),
donation_leaderboard_eq_0 AS (
    SELECT Member_ID, First_Name, Last_Name, 0 AS Num_Of_Donations 
    FROM member AS m 
    WHERE NOT EXISTS (SELECT 1 FROM donates WHERE donates.Member_ID = m.Member_ID)
)
SELECT * FROM donation_leadboard_gt_0 
UNION 
SELECT * FROM donation_leaderboard_eq_0 
ORDER BY Num_Of_Donations DESC;
```

## View Materials on Hold

```
WITH book_copy_w_info AS (
    SELECT book_has.Copy_ID, 'Book' AS Type, book_has.Book_ID, NULL AS Movie_ID, book.Title
    FROM book_has
    INNER JOIN book ON book_has.Book_ID = book.ISBN
),
movie_copy_w_info AS (
    SELECT movie_has.Copy_ID, 'Movie' AS Type, NULL AS Book_ID, movie_has.Movie_ID, movie.Title
    FROM movie_has
    INNER JOIN movie ON movie_has.Movie_ID = movie.ISAN
),
all_materials AS (
  SELECT * FROM book_copy_w_info
  UNION
  SELECT * FROM movie_copy_w_info
  ORDER BY Copy_ID
),
reserved_materials AS (
  SELECT Title, Reservation_Date
  FROM reserves_material
  INNER JOIN all_materials ON reserves_material.Material_ID = all_materials.Copy_ID
)
SELECT Title, Reservation_Date
FROM reserved_materials;
```

## View All Book Series Available and the Books in Them

```
SELECT book_series.ID, book.Title, book_series.NAME, is_part_of.Seq_Order FROM book_series, book, is_part_of
WHERE book_series.ID = is_part_of.Book_Series_ID
AND book.ISBN = is_part_of.Book_ID;
```

## View All Community Events Based on Relevant Attributes

```
# Select all
SELECT * from COMMUNITY_EVENT;
# Select before a certain time
SELECT * from community_event WHERE Start_Time < '2023-07-06 01:18:00';
# Select within a geographic area
SELECT * from community_event WHERE Longitude > 23.571 AND Longitude < 63.571 AND Latitude > 23.571 AND Latitude < 82.4708;
```

## View All Members Attending Community Events

```
WITH community_event_w_info AS (
    SELECT community_event.ID, registers.Member_ID, Start_Time, End_Time, Longitude, Latitude
    FROM community_event
    INNER JOIN registers ON community_event.ID = registers.Community_Event_ID
)
SELECT community_event_w_info.ID, member.Member_ID, Start_Time, End_Time, Longitude, Latitude, First_Name, Last_Name
FROM community_event_w_info
INNER JOIN member ON community_event_w_info.Member_ID = member.Member_ID;
```

## View How many Materials are Checked Out by Each Member

```
SELECT MEMBER_ID, COUNT(*) FROM loans WHERE DURATION is NULL GROUP BY MEMBER_ID;
```

## Calculate Book Checkout Duration

```
WITH outstanding_loans AS (
    SELECT MEMBER_ID, Count(*) AS num_outstanding_loans
    FROM loans
    WHERE Duration IS NULL
    GROUP BY MEMBER_ID
),
average_outstanding_loans AS (
  SELECT AVG(num_outstanding_loans) AS avg_outstanding_loans
  FROM outstanding_loans
)
SELECT (avg_outstanding_loans / (num_outstanding_loans+1))*60*24
FROM outstanding_loans, average_outstanding_loans;
```

## Calculate Fines for Each Member

```
WITH outstanding_loans AS (
    SELECT MEMBER_ID, Count(*) AS num_outstanding_loans
    FROM loans
    WHERE Duration IS NULL
    GROUP BY MEMBER_ID
),
average_outstanding_loans AS (
  SELECT AVG(num_outstanding_loans) AS avg_outstanding_loans
  FROM outstanding_loans
),
allowed_loan_time AS (
  SELECT (avg_outstanding_loans / num_outstanding_loans)*60*24 AS allowed_loan_time
  FROM outstanding_loans, average_outstanding_loans
),
fine_per_book AS (
  SELECT Member_ID, (loans.Start_Date - CURRENT_DATE()) * 0.00025 AS fine
  FROM loans, allowed_loan_time
  WHERE Duration IS NULL
)
SELECT Member_ID, SUM(fine) AS fine_total
FROM fine_per_book
GROUP BY Member_ID;
```
