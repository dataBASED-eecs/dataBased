# EECS 447 Final Project Part 2

## Introduction

  The library management system should facilitate knowledge sharing, community meeting and community events within the library. To do this, it should include an interface for members to checkout and return library materials. If materials are returned late, members will be able to view and resolve late fees. Members will also be able to view community events at the library and register to attend specific events. Library staff should be able to use the system to view, add and remove materials. The system will help librarians track and resolve late fees. In addition to intreating with library materials library, staff should be able to create and manage community events. The library margent system will not include inter-library communication, loans, or fundraising features. It will also not include financial features like donations, sales, or payroll. The system will not provide analytics of the library over time or suggest materials for either the library or members.

## Stakeholders

  The two main stakeholder groups for the library database are the librarians and the members of the library. Both parties will need an interface to search for materials within the library and modify the status of materials. They will also both interact with the library as a community meeting place. To facilitate that, the database system should track community events.
  
  The librarians will interact with the library database to search for, add and remove library materials. They will also need to organize community events, and set up rooms within the library that can be reserved. It would be helpful for the librarians to know how many people plan to attend each community event.
  
  The members of the library will use the library database to search for, checkout and return books. Members must also be able to register attendance for community events and reserve rooms within the library. If a member returns a book late, they must be able to view the late fees they owe and pay the late fees. The database system should facilitate tracking of late fees.
  
  Finally, the library administrators need to be able to see a high level view of the library's status. They should be able to see the total late fees the library is due, the materials the library has access to and how many materials are currently lent out. While the may not be directly lending or returning materials from the library system, this high level view will help them keep track of the health of the library

## Conceptual Requirements
### Data Entities
- Member: These are people who use the library, but do not work there
  - <u>Member ID</u> (Primary Key)
    - Numeric
    - ID >= 0
    - ID < ∞
  - First Name
    - String
    - Last Name
    - String
  - DOB
    - Numeric in Epoch Time
    - Phone Number
    - Numeric
    - International Phone Number (At Most 15 digits)
  - Email
    - String
    - RFC522 Compliant Email Address
  - Balance
    - Floating Point (two decimal places)
    - Balance <= 0
  - Staff: These are people who work at the library
    - <u>Staff ID</u> (Primary Key)
      - Numeric
      - ID >= 0
      - ID < ∞
    - First Name
      - String
    - Last Name
      - String
    - DOB
      - Numeric in Epoch Time
    - Work Phone Number
      - Numeric
      - International Phone Number (At Most 15 digits)
    - Work Email
     - String
     - RFC522 Compliant Email Address
   - Salary
     - Floating Point (two decimal places)
     - Salary >= 0
   - Start Date
     - Numeric in Epoch Time
- Book
  - Title
    - String
  - Author First Name
    - String
  - Author Last Name
    - String
  - Publishing Date
    - Numeric in Epoch Time
  - Publisher
    - String
  - ISBN
    - Numeric (13 digits to account for ISBN-10 and ISBN-13)
  - Quantity
    - Numeric
- Book Series
  - Series Name
    - String
- Movie
  - Title
    - String
  - Director First Name
    - String
  - Director Last Name
    - String
  - Release Date
    - Numeric in Epoch Time
  - Studio
    - String
  - ISAN
    - Alphanumeric (24 digits to account for ISAN-10 and ISAN-13)
  - Quantity
    - Numeric
- Rooms
  - Room Number
    - Numeric
    - Number >= 0
  - Room Capacity
    - Numeric
    - Number > 0
- Community Event
  - Organizer First Name
    - String
  - Organizer Last Name
    - String
  - Start Time
    - Numeric in Epoch Time
  - End Time 
    - Numeric in Epoch Time
  - Longitudinal Location
    - Floating Point
    - Location >= -180
    - Location <= 180
  - Latitudinal Location
    - Floating Point
    - Location >= -180
    - Location <= 180
### Functional Requirements
#### Conceptual Queries
- Members may search for any of the materials that the library has in inventory based on any attribute of said entity
  - Example: A user may search a book by title, as well as ISBN
- Members may hold a reserveration on library materials
- Members may reserve any of study rooms
  - Reservation start time
  - Reservation end time
  - Reservation name
- Members may create a community event
- Members may donate library materials
- Members may have the ability to pay fines and fees
- Members may see all library materials checked out at any given time
- Staff may check out a library material for a user. If a material is on hold the checkout is denied
  - Check out date
  - Expected Return Date
