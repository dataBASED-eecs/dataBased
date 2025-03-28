# EECS 447 Final Project Part 4

## Introduction

### Project Overview

The purpose of our Library Management System is to provide an effective method for knowledge sharing, community meeting, and community events within the library. The system enables members to check out and return materials while tracking overdue fees. Additionally, members can view and register for community events. Library staff can manage inventory, oversee late fees, and organize events.
### Scope

This project consists of the development of a database system to manage library materials, member checkouts, event registrations, and administrative tasks. It allows members to reserve rooms, track fees, and donate materials. Staff can administer fines, process returns, and manage inventory. The system excludes financial features such as payroll, fundraising, or donations. It also does not incorporate analytics or recommendations. The system is designed for single-library use and without inter-library communication.

### Glossary

- Library Management System (LMS)
    - A database-driven system that enables the management of books, members, staff, and events within a library.
- Member
    - A library user who can check out materials and participate in community events.
- Librarian / Library Staff
    - A staff member who is responsible for maintaining the library's collection, processing checkouts/returns, and managing late fees.
- Organizer
	- A member of library staff who is creating and organizing a community event at a given time.
- Material
	- Any book copy or movie copy that can be reserved by a user. These do not include normal books.
- Copy
	- A copy of an individual book that exists in the library. There can be multiple copies for a single book.
- Donation
	- Any material that has been given to the library by a member, without expectation of return.
- Loan
	- Any material that has been provided to the library by a member for a set amount of time.
- Reservation
	- Allows a library member to book materials for certain dates.
- Community Event
    - A scheduled gathering or program held at the library that members can register for and attend.
- ISBN
	- International Standard Book Number - A unique 10-13 digit identifier for books.
- ISAN
	- International Standard Audiovisual Number - A unique identifier for films, shorts, documentaries, etc.
- Key
	- Always uniquely identifies a single row in a relation.
- Primary Key
	- A designated attribute that can reliably identify and distinguish between each individual record in a table.
- Foreign Key
	- A foreign key is a set of attributes in a table that refers to the primary key of another table, linking these two tables.
- Schema of a Relation
	- Relation schema defines the design and structure of a relation or table in the database.
- Relational Database Schema 
	- Defines names of relations or tables in the database, attributes of each table, the domain of each column, and integrity constraints.
- Domain
	- Used to specify the data type and permissible values for an attribute.
- Domain Constraint
	- A rule that restricts the type of data that can be stored in a particular attribute of a relation.

NOTE: Other definitions for entities and relationships are defined at the intial mention.

## Data Dictionary
### Room
FDs: TDB
| Attribute Name        | Data Type | Foreign key of | primary key | Description |
|-|-|-|-|-|
| Capacity | INT | | | The number of people the room has space for |
| Number | INT | | x | The unique number of the room in the library |

### Reserves_Room
FDs: TDB
| Attribute Name        | Data Type | Foreign key of | primary key | Description |
|-|-|-|-|-|
| Duration | TIME | | | The duration the room is reserved for |
| Date | DATETIME | | | The time the room reservation starts at |
| Room_ID | Number | Room (Number) | x | The number of the room that is reserved. This is a foreign key into the Room table |
| Member_ID | INT | Member (Member_ID) | x | The member id that reserved the room. This is a foreign key into the Member table |

### Staff
FDs: TDB
| Attribute Name        | Data Type | Foreign key of | primary key | Description |
|-|-|-|-|-|
| Salary | DECIMAL(6,2) | | | The yearly salary of the staff member in dolars |
| Work_Phone | VARCHAR(15) |  | |The phone number of the staff member |
| Start_Date | DATE |  | |The date the staff member started working for the library |
| Work_Email | TEXT |  | |The work email of the staff member |
| Date_of_Birth         | DATE | | | The date the member was born |
| Member_ID             | INT | Member (Member_ID) | x | The unique id of the member. This is a foreign key into the Member table |
| Outstanding_Balance   | DECIMAL(6,2) | | |The amount the member owes to the library in dolars |
| Email | TEXT | | | The email of the member |
| First_Name | TEXT |  | | The first name of the member |
| Last_Name | TEXT |  | | The last name of the member |

### Organizes
FDs: TDB
| Attribute Name        | Data Type | Foreign key of | primary key | Description |
|-|-|-|-|-|
| Staff_ID | INT | Staff (Member_ID) | x | The ID of the staff member that organizes the event. This is a foreign key into the Staff table |
| Community_Event_ID | INT | Community_Event (ID) | x | The id of the community event that is organized. This is a foreign key into the Community Event table |

