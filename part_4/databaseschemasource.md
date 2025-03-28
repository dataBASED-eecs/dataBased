Here is the source code for the database schema

```
digraph G {
    ratio=1.5;
    rankdir=LR;
    /***************************************************************************************
    Entity Schema Definition
    ***************************************************************************************/
    node [shape=plaintext];
    
    // Define the Member Schema
    member [label=<
        <TABLE BORDER="0" CELLBORDER="1" CELLSPACING="0">
            <TR>
                <TD COLSPAN="1" ALIGN="LEFT" BORDER="0"><B>Member</B></TD>
            </TR>
            <TR>
                <TD PORT="member_member_id" BGCOLOR="lightblue"><U>Member_ID</U></TD>
                <TD PORT="member_dob" BGCOLOR="lightblue">Date_of_Birth</TD>
                <TD PORT="member_fname" BGCOLOR="lightblue">First_Name</TD>
                <TD PORT="member_lname" BGCOLOR="lightblue">Last_Name</TD>
                <TD PORT="member_email" BGCOLOR="lightblue">Email</TD>
                <TD PORT="member_balance" BGCOLOR="lightblue">Outstanding_Balance</TD>
            </TR>
        </TABLE>>];

    // Define the Staff Schema
    staff [label=<
        <TABLE BORDER="0" CELLBORDER="1" CELLSPACING="0">
            <TR>
                <TD COLSPAN="1" ALIGN="LEFT" BORDER="0"><B>Staff</B></TD>
            </TR>
            <TR>
                <TD PORT="member_member_id" BGCOLOR="lightblue"><U>Member_ID</U></TD>
                <TD PORT="member_dob" BGCOLOR="lightblue">Date_of_Birth</TD>
                <TD PORT="member_fname" BGCOLOR="lightblue">First_Name</TD>
                <TD PORT="member_lname" BGCOLOR="lightblue">Last_Name</TD>
                <TD PORT="member_email" BGCOLOR="lightblue">Email</TD>
                <TD PORT="member_balance" BGCOLOR="lightblue">Outstanding_Balance</TD>
                <TD PORT="staff_work_phone_number" BGCOLOR="lightblue">Work_Phone</TD>
                <TD PORT="staff_salary" BGCOLOR="lightblue">Salary</TD>
                <TD PORT="staff_work_email" BGCOLOR="lightblue">Work_Email</TD>
                <TD PORT="staff_start_date" BGCOLOR="lightblue">Start_Date</TD>
            </TR>
        </TABLE>>];

    // Define the Material Schema
    material [label=<
        <TABLE BORDER="0" CELLBORDER="1" CELLSPACING="0">
            <TR>
                <TD COLSPAN="1" ALIGN="LEFT" BORDER="0"><B>Material</B></TD>
            </TR>
            <TR>
                <TD PORT="material_id" BGCOLOR="lightblue"><U>ID</U></TD>
            </TR>
        </TABLE>>];
        
    // Define the Book Schema
    book [label=<
        <TABLE BORDER="0" CELLBORDER="1" CELLSPACING="0">
            <TR>
                <TD COLSPAN="1" ALIGN="LEFT" BORDER="0"><B>Book</B></TD>
            </TR>
            <TR>
                <TD PORT="book_isbn" BGCOLOR="lightblue"><U>ISBN</U></TD>
                <TD PORT="book_title" BGCOLOR="lightblue">Title</TD>
            </TR>
        </TABLE>>];
        
    // Define the Book Copy
    book_copy [label=<
        <TABLE BORDER="0" CELLBORDER="1" CELLSPACING="0">
            <TR>
                <TD COLSPAN="1" ALIGN="LEFT" BORDER="0"><B>Book_Copy</B></TD>
            </TR>
            <TR>
                <TD PORT="material_id" BGCOLOR="lightblue"><U>ID</U></TD>
            </TR>
        </TABLE>>];
        
    // Define the Author Schema
    author [label=<
        <TABLE BORDER="0" CELLBORDER="1" CELLSPACING="0">
            <TR>
                <TD COLSPAN="1" ALIGN="LEFT" BORDER="0"><B>Author</B></TD>
            </TR>
            <TR>
                <TD PORT="author_id" BGCOLOR="lightblue"><U>ID</U></TD>
                <TD PORT="author_fname" BGCOLOR="lightblue">First_Name</TD>
                <TD PORT="author_lname" BGCOLOR="lightblue">Last_Name</TD>
            </TR>
        </TABLE>>];
        
    // Define the Publisher Schema
    publisher [label=<
        <TABLE BORDER="0" CELLBORDER="1" CELLSPACING="0">
            <TR>
                <TD COLSPAN="1" ALIGN="LEFT" BORDER="0"><B>Publisher</B></TD>
            </TR>
            <TR>
                <TD PORT="publisher_id" BGCOLOR="lightblue"><U>ID</U></TD>
                <TD PORT="publisher_name" BGCOLOR="lightblue">Name</TD>
            </TR>
        </TABLE>>];
        
    // Define the Book Series Schema
    book_series [label=<
        <TABLE BORDER="0" CELLBORDER="1" CELLSPACING="0">
            <TR>
                <TD COLSPAN="1" ALIGN="LEFT" BORDER="0"><B>Book_Series</B></TD>
            </TR>
            <TR>
                <TD PORT="book_series_id" BGCOLOR="lightblue"><U>ID</U></TD>
                <TD PORT="book_series_name" BGCOLOR="lightblue">Name</TD>
            </TR>
        </TABLE>>];
        
    // Define the Movie Schema
    movie [label=<
        <TABLE BORDER="0" CELLBORDER="1" CELLSPACING="0">
            <TR>
                <TD COLSPAN="1" ALIGN="LEFT" BORDER="0"><B>Movie</B></TD>
            </TR>
            <TR>
                <TD PORT="movie_isan" BGCOLOR="lightblue"><U>ISAN</U></TD>
                <TD PORT="movie_title" BGCOLOR="lightblue">Title</TD>
            </TR>
        </TABLE>>];
        
    // Define the Movie Copy Schema
    movie_copy [label=<
        <TABLE BORDER="0" CELLBORDER="1" CELLSPACING="0">
            <TR>
                <TD COLSPAN="1" ALIGN="LEFT" BORDER="0"><B>Movie_Copy</B></TD>
            </TR>
            <TR>
                <TD PORT="movie_id" BGCOLOR="lightblue"><U>ID</U></TD>
            </TR>
        </TABLE>>];
        
    // Define the Director Schema
    director [label=<
        <TABLE BORDER="0" CELLBORDER="1" CELLSPACING="0">
            <TR>
                <TD COLSPAN="1" ALIGN="LEFT" BORDER="0"><B>Director</B></TD>
            </TR>
            <TR>
                <TD PORT="director_id" BGCOLOR="lightblue"><U>ID</U></TD>
                <TD PORT="director_fname" BGCOLOR="lightblue">First_Name</TD>
                <TD PORT="director_lname" BGCOLOR="lightblue">Last_Name</TD>
            </TR>
        </TABLE>>];
        
    // Define the Studio Schema
    studio [label=<
        <TABLE BORDER="0" CELLBORDER="1" CELLSPACING="0">
            <TR>
                <TD COLSPAN="1" ALIGN="LEFT" BORDER="0"><B>Studio</B></TD>
            </TR>
            <TR>
                <TD PORT="studio_id" BGCOLOR="lightblue"><U>ID</U></TD>
                <TD PORT="studio_name" BGCOLOR="lightblue">Name</TD>
            </TR>
        </TABLE>>];
       
    // Define the Room Schema 
    room [label=<
        <TABLE BORDER="0" CELLBORDER="1" CELLSPACING="0">
            <TR>
                <TD COLSPAN="1" ALIGN="LEFT" BORDER="0"><B>Room</B></TD>
            </TR>
            <TR>
                <TD PORT="room_number" BGCOLOR="lightblue"><U>Number</U></TD>
                <TD PORT="room_capacity" BGCOLOR="lightblue">Capacity</TD>
            </TR>
        </TABLE>>];
       
    // Define the Community Event Schema 
    community_event [label=<
        <TABLE BORDER="0" CELLBORDER="1" CELLSPACING="0">
            <TR>
                <TD COLSPAN="1" ALIGN="LEFT" BORDER="0"><B>Community_Event</B></TD>
            </TR>
            <TR>
                <TD PORT="community_event_id" BGCOLOR="lightblue"><U>ID</U></TD>
                <TD PORT="community_event_organizer_fname" BGCOLOR="lightblue">Organizer_First_Name</TD>
                <TD PORT="community_event_organizer_lname" BGCOLOR="lightblue">Organizer_Last_Name</TD>
                <TD PORT="community_event_start_time" BGCOLOR="lightblue">Start_Time</TD>
                <TD PORT="community_event_end_time" BGCOLOR="lightblue">End_Time</TD>
                <TD PORT="community_event_longitudinal_location" BGCOLOR="lightblue">Longitude</TD>
                <TD PORT="community_event_latitudinal_location" BGCOLOR="lightblue">Latitude</TD>
            </TR>
        </TABLE>>];
        
    /***************************************************************************************
    Relationship Schema Definition
    ***************************************************************************************/
    // M Members can Register for N Community Events
    member_registers_community_event [label=<
        <TABLE BORDER="0" CELLBORDER="1" CELLSPACING="0">
            <TR>
                <TD COLSPAN="1" ALIGN="LEFT" BORDER="0"><B>Registers</B></TD>
            </TR>
            <TR>
                <TD PORT="member_id" BGCOLOR="#ffcccc"><U>Member_ID</U></TD>
                <TD PORT="community_event_id" BGCOLOR="#ffcccc"><U>Community_Event_ID</U></TD>
            </TR>
        </TABLE>>];
        
    // M Staff can Organize for N Community Events
    staff_organizes_community_event [label=<
        <TABLE BORDER="0" CELLBORDER="1" CELLSPACING="0">
            <TR>
                <TD COLSPAN="1" ALIGN="LEFT" BORDER="0"><B>Organizes</B></TD>
            </TR>
            <TR>
                <TD PORT="member_id" BGCOLOR="#ffcccc"><U>Staff_ID</U></TD>
                <TD PORT="community_event_id" BGCOLOR="#ffcccc"><U>Community_Event_ID</U></TD>
            </TR>
        </TABLE>>];
        
    // M Members can Loan N Books
    member_loans_material [label=<
        <TABLE BORDER="0" CELLBORDER="1" CELLSPACING="0">
            <TR>
                <TD COLSPAN="1" ALIGN="LEFT" BORDER="0"><B>Loans</B></TD>
            </TR>
            <TR>
                <TD PORT="member_id" BGCOLOR="#ffcccc"><U>Member_ID</U></TD>
                <TD PORT="material_id" BGCOLOR="#ffcccc"><U>Material_ID</U></TD>
                <TD PORT="start_date" BGCOLOR="#ffcccc">Start_Date</TD>
                <TD PORT="duration" BGCOLOR="#ffcccc">Duration</TD>
            </TR>
        </TABLE>>];
        
    // M Members can Reserve N Materials
    member_reserves_material [label=<
        <TABLE BORDER="0" CELLBORDER="1" CELLSPACING="0">
            <TR>
                <TD COLSPAN="1" ALIGN="LEFT" BORDER="0"><B>Reserves_Material</B></TD>
            </TR>
            <TR>
                <TD PORT="member_id" BGCOLOR="#ffcccc"><U>Member_ID</U></TD>
                <TD PORT="material_id" BGCOLOR="#ffcccc"><U>Material_ID</U></TD>
                <TD PORT="reservation_date" BGCOLOR="#ffcccc">Reservation_Date</TD>
            </TR>
        </TABLE>>];
        
    // 1 book can be donated by 1 member, a member can donate any number of books
    member_donates_material [label=<
        <TABLE BORDER="0" CELLBORDER="1" CELLSPACING="0">
            <TR>
                <TD COLSPAN="1" ALIGN="LEFT" BORDER="0"><B>Donates</B></TD>
            </TR>
            <TR>
                <TD PORT="member_id" BGCOLOR="#ffcccc">Member_ID</TD>
                <TD PORT="material_id" BGCOLOR="#ffcccc"><U>Material_ID</U></TD>
            </TR>
        </TABLE>>];
    
    // M Members can Search N Books
    member_searches_book [label=<
        <TABLE BORDER="0" CELLBORDER="1" CELLSPACING="0">
            <TR>
                <TD COLSPAN="1" ALIGN="LEFT" BORDER="0"><B>Searches_Book</B></TD>
            </TR>
            <TR>
                <TD PORT="member_id" BGCOLOR="#ffcccc"><U>Member_ID</U></TD>
                <TD PORT="book_id" BGCOLOR="#ffcccc"><U>Book_ID</U></TD>
            </TR>
        </TABLE>>];
    
    // M Members can Search N Books   
    member_searches_movie [label=<
        <TABLE BORDER="0" CELLBORDER="1" CELLSPACING="0">
            <TR>
                <TD COLSPAN="1" ALIGN="LEFT" BORDER="0"><B>Searches_Movie</B></TD>
            </TR>
            <TR>
                <TD PORT="member_id" BGCOLOR="#ffcccc"><U>Member_ID</U></TD>
                <TD PORT="movie_id" BGCOLOR="#ffcccc"><U>Movie_ID</U></TD>
            </TR>
        </TABLE>>];
    
    // A room can be reserved by one member, a member can reserve N rooms
    member_reserves_room [label=<
        <TABLE BORDER="0" CELLBORDER="1" CELLSPACING="0">
            <TR>
                <TD COLSPAN="1" ALIGN="LEFT" BORDER="0"><B>Reserves_Room</B></TD>
            </TR>
            <TR>
                <TD PORT="member_id" BGCOLOR="#ffcccc">Member_ID</TD>
                <TD PORT="room_id" BGCOLOR="#ffcccc"><U>Room_ID</U></TD>
                <TD PORT="date" BGCOLOR="#ffcccc">Date</TD>
                <TD PORT="duration" BGCOLOR="#ffcccc">Duration</TD>
            </TR>
        </TABLE>>];
        
    // M authors can write N books
    author_writes_book [label=<
        <TABLE BORDER="0" CELLBORDER="1" CELLSPACING="0">
            <TR>
                <TD COLSPAN="1" ALIGN="LEFT" BORDER="0"><B>Writes</B></TD>
            </TR>
            <TR>
                <TD PORT="author_id" BGCOLOR="#ffcccc"><U>Author_ID</U></TD>
                <TD PORT="book_id" BGCOLOR="#ffcccc"><U>Book_ID</U></TD>
            </TR>
        </TABLE>>];
        
    // A Book is pubished by 1 publisher, a publisher can publish N books
    publisher_publishes_book [label=<
        <TABLE BORDER="0" CELLBORDER="1" CELLSPACING="0">
            <TR>
                <TD COLSPAN="1" ALIGN="LEFT" BORDER="0"><B>Publishes</B></TD>
            </TR>
            <TR>
                <TD PORT="publisher_id" BGCOLOR="#ffcccc">Publisher_ID</TD>
                <TD PORT="book_id" BGCOLOR="#ffcccc"><U>Book_ID</U></TD>
                <TD PORT="publish_date" BGCOLOR="#ffcccc">Publish_Date</TD>
            </TR>
        </TABLE>>];
        
    // A Series can contain multiple books, a Book is part of one series
    book_is_part_of_book_series [label=<
        <TABLE BORDER="0" CELLBORDER="1" CELLSPACING="0">
            <TR>
                <TD COLSPAN="1" ALIGN="LEFT" BORDER="0"><B>Is_Part_Of</B></TD>
            </TR>
            <TR>
                <TD PORT="book_series_id" BGCOLOR="#ffcccc">Book_Series_ID</TD>
                <TD PORT="book_id" BGCOLOR="#ffcccc"><U>Book_ID</U></TD>
                <TD PORT="book_series_order" BGCOLOR="#ffcccc">Order</TD>
            </TR>
        </TABLE>>];
        
     // M directors can direct N movies
    director_directs_movie [label=<
        <TABLE BORDER="0" CELLBORDER="1" CELLSPACING="0">
            <TR>
                <TD COLSPAN="1" ALIGN="LEFT" BORDER="0"><B>Directs</B></TD>
            </TR>
            <TR>
                <TD PORT="author_id" BGCOLOR="#ffcccc"><U>Director_ID</U></TD>
                <TD PORT="book_id" BGCOLOR="#ffcccc"><U>Movie_ID</U></TD>
            </TR>
        </TABLE>>];
        
    // A movie is released by 1 studio, a studio can release N movies
    studio_releases_movie [label=<
        <TABLE BORDER="0" CELLBORDER="1" CELLSPACING="0">
            <TR>
                <TD COLSPAN="1" ALIGN="LEFT" BORDER="0"><B>Releases</B></TD>
            </TR>
            <TR>
                <TD PORT="studio_id" BGCOLOR="#ffcccc">Studio_ID</TD>
                <TD PORT="movie_id" BGCOLOR="#ffcccc"><U>Movie_ID</U></TD>
                <TD PORT="release_date" BGCOLOR="#ffcccc">Release_Date</TD>
            </TR>
        </TABLE>>];
        
    // One Book can have multiple copies
    book_has_book_copy [label=<
        <TABLE BORDER="0" CELLBORDER="1" CELLSPACING="0">
            <TR>
                <TD COLSPAN="1" ALIGN="LEFT" BORDER="0"><B>Book_Has</B></TD>
            </TR>
            <TR>
                <TD PORT="book_id" BGCOLOR="#ffcccc"><U>Book_ID</U></TD>
                <TD PORT="book_copy_id" BGCOLOR="#ffcccc">Copy_ID</TD>
            </TR>
        </TABLE>>];
        
    // One Movie can have multiple copies
    movie_has_movie_copy [label=<
        <TABLE BORDER="0" CELLBORDER="1" CELLSPACING="0">
            <TR>
                <TD COLSPAN="1" ALIGN="LEFT" BORDER="0"><B>Movie_Has</B></TD>
            </TR>
            <TR>
                <TD PORT="movie_id" BGCOLOR="#ffcccc"><U>Movie_ID</U></TD>
                <TD PORT="movie_copy_id" BGCOLOR="#ffcccc">Copy_ID</TD>
            </TR>
        </TABLE>>];
    
    
    /***************************************************************************************
    Foreign Key Links
    ***************************************************************************************/
    // Staffs Inherit from member
    staff:member_member_id -> member:member_member_id
    
    // A book copy and movie copies inherit from material
    book_copy:material_id -> material:material_id
    movie_copy:material_id -> material:material_id
    
    // Link the foreign keys in staff_organizes_community_event
    member_registers_community_event:member_id -> member:member_member_id
    member_registers_community_event:community_event_id -> community_event:community_event_id
    
    // Link the foreign keys in staff_organizes_community_event
    staff_organizes_community_event:member_id -> staff:member_member_id
    staff_organizes_community_event:community_event_id -> community_event:community_event_id

    // Link the foreign keys in member_loans_material
    member_loans_material:member_id -> member:member_member_id
    member_loans_material:material_id -> material:material_id
    
    // Link the foreign keys in member_reserves_material
    member_reserves_material:member_id -> member:member_member_id
    member_reserves_material:material_id -> material:material_id
    
    // Link the foreign keys in member_donates_material
    member_donates_material:member_id -> member:member_member_id
    member_donates_material:material_id -> material:material_id
    
    // Link the foreign keys in member_searches_book
    member_searches_book:member_id -> member:member_member_id
    member_searches_book:book_id -> book:book_isbn
    
    // Link the foreign keys in member_searches_movie
    member_searches_movie:member_id -> member:member_member_id
    member_searches_movie:movie_id -> movie:movie_isan
    
    // Link the foreign keys in member_reserves_room
    member_reserves_room:member_id -> member:member_member_id
    member_reserves_room:room_id -> room:room_number
    
    // Link the foreign keys in author_writes_book
    author_writes_book:author_id -> author:author_id
    author_writes_book:book_id -> book:book_isbn
    
    // Link the foreign keys in author_writes_book
    publisher_publishes_book:publisher_id -> publisher:publisher_id
    publisher_publishes_book:book_id -> book:book_isbn
    
    // Link the foreign keys in book_is_part_of_book_series
    book_is_part_of_book_series:book_series_id -> book_series:book_series_id
    book_is_part_of_book_series:book_id -> book:book_isbn
    
    // Link the foreign keys in author_writes_book
    director_directs_movie:director_id -> director:director_id
    director_directs_movie:movie_id -> movie:movie_isan
    
    // Link the foreign keys in author_writes_book
    studio_releases_movie:studio_id -> studio:studio_id
    studio_releases_movie:movie_id -> movie:movie_isan
    
    // Link the foreign keys in book_has_book_copy
    book_has_book_copy:book_id -> book:book_isbn
    book_has_book_copy:book_copy_id -> book_copy:material_id
    
    // Link the foreign keys in movie_has_movie_copy
    movie_has_movie_copy:movie_id -> movie:movie_isan
    movie_has_movie_copy:movie_copy_id -> movie_copy:material_id

}
```
