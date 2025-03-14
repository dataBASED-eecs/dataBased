View the Graph on https://dreampuf.github.io/GraphvizOnline/?engine=dot#

```
graph ERDiagram 
    {
    graph [compound=true];
    node [shape=record];
    ranksep=2.0;
    nodesep=1.0;
    splines=true;
    rankdir=LR;
    
    /***************************************************************************************
    Entity Definitions
    ***************************************************************************************/
    subgraph cluster_member_entity
        {
        style=filled;
        fillcolor=lightblue;
        // Define the entity node
        node[ shape=rect ]
        member[shape=rect, label="Member"]
        
        // Define the attributes
        node[ shape=ellipse ]
        member_member_id[ label=<<u>Member ID</u>> ]
        member_dob[ label="Date of Birth" ]
        member_fname[ label="First Name" ]
        member_lname[ label="Last Name" ]
        member_email[ label="Email" ]
        member_balance[ label="Outstanding Balance" ]

        // Link the attributes
        member -- member_member_id
        member -- member_dob
        member -- member_fname
        member -- member_lname
        member -- member_email
        member -- member_balance
        }
    
    /* Staff Entity Definition */
    subgraph cluster_staff_entity 
        {
        style=filled;
        fillcolor=lightblue;
        // Define the entity node
        node[ shape=rect ]
        staff[ label="Staff" ]
        
        // Define the attributes
        node[ shape=ellipse ]
        staff_work_phone_number[ label="Work Phone #" ]
        staff_salary[ label="Salary" ]
        staff_work_email[ label="Work Email" ]
        staff_start_date[ label="Start Date" ]

        // Link the attributes
        staff -- staff_work_phone_number
        staff -- staff_salary
        staff -- staff_work_email
        staff -- staff_start_date
        }
        
    /* Cluster Entity Definition */
    subgraph cluster_material_entity
        {
        style=filled;
        fillcolor=lightblue;
        // Define the entity node
        node[ shape=rect ]
        material[ label="Material" ]
        
        // Define the attributes
        node[ shape=ellipse ]
        material_id[ label=<<u>ID</u>> ]

        // Link the attributes
        material -- material_id
        }
        
    /* Book Entity Definition */
    subgraph cluster_book_entity 
        {
        style=filled;
        fillcolor=lightblue;
        // Define the entity node
        node[ shape=rect ]
        book[ label="Book" ]
        
        // Define the attributes
        node[ shape=ellipse ]
        book_title[ label="Title" ]
        book_isbn[ label=<<u>ISBN</u>> ]

        // Link the attributes
        book -- book_title
        book -- book_isbn
        }
        
    /* Book-Copy Entity Definition */
    subgraph book_copy_entity 
        {
        // Define the entity node
        node[ shape=rect ]
        book_copy[ label="Book Copy", style=filled, fillcolor=lightblue ]
        }
    
    /* Author Entity Definition */
    subgraph cluster_author_entity 
        {
        style=filled;
        fillcolor=lightblue;
        // Define the entity node
        node[ shape=rect ]
        author[ label="Author" ]
        
        // Define the attributes
        node[ shape=ellipse ]
        author_id[ label=<<u>ID</u>> ]
        author_fname[ label="First Name" ]
        author_lname[ label="Last Name" ]

        // Link the attributes
        author -- author_id
        author -- author_fname
        author -- author_lname
        }
        
    /* Publisher Entity Definition */
    subgraph cluster_publisher_entity 
        {
        style=filled;
        fillcolor=lightblue;
        // Define the entity node
        node[ shape=rect ]
        publisher[ label="Publisher" ]
        
        // Define the attributes
        node[ shape=ellipse ]
        publisher_id[ label=<<u>ID</u>> ]
        publisher_name[ label="Name" ]

        // Link the attributes
        publisher -- publisher_id
        publisher -- publisher_name
        }
    
    
    /* Book Series Entity Definition */
    subgraph cluster_book_series_entity 
        {
        style=filled;
        fillcolor=lightblue;
        // Define the entity node
        node[ shape=rect ]
        book_series[ label="Book Series" ]
        
        // Define the attributes
        node[ shape=ellipse ]
        book_series_id[ label=<<u>ID</u>> ]
        book_series_name[ label="Name" ]

        // Link the attributes
        book_series -- book_series_id
        book_series -- book_series_name
        }
    
    /* Movie Entity Definition */
    subgraph cluster_movie_entity 
        {
        style=filled;
        fillcolor=lightblue;
        // Define the entity node
        node[ shape=rect ]
        movie[ label="Movie" ]
        
        // Define the attributes
        node[ shape=ellipse ]
        movie_isan[ label=<<u>ISAN</u>> ]
        movie_title[ label="Title" ]
        movie_release_date[ label="Release Date" ]

        // Link the attributes
        movie -- movie_isan
        movie -- movie_title
        movie -- movie_release_date
        }
        
    /* Movie Copy Definition */
    subgraph movie_copy_entity 
        {
        // Define the entity node
        node[ shape=rect ]
        movie_copy[ label="Movie Copy", style=filled, fillcolor=lightblue ]
        }
        
    /* Director Entity Definition */
    subgraph cluster_director_entity 
        {
        style=filled;
        fillcolor=lightblue;
        // Define the entity node
        node[ shape=rect ]
        director[ label="Director" ]
        
        // Define the attributes
        node[ shape=ellipse ]
        director_id[ label=<<u>ID</u>> ]
        director_fname[ label="First Name" ]
        director_lname[ label="Last Name" ]

        // Link the attributes
        director -- director_id
        director -- director_fname
        director -- director_lname
        }
    
    /* Studio Entity Definition */
    subgraph cluster_studio_entity 
        {
        style=filled;
        fillcolor=lightblue;
        // Define the entity node
        node[ shape=rect ]
        studio[ label="Studio" ]
        
        // Define the attributes
        node[ shape=ellipse ]
        studio_id[ label=<<u>ID</u>> ]
        studio_name[ label="Name" ]

        // Link the attributes
        studio -- studio_id
        studio -- studio_name
        }
        
    /* Room Entity Definition */
    subgraph cluster_room_entity 
        {
        style=filled;
        fillcolor=lightblue;
        // Define the entity node
        node[ shape=rect ]
        room[ label="Room" ]
        
        // Define the attributes
        node[ shape=ellipse ]
        room_number[ label=<<u>Number</u>> ]
        room_capacity[ label="Capacity" ]

        // Link the attributes
        room -- room_number
        room -- room_capacity
        }
        
    /* Community Event Entity Definition */
    subgraph cluster_community_event_entity 
        {
        style=filled;
        fillcolor=lightblue;
        // Define the entity node
        node[ shape=rect ]
        community_event[ label="Community Event" ]
        
        // Define the attributes
        node[ shape=ellipse ]
        community_event_id[ label=<<u>ID</u>> ]
        community_event_organizer_fname[ label="Organizer First Name" ]
        community_event_organizer_lname[ label="Organizer Last Name" ]
        community_event_start_time[ label="Start Time" ]
        community_event_end_time[ label="End Time" ]
        community_event_longitudinal_location[ label="Longitude" ]
        community_event_latitudinal_location[ label="Latitude" ]

        // Link the attributes
        community_event -- community_event_id
        community_event -- community_event_organizer_fname
        community_event -- community_event_organizer_lname
        community_event -- community_event_start_time
        community_event -- community_event_end_time
        community_event -- community_event_longitudinal_location
        community_event -- community_event_latitudinal_location
        }
        
    /***************************************************************************************
    Generalization and Specialization Definition
    ***************************************************************************************/
    /* A staff is a specialized version of a member */
    subgraph member_is_a_staff_relationship
        {
        node[ shape=triangle orientation=270 ]
        member_is_a_staff[ label="Is A" , fillcolor=lightgreen, style=filled ]
        
        member -- member_is_a_staff[ ltail=cluster_member_entity, lhead=member_is_a_staff_relationship ]
        member_is_a_staff -- staff[ ltail=member_is_a_staff_relationship, lhead=cluster_staff_entity ]
        }
        
    subgraph copy_is_material_relationship
        {
        node[ shape=triangle orientation=270 ]
        copy_is_a_material[ label="Is A" , fillcolor=lightgreen, style=filled ]
        
        material -- copy_is_a_material[ ltail=cluster_material_entity, lhead=copy_is_a_material ]
        copy_is_a_material -- book_copy[ ltail=copy_is_a_material, lhead=book_copy_entity ]
        copy_is_a_material -- movie_copy[ ltail=copy_is_a_material, lhead=movie_copy_entity ]
        }
        
    /***************************************************************************************
    Relationship Definition
    ***************************************************************************************/  
    /* Member registers for Community Event Relationships */
    subgraph member_registers_community_event_relationship
        {
        style=filled;
        fillcolor="#ffcccc";
        node[ shape=diamond ]
        member_registers_community_event[ label="Registers", fillcolor="#ffcccc", style=filled ]
        }
        
    /* Staff can organize a community event */
    subgraph staff_organizes_community_event_relationship
        {
        node[ shape=diamond ]
        staff_organizes_community_event[ label="Organizes", fillcolor="#ffcccc", style=filled ]
        }
    
    /* A member can loan a material */
    subgraph cluster_member_loans_material_relationship
        {
        node[ shape=diamond ]
        fillcolor="#ffcccc"
        style=filled
        member_loans_material[ label="Loans" ]
        
        node[ shape=ellipse ]
        member_loans_material_loan_start_date[ label="Start Date" ]
        member_loans_material_loan_duration[ label="Duration" ]
        
        member_loans_material -- member_loans_material_loan_start_date
        member_loans_material -- member_loans_material_loan_duration
        }
        
    /* A member can reserve a material */
    subgraph cluster_member_reserves_material_relationship
        {
        node[ shape=diamond ]
        fillcolor="#ffcccc"
        style=filled
        member_reserves_material[ label="Reserves" ]
        
        node[ shape=ellipse ]
        member_reserves_material_reservation_date[ label="Reservation Date" ]
        
        member_reserves_material -- member_reserves_material_reservation_date
        }
        
    /* A member can reserve a material */
    subgraph member_donates_material_relationship
        {
        node[ shape=diamond ]
        member_donates_material[ label="Donates", fillcolor="#ffcccc", style=filled ]
        }
        
    
    /* A member can search for a book */
    subgraph member_searches_book_relationship
        {
        node[ shape=diamond ]
        member_searches_book[ label="Searches", fillcolor="#ffcccc", style=filled ]
        }
        
    /* Member can search for a movie */
    subgraph member_searches_movie_relationship
        {
        node[ shape=diamond ]
        member_searches_movie[ label="Searches", fillcolor="#ffcccc", style=filled ]
        }
        
    /* Members can reserve a rooms */
    subgraph cluster_member_reserves_room_relationship
        {
        node[ shape=diamond ]
        fillcolor="#ffcccc"
        style=filled
        member_reserves_room[ label="Reserves"]
        
        node[ shape=ellipse ]
        member_reserves_room_reservation_date[ label="Date" ]
        member_reserves_room_reservation_duration[ label="Duration" ]
        
        member_reserves_room -- member_reserves_room_reservation_date
        member_reserves_room -- member_reserves_room_reservation_duration
        }
        
    /* Authors can write books */
    subgraph author_writes_book_relationship
        {
        node[ shape=diamond ]
        author_writes_book[ label="Writes", fillcolor="#ffcccc", style=filled ]
        }
        
    /* Publishers can publish book */
    subgraph publisher_publishes_book_relationship
        {
        node[ shape=diamond ]
        publisher_publishes_book[ label="Publishes", fillcolor="#ffcccc", style=filled ]
        }
        
    /* Books are part of a book series */
    subgraph cluster_book_is_part_of_book_series_relationship
        {
        style=filled;
        fillcolor="#ffcccc";
        node[ shape=diamond ]
        book_is_part_of_book_series[ label="Is part of" ]
        
        node[ shape=ellipse ]
        book_is_part_of_book_series_order_num[ label="Order" ]
        
        book_is_part_of_book_series -- book_is_part_of_book_series_order_num
        }
        
    /* Directors can direct movies */
    subgraph director_directs_movie_relationship
        {
        node[ shape=diamond ]
        director_directs_movie[ label="Directs", fillcolor="#ffcccc", style=filled ]
        }
        
    /* Studios can release movies */
    subgraph studio_releases_movie_relationship
        {
        node[ shape=diamond ]
        studio_releases_movie[ label="Releases", fillcolor="#ffcccc", style=filled ]
        }
        
    /* Book can have copies */
    subgraph book_has_book_copy_relationship
        {
        node[ shape=diamond ]
        book_has_book_copy[ label="Has", fillcolor="#ffcccc", style=filled ]
        }
        
    /* Movie can have copies */
    subgraph movie_has_movie_copy_relationship
        {
        node[ shape=diamond ]
        movie_has_movie_copy[ label="Has", fillcolor="#ffcccc", style=filled ]
        }
        
    /***************************************************************************************
    Group Relationship Links
    ***************************************************************************************/
    /* Group all Member to Community Event relationships */
    subgraph member_to_community_event
        {
        // Members Register for community events
        member -- member_registers_community_event[ ltail=cluster_member_entity, lhead=member_registers_community_event_relationship, label="(0..N)" ]
        member_registers_community_event -- community_event[ ltail=member_registers_community_event_relationship, lhead=cluster_community_event_entity, label="(1..M)", color="black:invis:black" ]
        }
        
    /* Group all Staff to Community Event relationshiops */
    subgraph staff_to_community_event
        {
        // Staffs Organize community events
        staff -- staff_organizes_community_event[ ltail=cluster_staff_entity, lhead=staff_organizes_community_event_relationship, xlabel="(0..N)" ]
        staff_organizes_community_event -- community_event[ ltail=staff_organizes_community_event_relationship, lhead=cluster_community_event_entity, label="(1..M)", color="black:invis:black" ]
        }
        
    /* Group all the Member to Material relationships */
    subgraph member_to_material
        {
        member -- member_loans_material[ ltail=cluster_member_entity, lhead=cluster_member_loans_material_relationship, xlabel="(0..N)" ]
        member_loans_material -- material[ ltail=cluster_member_loans_material_relationship, lhead=cluster_material_entity, xlabel="(0..1)" ]
        
        member -- member_reserves_material[ ltail=cluster_member_entity, lhead=cluster_member_reserves_material_relationship, label="(0..N)" ]
        member_reserves_material -- material[ ltail=cluster_member_reserves_material_relationship, lhead=cluster_material_entity, label="(0..M)" ]
        
        member -- member_donates_material[ ltail=cluster_member_entity, lhead=member_donates_material_relationship, xlabel="(0..N)" ]
        member_donates_material -- material[ ltail=member_donates_material_relationship, lhead=cluster_material_entity, label="(1..1)", color="black:invis:black" ]
        }
        
    /* Group all the Member to Book relationships */
    subgraph member_to_book
        {
        member -- member_searches_book[ ltail=cluster_member_entity, lhead=member_searches_book_relationship, label="(0..N)" ]
        member_searches_book -- book[ ltail=member_searches_book_relationship, lhead=cluster_book_entity, label="(0..M)" ]
        }
        
    /* Group all the Member to Movie relationships */
    subgraph member_to_movie
        {
        member -- member_searches_movie[ ltail=cluster_member_entity, lhead=member_searches_movie_relationship, label="(0..N)" ]
        member_searches_movie -- movie[ ltail=member_searches_movie_relationship, lhead=cluster_movie_entity, label="(0..M)" ]
        }
        
    /* Group all the Member to Room relationships */
    subgraph member_to_room
        {
        member -- member_reserves_room[ ltail=cluster_member_entity, lhead=cluster_member_reserves_room_relationship, label="(0..N)" ]
        member_reserves_room -- room[ ltail=cluster_member_reserves_room_relationship, lhead=cluster_room_entity, label="(0..1)" ]
        }
        
    /* Group all the Author to Book relationships */
    subgraph author_to_book
        {
        author -- author_writes_book[ ltail=cluster_author_entity, lhead=author_writes_book_relationship, xlabel="(1..N)", color="black:invis:black" ]
        author_writes_book -- book[ ltail=author_writes_book_relationship, lhead=cluster_book_entity, label="(1..M)", color="black:invis:black" ]
        }
        
    /* Group all the Publisher to Book Relationships */
    subgraph publisher_to_book
        {
        publisher -- publisher_publishes_book[ ltail=cluster_publisher_entity, lhead=publisher_publishes_book_relationship, label="(1..M)" ]
        publisher_publishes_book -- book[ ltail=publisher_publishes_book_relationship, lhead=cluster_book_entity, label="(1..1)", color="black:invis:black" ]
        }
        
    /* Group all the Book Series to Book relationships */
    subgraph book_series_to_book
        {
        book_series -- book_is_part_of_book_series[ ltail=cluster_book_series_entity, lhead=cluster_book_is_part_of_book_series_relationship, color="black:invis:black", xlabel="(1..M)" ]
        book_is_part_of_book_series -- book[ ltail=cluster_book_is_part_of_book_series_relationship, lhead=cluster_book_entity, label="(0..1)" ]
        }
        
    /* Group all the Director to Movie Relationships */
    subgraph director_to_movie
        {
        director -- director_directs_movie[ ltail=cluster_director_entity, lhead=director_directs_movie_relationship, xlabel="(1..N)", color="black:invis:black" ]
        director_directs_movie -- movie[ ltail=director_directs_movie_relationship, lhead=cluster_movie_entity, label="(1..M)", color="black:invis:black" ]
        }
        
    /* Group all the Studio to Movie Relationships */
    subgraph studio_to_movie
        {
        studio -- studio_releases_movie[ ltail=cluster_studio_entity, lhead=studio_releases_movie_relationship, label="(1..M)" ]
        studio_releases_movie -- movie[ ltail=studio_releases_movie_relationship, lhead=cluster_movie_entity, label="(1..1)", color="black:invis:black" ]
        }
        
    /* Group all the book to book copy relationships */
    subgraph book_to_book_copy
        {
        book -- book_has_book_copy[ ltail=cluster_book_entity, lhead=book_has_book_copy_relationship, label="(1..M)", color="black:invis:black" ]
        book_has_book_copy -- book_copy[ ltail=book_has_book_copy_relationship, lhead=book_copy_entity, xlabel="(1..1)", color="black:invis:black" ]
        }
        
    /* Group all the movie to movie copy relationships */
    subgraph book_to_book_copy
        {
        movie -- movie_has_movie_copy[ ltail=cluster_movie_entity, lhead=movie_has_movie_copy_relationship, label="(1..M)", color="black:invis:black" ]
        movie_has_movie_copy -- movie_copy[ ltail=movie_has_movie_copy_relationship, lhead=movie_copy_entity, xlabel="(1..1)", color="black:invis:black" ]
        }
    }
```
