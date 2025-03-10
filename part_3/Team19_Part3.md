# Graphviz
graph ER_Diagram 
    {
    /* Entity Definition */
    node[ shape=rect ]
    member[ label="Member" ]
    staff[ label="Staff" ]
    book[ label="Book" ]
    book_copy[ label="Book Copy" ]
    author[ label="Author" ]
    publisher[ label="Publisher" ]
    book_series[ label="Book Series" ]
    movie[ label="Movie" ]
    movie_copy[ label="Movie Copy" ]
    director[ label="Director" ]
    studio[ label="Studio" ]
    room[ label="Room" ]
    community_event[ label="Community Event" ]
    
    /* Attribute Definitions */
    node[ shape=ellipse ]
    
    // The following are member specific attributes
    member_member_id[ label=<<u>Member ID</u>> ]
    member_dob[ label="Date of Birth" ]
    member_fname[ label="First Name" ]
    member_lname[ label="Last Name" ]
    member_email[ label="Email" ]
    member_balance[ label="Outstanding Balance" ]
    
    // The following are staff specific attributes
    staff_work_phone_number[ label="Work Phone #" ]
    staff_salary[ label="Salary" ]
    staff_work_email[ label="Work Email" ]
    staff_start_date[ label="Start Date" ]
    
    // The following are book specific attributes
    book_title[ label="Title" ]
    book_isbn[ label=<<u>ISBN</u>> ]
    
    // The following are copy specific attributes
    book_copy_id[ label=<<u>ID</u>> ]
    
    // The following are Author specific attributes
    author_id[ label=<<u>ID</u>> ]
    author_fname[ label="First Name" ]
    author_lname[ label="Last Name" ]
    
    // The following are the publisher specific attributes
    publisher_id[ label=<<u>ID</u>> ]
    publisher_name[ label="Name" ]
    
    // The following are the book series specific attributes
    book_series_id[ label=<<u>ID</u>> ]
    book_series_name[ label="Name" ]
    
    // The following are movie specific attributes
    movie_isan[ label=<<u>ISAN</u>> ]
    movie_title[ label="Title" ]
    movie_release_date[ label="Release Date" ]
    
    // The following are movie copy specific attributes
    movie_copy_id[ label=<<u>ID</u>> ]
    
    // The following are director specific attributes
    director_id[ label=<<u>ID</u>> ]
    director_fname[ label="First Name" ]
    director_lname[ label="Last Name" ]
    
    // The following are studio specific attributes
    studio_id[ label=<<u>ID</u>> ]
    studio_name[ label="Name" ]
    
    // The following are room specific attributes
    room_number[ label=<<u>Number</u>> ]
    room_capacity[ label="Capacity" ]
    
    // The following are community event specific attributes
    community_event_id[ label=<<u>ID</u>> ]
    community_event_organizer_fname[ label="Organizer First Name" ]
    community_event_organizer_lname[ label="Organizer Last Name" ]
    community_event_start_time[ label="Start Time" ]
    community_event_end_time[ label="End Time" ]
    community_event_longitudinal_location[ label="Longitude" ]
    community_event_latitudinal_location[ label="Latitude" ]
    
    /* Relationship Definitions */
    node[ shape=diamond ]
    // Member can search for these
    book_search[ label="Search" ]
    movie_search[ label="Search" ]
    
    // Members can loan books and movies
    book_loan[ label="Loan" ]
    movie_loan[ label="Loan" ]
    
    // Members can reserve spot for books and movies
    book_reserve[ label="Reserve" ]
    movie_reserve[ label="Reserve" ]
    
    // Members can donate books and movies
    book_donate[ label="Donate" ]
    movie_donate[ label="Donate" ]
    
    // Member can reserve room
    room_reserve[ label="Reserve" ]
    
    // Member can register for community event
    register_community_event[ label="Register" ]
    
    // Staff can organize community event
    organize_community_event[ label="Organize" ]
    
    /* Generalization/Specialization Definitions */
    node[ shape=triangle orientation=180 ]
    staff_is_a_member[ label="Is A" ]
    
    /* Entity to attribute lines */
    // Link the member attributes
    member -- member_member_id
    member -- member_dob
    member -- member_fname
    member -- member_lname
    member -- member_email
    member -- member_balance
    
    // Link the staff attributes
    staff -- staff_work_phone_number
    staff -- staff_salary
    staff -- staff_work_email
    staff -- staff_start_date
    
    // Link the book attributes
    book -- book_title
    book -- book_isbn
    
    // Link the Copy attributes
    book_copy -- book_copy_id
    
    // Link the Author attributes
    author -- author_id
    author -- author_fname
    author -- author_lname
    
    // Link the publisher attributes
    publisher -- publisher_id
    publisher -- publisher_name
    
    // Link the Book Series Attributes
    book_series -- book_series_id
    book_series -- book_series_name
    
    // Link the movie attributes
    movie -- movie_isan
    movie -- movie_title
    movie -- movie_release_date
    
    // Link the Movie Copy Attributes
    movie_copy -- movie_copy_id
    
    // Link the Director attributes
    director -- director_id
    director -- director_fname
    director -- director_lname
    
    // Link the Studio Attributes
    studio -- studio_id
    studio -- studio_name
    
    // Link the room attributes
    room -- room_number
    room -- room_capacity
    
    // Link the community event attributes
    community_event -- community_event_id
    community_event -- community_event_organizer_fname
    community_event -- community_event_organizer_lname
    community_event -- community_event_start_time
    community_event -- community_event_end_time
    community_event -- community_event_longitudinal_location
    community_event -- community_event_latitudinal_location
    
    /* Generalization/Specialization Lines */
    member -- staff_is_a_member
    staff_is_a_member -- staff
    
    /* Relation to entity lines */
    // Members can loan books
    member -- book_loan
    book_loan -- book
    
    // Memebers can loan movies
    member -- movie_loan
    movie_loan -- movie
    
    // Members can search for books
    member -- book_search
    book_search -- book
    
    // Memebers can search for movies
    member -- movie_search
    movie_search -- movie
    
    // Members can reserve books
    member -- book_reserve
    book_reserve -- book
    
    // Memebers can reserve movies
    member -- movie_reserve
    movie_reserve -- movie
    
    // Members can donate books
    member -- book_donate
    book_donate -- book
    
    // Memebers can donate movies
    member -- movie_donate
    movie_donate -- movie
    
    // Members can reserve rooms
    member -- room_reserve
    room_reserve -- room
    
    // Member can register for community event
    member -- register_community_event
    register_community_event -- community_event
    
    // Staff can organize community events
    staff -- organize_community_event
    organize_community_event -- community_event
    
    }