- Staff may process returns of library materials
- Staff may extend the return data for a member's material checkout
- Staff may bestow a membership on a new member
- Staff may administer fines and fees to members
- Staff may view how many donations a member has donated
- Staff may the ability to add/remove items from library inventory
#### Report Generation
- All materials
  - The user of the database will be able to see all the materials that a library has based on attributes of interest
- All room reservations based on relevant attributes
  - The user of the database will be able to see all the relevant datapoints for any room reservation
- View all fines (with implicit fine calculation)
  - The user of the database will view all the fines for every member in the database
- View number of donations for every member
  - The user of the database will be able to see the number of items that every member has donated
- View the materials on hold
  - The user of the database will be able to see what materials each user has placed a reservation for
- View all the book series available and the books attributed to them
- View all the community events available based on relevant attributes
- View all the members attending community events
- (Elaborate) - Determine how long a member can check out a book based on how often they accumlate and pay back fines

### Non-Functional Requirements

#### 1. Performance

- Response Time: The system should respond to user queries (e.g., searching for books, checking out materials) within 2 seconds under normal load.
    
- Throughput: The system should handle at least 5 concurrent users without significant performance degradation.
    
- Latency: Database operations (e.g., adding, updating, or deleting records) should complete within 1 second for 95% of transactions.
#### 2. Availability

- The system should have an uptime of 99.9%, ensuring it is available for use during library operating hours.
    
- The system should support failover mechanisms to ensure continuity of service in case of hardware or software failures.
#### 3. Security

- Authentication: All users (members and staff) must authenticate using a secure method (e.g., username/password, multi-factor authentication).
    
- Authorization: Access to sensitive operations (e.g., adding/removing materials, managing fines) should be restricted to authorized staff members.
    
- Data Encryption: Sensitive data (e.g., member information, payment details) should be encrypted both in transit and at rest.
    
- Audit Logs: The system should maintain logs of all critical operations (e.g., checkouts, returns, fee payments) for auditing purposes.

#### 4. Reliability

- The system should have a mean time between failures (MTBF) of at least 1,000 hours.
    
- In case of a failure, the system should recover within 5 minutes (mean time to recovery, MTTR).
    
#### 5. Usability

- User Interface: The system should have an intuitive and user-friendly interface for both members and staff.
    
- Accessibility: The system should comply with accessibility standards (e.g., WCAG 2.1) to ensure it is usable by individuals with disabilities.
    
- Training: The system should include documentation and training materials to help users (especially staff) learn how to use it effectively.

#### 6. Scalability

- The system should support horizontal scaling to accommodate an increasing number of users - the amount of users our library serves - (e.g., from 100 to 10,000 users) without requiring significant architectural changes.
    
- The database should handle up to 1 million records (e.g., books, members, events) without performance degradation.
  
#### 7. Data Integrity

- The system should enforce referential integrity (e.g., a member cannot check out a book that does not exist in the database).
    
- The system should prevent data corruption by implementing transaction management (e.g. ACID properties).
#### 8. Backup and Recovery

- The system should perform daily backups of the database to prevent data loss.
    
- In case of data loss, the system should be able to restore data from backups within 1 hour.
  
#### 9. Extensibility

- The system should allow for the addition of new features (e.g., support for new types of materials, integration with external APIs) without requiring significant rework.


## System Requirements
### Software Requirements
- Operating System: Ubuntu 22.04.5 LTS (Jammy Jellyfish) Linux
- Database Management System (DBMS): PostgreSQL 16
- Rust Toolchain: Rust 1.75+, interfacing the DBMS with `sqlx`, the open source Rust SQL toolkit
### Hardware Requirements
  - Components (built based on the KU EECS cycle servers):
    - CPU: Intel Xeon Gold 6526Y, 3500 MHz, 38400 KB Cache Size, 16 Cores (64 total units, ensuring high concurrency and parallelism)
    - Memory (RAM): 264 GB DDR5 ECC RAM (for large-scale in-memory processing and caching)
    - Storage:
      - Primary Storage: 2TB NVMe SSD (low-latency for database transactions)
      - Backup Storage: 8TB SATA SSD (for periodic snapshots and redundancy)
      - RAID Configuration: RAID 1 or RAID 10 (for fault tolerance and data integrity)
    - Networking:
      - NIC: Dual 25DbE Network Adapters (for high-speed data transmission)
      - Firewall & Security UFW (Uncomplicated Firewall) with IP whitelisting
## Appendices
None to include.

## Meeting Logs
**Reference Document: **[022125](https://github.com/dataBASED-eecs/dataBased/blob/main/documentation/meeting_logs/022125.md)<br/>
**Reference Document: **[022825](https://github.com/dataBASED-eecs/dataBased/blob/main/documentation/meeting_logs/022825.md)
