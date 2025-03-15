# EECS 447 Final Project Part 3
## Introduction
### Project Overview
The purpose of our Library Management System is to provide an effective method for knowledge sharing, community meeting, and community events within the library. The system enables members to check out and return materials while tracking overdue fees. Additionaly, members can view and regsiter for community events. Library staff can manage inventory, oversee late fees, and organize events. 
### Scope
This project consists of the development of a database system to manage library materials, member checkouts, event registrations, and administrative tasks. It allows members to reserve rooms, track fees, and donate materials. Staff can administer fines, process returns, and manage inventory. The system excludes financial features such as payroll, fundraising, or donations. It also does not incorporate analytics or recommendations. The system is designed for single-library use and without inter-library communication.
### Glossary
- Library Management System (LMS) 
	- A database-driven system that enables the management of books, members, staff, and events within a library.
- Member 
	- A library user who can check out materials and participate in community events.
- Librarian / Library Staff 
	- A staff member who is responsible for maintaining the library's collection, processing checkouts/returns, and managing late fees.
- Community Event 
	- A scheduled gathering or program held at the library that members can register for and attend.

NOTE: Other definitions for entities and relationships are defined at the intial mention.

## Entity/Relationship Information
### Standalone Entities
1. Member: All people who use the library, including people who work there
	- Member ID (Primary Key)
		- Numeric Type where ID >= 0 and ID < ∞
	- Date of Birth
		- Numeric In Epoch Time
	- Outstanding Balance
		- Floating Point Number (Two Decimal Places) where Balance <= 0
	- Email
		- String Compliant with RFC522
	- Last Name
		- Variable Character String with Max Size of 100
	- First Name
		- Variable Character String with Max Size of 100
2. Library Room: A place where an individual can study and use to their liking
	- Number (Primary Key)
		- Numeric Type where Number >= 0
	- Room Capacity
		 - Numeric Type where Capacity >= 1
3. Community Event: An event that is held at the library location or created in association with the library
	- ID (Primary Key)
		- Numeric Type where ID >= 0 and ID < ∞
	- Organizer First Name
		- Variable Character String with Max Size of 100
	- Organizer Last Name
		- Variable Character String with Max Size of 100
	- Event Start Time
		- Numeric In Epoch Time
	- Event Time
		- Numeric In Epoch Time
	- Longitudinal Location
		- Numeric Value where Longitude >= -180 and Longitude <= 180
	- Latitudinal Location
		- Numeric Value where Latitude >= -90 and Latitude <= 90
3. Material: This is the physical instance of something that can be checked out from the library
	- ID (Primary Key)
		- Numeric Type where ID >= 0 and ID < ∞
3. Book: A written artifact registered in the ISBN Registry
	- ISBN (Primary Key)
		- 13 Digit Numeric Value to support ISBN-10 and ISBN-13
	- Title
		- Variable Character String with Max of 100 Characters
4. Book Series: A Collection of Books that are intentionally grouped together
	- ID (Primary Key)
   		- Numeric Type where ID >= 0 and ID < ∞
	- Name
		- Variable Character String with Max Size of 100
6. Author: A person who writes a book
   	- ID (Primary Key)
		- Numeric Type where ID >= 0 and ID < ∞
  	- Last Name
		- Variable Character String with Max Size of 100
	- First Name
		- Variable Character String with Max Size of 100
7. Publisher: An entity that is reponsible for managing and publishing books
   	- ID (Primary Key)
   		- Numeric Type where ID >= 0 and ID < ∞
	- Name
		- Variable Character String with Max Size of 100
8. Movie: A visual artifact registered in the ISAN Registry
	- ISAN (Primary Key)
		- Alphanumeric String that is 24 Characters
	- Release Date
		- Numeric value in Epoch Time
	- Title
		- Variable Character String with Max of 100 Characters
