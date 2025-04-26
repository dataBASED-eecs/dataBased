# EECS 447 Final Project Part 5

## Introduction

## Choice of Platform

## DDL
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

## Meeting Logs