### Registers
FDs: TDB
| Attribute Name        | Data Type | Foreign key of | primary key | Description |
|-|-|-|-|-|
| Member_ID | INT | Member (Member_ID) | x | The member id of a member that is registered for an event. This is a foreign key into the Staff table |
| Community_Event_ID | INT | Community_Event (ID) | x | The id of the community event that is organized. This is a foreign key into the Community Event table |

### Community_Event
FDs: TDB
| Attribute Name        | Data Type | Foreign key of | primary key | Description |
|-|-|-|-|-|
| Latitude | FLOAT | | | The latitude of the location of the event |
| Longitude | FLOAT | | | The longitude of the location of the event |
| Start_Time | DATETIME | | | The time the event starts |
| End_Time | DATETIME | | | The time the event ends |
| Organizer_First_Name | | | TEXT | The first name of the event organizer |
| Organizer_Last_Name | | | TEXT | The last name of the event organizer |
| ID | INT | | x | The unique id of the event |

### Member
FDs: TDB
| Attribute Name        | Data Type | Foreign key of | primary key | Description |
|-|-|-|-|-|
| Date_of_Birth         | DATE | | | The date the member was born |
| Member_ID             | INT | | x | The unique id of the member |
| Outstanding_Balance   | DECIMAL(6,2) | | | The amount the member owes to the library in dolars |
| Email | TEXT | | | The email of the member |
| First_Name | TEXT | | | The first name of the member |
| Last_Name | TEXT | | | The last name of the member |

### Loans
FDs: TDB
| Attribute Name        | Data Type | Foreign key of | primary key | Description |
|-|-|-|-|-|
| Duration | TIME | | | The time the loan is approved for |
| Start_Date | DATETIME | | | The time the loan started |
| Member_ID | INT | Member (Member_ID) | x | The member id of the member that is loaning a material. This is a foreign key into the Member table |
| Material_ID | INT | Material (ID) | x | The id of the material that is loaned. This is a foreign key into the Material table |

### Donates
FDs: TDB
| Attribute Name        | Data Type | Foreign key of | primary key | Description |
|-|-|-|-|-|
| Member_ID | INT | Member (Member_ID) | | The member id of the member that donated the material. This is a foreign key into the Member table |
| Material_ID | INT | Material (ID) | x | The id of the material that was donated. This is a foreign key into the Material table |

### Material
FDs: TDB
| Attribute Name        | Data Type | Foreign key of | primary key | Description |
|-|-|-|-|-|
| ID | INT | | x | The unique id of the material |

### Reserves_Material
FDs: TDB
| Attribute Name        | Data Type | Foreign key of | primary key | Description |
|-|-|-|-|-|
| Reservation_Date | DATETIME | | | The date the reservation is for |
| Member_ID | INT | Member (Member_ID) | x | The member id of the member that reserved the material. This is a foreign key into the Member table |
| Material_ID | INT | Material (ID) | x | The id of the material that was reserved. This is a foreign key into the Material table |

### Author
FDs: TDB
| Attribute Name        | Data Type | Foreign key of | primary key | Description |
|-|-|-|-|-|
| First_Name | TEXT | | | The first name of the author |
| Last_Name | TEXT | | | The second name of the author |
| ID | INT | | x | The unique id of the author |

### Searches_Book
FDs: TDB
| Attribute Name        | Data Type | Foreign key of | primary key | Description |
|-|-|-|-|-|
| Member_ID | INT | Member (Member_ID) | x | The member id of the member that searched for a book. This is a foreign key into the Member table |
| Book_ID | INT | Book (ID) | x | The id of the book that was searched for. This is a foreign key into the Book table |

### Searches_Movie
FDs: TDB
| Attribute Name        | Data Type | Foreign key of | primary key | Description |
|-|-|-|-|-|
| Member_ID | INT | Member (Member_ID) | x | The member id of the member that searched for a movie. This is a foreign key into the Member table |
| Movie_ID | INT | Movie (ID) | x | The id of the movie that was searched for. This is a foreign key into the Movie table |

### Book_Copy
FDs: TDB
| Attribute Name        | Data Type | Foreign key of | primary key | Description |
|-|-|-|-|-|
| ID | INT | Material (ID) | x |  The unique id of the movie copy. This is a foreign key into the material table |

### Book_Has
FDs: TDB
| Attribute Name        | Data Type | Foreign key of | primary key | Description |
|-|-|-|-|-|
| Copy_ID | INT | Book_Copy (ID) | x | The id of the book instance which is a copy of the book. This is a foreign key into the Book Copy table |
| Book_ID | INT | Book (ISBN) | | The id of the book. This is a foreign key into the Book table |

