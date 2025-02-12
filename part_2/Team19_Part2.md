# EECS 447 Final Project Part 1

## Introduction
## Stakeholders
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
- Users may check out any number of books
  - Check out Date
    - Numeric in Epoch Time
- User may check out any number of movies
- 
### Non-Functional Requirements
## System Requirements
### Software Requirements
### Hardware Requirements
## Appendices
