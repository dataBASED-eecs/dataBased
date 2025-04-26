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