### Writes
FDs: TDB
| Attribute Name        | Data Type | Foreign key of | primary key | Description |
|-|-|-|-|-|
| Author_ID | INT | Author (ID) | x | The id of the author that wrote a book. This is a foreign key into the Author table |
| Book_ID | INT | Book (ISBN) | x | The id of the book that the author wrote. This is a foreign key into the Book table |

### Publishes
FDs: TDB
| Attribute Name        | Data Type | Foreign key of | primary key | Description |
|-|-|-|-|-|
| Publisher_ID | INT | Publisher (ID) | | The id of the publisher that published a book. This is a foreign key into the Publisher table |
| Book_ID | INT | Book (ISBN) | x |  The id of the book that the publisher published. This is a foreign key into the Book table |
| Publish_Date | DATE| | | The date the book was published |

### Is_Part_Of
FDs: TDB
| Attribute Name        | Data Type | Foreign key of | primary key | Description |
|-|-|-|-|-|
| Order | INT | The position of the item in the book series |
| Book_Series_ID | INT | Book_Series (ID) |  | The id of the book series a book is part of. This is a foreign key into the Book Series table |
| Book_ID | INT | Book (ISBN) | x | The id of the book in the series. This is a foreign key into the Book table |

### Directs
FDs: TDB
| Attribute Name        | Data Type | Foreign key of | primary key | Description |
|-|-|-|-|-|
| Director_ID | INT | Director (ID) | x | The id of the director that directed the movie. This is a foreign key into the Director table |
| Movie_ID | INT | Movie (ISAN) | x | The id of the movie that director directed. This is a foreign key into the Movie table |

### Releases
FDs: TDB
| Attribute Name        | Data Type | Foreign key of | primary key | Description |
|-|-|-|-|-|
| Studio_ID | INT | Studio (ID) |  | The id of the studio that released the movie. This is a foreign key into the Studio table |
| Movie_ID | INT | Movie (ISAN) | x |The id of the movie that studio released. This is a foreign key into the Movie table |
| Release_Date | DATE | | | The date the movie was released |

### Book
FDs: TDB
| Attribute Name        | Data Type | Foreign key of | primary key | Description |
|-|-|-|-|-|
| ISBN | CHAR(13) | | x | The unique ISBN id of the book |
| Title | TEXT | | | The title of the book |

### Publisher
FDs: TDB
| Attribute Name        | Data Type | Foreign key of | primary key | Description |
|-|-|-|-|-|
| Name | TEXT | | | The name of the publishing organization |
| ID | INT | | x | The unique id of the publisher |

### Book_Series
FDs: TDB
| Attribute Name        | Data Type | Foreign key of | primary key | Description |
|-|-|-|-|-|
| Name | TEXT | | | The name of the book series |
| ID | INT | | x | The unique id of the book series |

### Director
FDs: TDB
| Attribute Name        | Data Type | Foreign key of | primary key | Description |
|-|-|-|-|-|
| First_Name | TEXT | | | The first name of the director |
| Last_Name | TEXT | | | The last name of the director |
| ID | INT | | x | The unique id of the director |

### Movie_Copy
FDs: TDB
| Attribute Name        | Data Type | Foreign key of | primary key | Description |
|-|-|-|-|-|
| ID | INT | Material (ID) | x | The unique id of the movie copy. This is a foreign key into the material table |

### Movie_Has
FDs: TDB
| Attribute Name        | Data Type | Foreign key of | primary key | Description |
|-|-|-|-|-|
| Copy_ID | INT | Movie_Copy (ID) | x | The id of the movie instance which is a copy of the movie. This is a foreign key into the Movie Copy table |
| Movie_ID | INT | Movie (ISAN) | | The id of the movie. This is a foreign key into the Movie table |

### Movie
FDs: TDB
| Attribute Name        | Data Type | Foreign key of | primary key | Description |
|-|-|-|-|-|
| ISAN | TEXT | | x | The unique ISAN id of the movie |
| Title | TEXT | | | The title of the movie |

### Studio
FDs: TDB
| Attribute Name        | Data Type | Foreign key of | primary key | Description |
|-|-|-|-|-|
| Name | TEXT | | | The name of the studio |
| ID | INT | | x | The unique id of the studio |

## Schema Diagram
You can find the Schema Image [Here](schema_relationship_diagram.png) and the Source Code [Here](databaseschemasource.md)