9. Director: A person who directs a movie
   	- ID (Primary Key)
		- Numeric Type where ID >= 0 and ID < ∞
  	- Last Name
		- Variable Character String with Max Size of 100
	- First Name
		- Variable Character String with Max Size of 100
10. Studio: An entity that is reponsible for managing and releasing movies
   	- ID (Primary Key)
   		- Numeric Type where ID >= 0 and ID < ∞
	- Name
		- Variable Character String with Max Size of 100
### Generalized/Specialized Entities
1. Staff (Inherits from Member): These are people who work at the library
	- Work Email
		- String Compliant with RFC522
	- Work Phone #
		- Numeric Value that is at most 15 digits
	- Start Date
		- Numeric in Epoch Time
	- Salary
		- Numeric value where Salary >= 0
2. Book Copy (Inherits from Material): These are physical instances of an abstract book
3. Movie Copy (Inherits from Material): These are physical instances of an abstract movie
### Relationships
1. Member RESERVES Room
   	- Multiplicities
   		- 1 Member can reserve (0..N) Rooms
   		- 1 Room can be reserved by (0..1) People
   	- Attributes
   		- Duration
   			- Numeric Value (in Minutes) where Duration >= 15 and Duration <= 60
   		- Date
			- Numeric Value in Epoch Time
2. Member REGISTERS for Community Event
	- Multiplicities
		- 1 Member can register for (0..N) Community Events
		- 1 Community Event can registered by (1..M) Members
3. Staff ORGANIZES Community Event
	- Multiplicities
		- 1 Staff can organize (0..N) Community Events
		- 1 Community Event can registered by (1..M) Staff
4. Member LOANS Material
   	- Multiplicities
   		- 1 Member can loan (0..N) material
   		- 1 Material can be loaned by (0..1) Members
   	- Attributes
   		- Duration
   			- Numeric Value (in Minutes) where Duration >= 15 and Duration <= 60
   		- Start Date
			- Numeric Value in Epoch Time
5. Member DONATES Material
   	- Multiplicities
   		- 1 Member can donate (0..N) material
   		- 1 Material can be donated by (1..1) Members
6. Member RESERVES Material
   	- Multiplicities
   		- 1 Member can loan (0..N) material
   		- 1 Material can be loaned by (1..1) Members
7. Member SEARCHES Book
   	- Multiplicities
   		- 1 Member can search (0..N) books
   		- 1 book can be searched by (0..M) Members
8. Member SEARCHES Movie
   	- Multiplicities
   		- 1 Member can search (0..N) movies
   		- 1 movie can be searched by (0..M) Members
9. Book HAS Book Copy
   	- Multiplicities
   	  	- 1 Book has (1..N) Book Copies
   	  	- 1 Book Copy is associated with (1..1) Books
10. Movie HAS Movie Copy
   	- Multiplicities
   	  	- 1 Movie has (1..N) Movie Copies
   	  	- 1 Movie Copy is associated with (1..1) Movies
11. Author WRITES Book
	- Multiplicities
		- 1 Author can write (1..N) books
		- 1 Book can be written (1..M) Authors
12. Publisher PUBLISHES Book
	- Multiplicities
		- 1 Publisher can publish (1..N) Books
		- 1 Book can be published by (1..1) Publishers
11. Director DIRECTS Movie
	- Multiplicities
		- 1 Director can direct (1..N) Movies
		- 1 Movie can be directed (1..M) Directors
12. Studio RELEASES Movie
	- Multiplicities
		- 1 Studio can release (1..N) Movies
		- 1 Movie can be released by (1..1) Studios
## Entity/Relationship Diagram
The source code for the following diagram is located at [Reference File]( ERDiagram_Source.md )<br/>
The ER Diagram can be found [Here](ER_Diagram_FINAL.png)

## Meeting Logs
**Reference Document:** [030725](../documentation/meeting_logs/030725.md)<br/>
**Reference Document:** [031025](../documentation/meeting_logs/031025.md)<br/>
**Reference Document:** [031424](../documentation/meeting_logs/031425.md)
