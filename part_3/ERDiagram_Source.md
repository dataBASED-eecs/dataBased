To render this code, please got to [Online Graphviz Compiler](https://dreampuf.github.io/GraphvizOnline/?engine=dot#digraph%20G%20%7B%0A%0A%7D) and paste the following code
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
    subgraph cluster_book_copy_entity 
        {
        // Define the entity node
        node[ shape=rect ]
        book_copy[ label="Book Copy" ]
        
        // Define the attributes
        node[ shape=ellipse ]
        book_copy_id[ label=<<u>ID</u>> ]

        // Link the attributes
        book_copy -- book_copy_id
        }
    
    /* Author Entity Definition */
    subgraph cluster_author_entity 
        {
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
    subgraph cluster_movie_copy_entity 
        {
        // Define the entity node
        node[ shape=rect ]
        movie_copy[ label="Movie Copy" ]
        
        // Define the attributes
        node[ shape=ellipse ]
        movie_copy_id[ label=<<u>ID</u>> ]

        // Link the attributes
        movie_copy -- movie_copy_id
        }
        
    /* Director Entity Definition */
    subgraph cluster_director_entity 
        {
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
        member_is_a_staff[ label="Is A" ]
        
        member -- member_is_a_staff[ ltail=cluster_member_entity, lhead=member_is_a_staff_relationship ]
        member_is_a_staff -- staff[ ltail=member_is_a_staff_relationship, lhead=cluster_staff_entity ]
        }
        
    /***************************************************************************************
    Relationship Definition
    ***************************************************************************************/  
    /* Member registers for Community Event Relationships */
    subgraph member_registers_community_event_relationship
        {
        node[ shape=diamond ]
        member_registers_community_event[ label="Registers" ]
        }
        
    /* Staff can organize a community event */
    subgraph staff_organizes_community_event_relationship
        {
        node[ shape=diamond ]
        staff_organizes_community_event[ label="Organizes" ]
        }
        
    /* A material can be a book copy */
    subgraph material_can_be_book_copy_relationship
        {
        node[ shape=diamond ]
        material_can_be_book_copy[ label="Can Be" ]
        }
    
    /* A material can be a movie copy */
    subgraph material_can_be_movie_copy_relationship
        {
        node[ shape=diamond ]
        material_can_be_movie_copy[ label="Can Be" ]
        }
    
    /* A member can loan a material */
    subgraph member_loans_material_relationship
        {
        node[ shape=diamond ]
        member_loans_material[ label="Loans" ]
        }
        
    /* A member can reserve a material */
    subgraph member_reserves_material_relationship
        {
        node[ shape=diamond ]
        member_reserves_material[ label="Reserves" ]
        }
    
    /* A member can search for a book */
    subgraph member_searches_book_relationship
        {
        node[ shape=diamond ]
        member_searches_book[ label="Searches" ]
        }
        
    /* Member can search for a movie */
    subgraph member_searches_movie_relationship
        {
        node[ shape=diamond ]
        member_searches_movie[ label="Searches" ]
        }
        
    /* Members can reserve a rooms */
    subgraph member_reserves_room_relationship
        {
        node[ shape=diamond ]
        member_reserves_room[ label="Reserves" ]
        }
        
    /* Authors can write books */
    subgraph author_writes_book_relationship
        {
        node[ shape=diamond ]
        author_writes_book[ label="Writes" ]
        }
        
    /* Publishers can publish book */
    subgraph publisher_publishes_book_relationship
        {
        node[ shape=diamond ]
        publisher_publishes_book[ label="Publishes" ]
        }
        
    /* Books are part of a book series */
    subgraph book_is_part_of_book_series_relationship
        {
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
        director_directs_movie[ label="Directs" ]
        }
        
    /* Studios can release movies */
    subgraph studio_releases_movie_relationship
        {
        node[ shape=diamond ]
        studio_releases_movie[ label="Releases" ]
        }
        
        
    /***************************************************************************************
    Group Relationship Links
    ***************************************************************************************/
    /* Group all Member to Community Event relationships */
    subgraph member_to_community_event
        {
        // Members Register for community events
        member -- member_registers_community_event[ ltail=cluster_member_entity, lhead=member_registers_community_event_relationship ]
        member_registers_community_event -- community_event[ ltail=member_registers_community_event_relationship, lhead=cluster_community_event_entity ]
        }
        
    /* Group all Staff to Community Event relationshiops */
    subgraph staff_to_community_event
        {
        // Staffs Organize community events
        staff -- staff_organizes_community_event[ ltail=cluster_staff_entity, lhead=staff_organizes_community_event_relationship ]
        staff_organizes_community_event -- community_event[ ltail=staff_organizes_community_event_relationship, lhead=cluster_community_event_entity ]
        }
        
    /* Group all the Material to Book Copy relationships */
    subgraph material_to_book_copy
        {
        material -- material_can_be_book_copy[ ltail=cluster_material_entity, lhead=material_can_be_book_copy_relationship ]
        material_can_be_book_copy -- book_copy[ ltail=material_can_be_book_copy_relationship, lhead=cluster_book_copy_entity ]
        }
        
    /* Group all the Material to Movie Copy relationships */
    subgraph material_to_movie_copy
        {
        material -- material_can_be_movie_copy[ ltail=cluster_material_entity, lhead=material_can_be_movie_copy_relationship ]
        material_can_be_movie_copy -- movie_copy[ ltail=material_can_be_movie_copy_relationship, lhead=cluster_movie_copy_entity ]
        }
        
    /* Group all the Member to Material relationships */
    subgraph member_to_material
        {
        member -- member_loans_material[ ltail=cluster_member_entity, lhead=member_loans_material_relationship ]
        member_loans_material -- material[ ltail=member_loans_material_relationship, lhead=cluster_material_entity ]
        
        member -- member_reserves_material[ ltail=cluster_member_entity, lhead=member_reserves_material_relationship ]
        member_reserves_material -- material[ ltail=member_reserves_material_relationship, lhead=cluster_material_entity ]
        }
        
    /* Group all the Member to Book relationships */
    subgraph member_to_book
        {
        member -- member_searches_book[ ltail=cluster_member_entity, lhead=member_searches_book_relationship ]
        member_searches_book -- book[ ltail=member_loans_material_relationship, lhead=cluster_book_entity ]
        }
        
    /* Group all the Member to Movie relationships */
    subgraph member_to_movie
        {
        member -- member_searches_movie[ ltail=cluster_member_entity, lhead=member_searches_movie_relationship ]
        member_searches_movie -- movie[ ltail=member_searches_movie_relationship, lhead=cluster_movie_entity ]
        }
        
    /* Group all the Member to Room relationships */
    subgraph member_to_room
        {
        member -- member_reserves_room[ ltail=cluster_member_entity, lhead=member_reserves_room_relationship ]
        member_reserves_room -- room[ ltail=member_reserves_room_relationship, lhead=cluster_room_entity ]
        }
        
    /* Group all the Author to Book relationships */
    subgraph author_to_book
        {
        author -- author_writes_book[ ltail=cluster_author_entity, lhead=author_writes_book_relationship ]
        author_writes_book -- book[ ltail=author_writes_book_relationship, lhead=cluster_book_entity ]
        }
        
    /* Group all the Publisher to Book Relationships */
    subgraph publisher_to_book
        {
        publisher -- publisher_publishes_book[ ltail=cluster_publisher_entity, lhead=publisher_publishes_book_relationship ]
        publisher_publishes_book -- book[ ltail=publisher_publishes_book_relationship, lhead=cluster_book_entity ]
        }
        
    /* Group all the Book Series to Book relationships */
    subgraph book_series_to_book
        {
        book_series -- book_is_part_of_book_series[ ltail=cluster_book_series_entity, lhead=book_is_part_of_book_series_relationship ]
        book_is_part_of_book_series -- book[ ltail=book_is_part_of_book_series_relationship, lhead=cluster_book_entity ]
        }
        
    /* Group all the Director to Movie Relationships */
    subgraph director_to_movie
        {
        director -- director_directs_movie[ ltail=cluster_director_entity, lhead=director_directs_movie_relationship ]
        director_directs_movie -- movie[ ltail=director_directs_movie_relationship, lhead=cluster_movie_entity ]
        }
        
    /* Group all the Studio to Movie Relationships */
    subgraph studio_to_movie
        {
        studio -- studio_releases_movie[ ltail=cluster_studio_entity, lhead=studio_releases_movie_relationship ]
        studio_releases_movie -- movie[ ltail=studio_releases_movie_relationship, lhead=cluster_movie_entity ]
        }
    }
```
