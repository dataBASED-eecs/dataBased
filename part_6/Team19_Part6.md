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
