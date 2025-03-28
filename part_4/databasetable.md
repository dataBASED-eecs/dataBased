## Room
| Attribute Name        | Data Type | Description |
|-|-|-|
| Capacity | INT | The number of people the room has space for |
| Number | INT | The unique number of the room in the library |

## Reserves_Room
| Attribute Name        | Data Type | Description |
|-|-|-|
| Duration | TIME | The duration the room is reserved for |
| Date | DATETIME | The time the room reservation starts at |
| Room_ID | Number | The number of the room that is reserved. This is a foreign key into the Room table |
| Member_ID | INT | The member id that reserved the room. This is a foreign key into the Member table |

## Staff
| Attribute Name        | Data Type | Description |
|-|-|-|
| Salary | DECIMAL(6,2) | The yearly salary of the staff member in dolars |
| Work_Phone | VARCHAR(15) | The phone number of the staff member |
| Start_Date | DATE | The date the staff member started working for the library |
| Work_Email | TEXT | The work email of the staff member |
| Date_of_Birth         | DATE | The date the member was born |
| Member_ID             | INT | The unique id of the member. This is a foreign key into the Member table |
| Outstanding_Balance   | DECIMAL(6,2) | The amount the member owes to the library in dolars |
| Email | TEXT | The email of the member |
| First_Name | TEXT | The first name of the member |
| Last_Name | TEXT | The last name of the member |

## Organizes
| Attribute Name        | Data Type | Description |
|-|-|-|
| Staff_ID | INT | The ID of the staff member that organizes the event. This is a foreign key into the Staff table |
| Community_Event_ID | INT | The id of the community event that is organized. This is a foreign key into the Community Event table |

## Registers
| Attribute Name        | Data Type | Description |
|-|-|-|
| Member_ID | INT | The member id of a member that is registered for an event. This is a foreign key into the Staff table |
| Community_Event_ID | INT | The id of the community event that is organized. This is a foreign key into the Community Event table |

## Community_Event
| Attribute Name        | Data Type | Description |
|-|-|-|
| Latitude | FLOAT | The latitude of the location of the event |
| Longitude | FLOAT | The longitude of the location of the event |
| Start_Time | DATETIME | The time the event starts |
| End_Time | DATETIME | The time the event ends |
| Organizer_First_Name | TEXT | The first name of the event organizer |
| Organizer_Last_Name | TEXT | The last name of the event organizer |
| ID | INT | The unique id of the event |

## Member
| Attribute Name        | Data Type | Description |
|-|-|-|
| Date_of_Birth         | DATE | The date the member was born |
| Member_ID             | INT | The unique id of the member |
| Outstanding_Balence   | DECIMAL(6,2) | The amount the member owes to the library in dolars |
| Email | TEXT | The email of the member |
| First_Name | TEXT | The first name of the member |
| Last_Name | TEXT | The last name of the member |

## Loans
| Attribute Name        | Data Type | Description |
|-|-|-|
| Duration | TIME | The time the loan is approved for |
| Start_Date | DATETIME | The time the loan started |
| Member_ID | INT | The member id of the member that is loaning a material. This is a foreign key into the Member table |
| Material_ID | INT | The id of the material that is loaned. This is a foreign key into the Material table |

## Donates
| Attribute Name        | Data Type | Description |
|-|-|-|
| Member_ID | INT | The member id of the member that donated the material. This is a foreign key into the Member table |
| Material_ID | INT | The id of the material that was donated. This is a foreign key into the Material table |

## Material
| Attribute Name        | Data Type | Description |
|-|-|-|
| ID | INT | The unique id of the material |

## Reserves_Material
| Attribute Name        | Data Type | Description |
|-|-|-|
| Reservation_Date | DATETIME | The date the reservation is for |
| Member_ID | INT | The member id of the member that reserved the material. This is a foreign key into the Member table |
| Material_ID | INT | The id of the material that was reserved. This is a foreign key into the Material table |

## Author
| Attribute Name        | Data Type | Description |
|-|-|-|
| First_Name | TEXT | The first name of the author |
| Last_Name | TEXT | The second name of the author |
| ID | INT | The unique id of the author |

## Searches_Book
| Attribute Name        | Data Type | Description |
|-|-|-|
| Member_ID | INT | The member id of the member that searched for a book. This is a foreign key into the Member table |
| Book_ID | INT | The id of the book that was searched for. This is a foreign key into the Book table |

## Searches_Movie
| Attribute Name        | Data Type | Description |
|-|-|-|
| Member_ID | INT | The member id of the member that searched for a movie. This is a foreign key into the Member table |
| Movie_ID | INT | The id of the movie that was searched for. This is a foreign key into the Movie table |

## Book_Copy
| Attribute Name        | Data Type | Description |
|-|-|-|
| ID | INT | The unique id of the movie copy. This is a foreign key into the material table |

## Book_Has
| Attribute Name        | Data Type | Description |
|-|-|-|
| Copy_ID | INT | The id of the book instance which is a copy of the book. This is a foreign key into the Book Copy table |
| Book_ID | INT | The id of the book. This is a foreign key into the Book table |

## Writes
| Attribute Name        | Data Type | Description |
|-|-|-|
| Author_ID | INT | The id of the author that wrote a book. This is a foreign key into the Author table |
| Book_ID | INT | The id of the book that the author wrote. This is a foreign key into the Book table |

## Publishes
| Attribute Name        | Data Type | Description |
|-|-|-|
| Publisher_ID | INT | The id of the publisher that published a book. This is a foreign key into the Publisher table |
| Book_ID | INT | The id of the book that the publisher published. This is a foreign key into the Book table |

## Is_Part_Of
| Attribute Name        | Data Type | Description |
|-|-|-|
| Order | INT | The position of the item in the book series |
| Book_Series_ID | INT | The id of the book series a book is part of. This is a foreign key into the Book Series table |
| Book_ID | INT | The id of the book in the series. This is a foreign key into the Book table |

## Directs
| Attribute Name        | Data Type | Description |
|-|-|-|
| Director_ID | INT | The id of the director that directed the movie. This is a foreign key into the Director table |
| Movie_ID | INT | The id of the movie that director directed. This is a foreign key into the Movie table |

## Releases
| Attribute Name        | Data Type | Description |
|-|-|-|
| Studio_ID | INT | The id of the studio that released the movie. This is a foreign key into the Studio table |
| Movie_ID | INT | The id of the movie that studio released. This is a foreign key into the Movie table |

## Book
| Attribute Name        | Data Type | Description |
|-|-|-|
| ISBN | CHAR(13) | The unique ISBN id of the book |
| Title | TEXT | The title of the book |

## Publisher
| Attribute Name        | Data Type | Description |
|-|-|-|
| Name | TEXT | The name of the publishing organization |
| ID | INT | The unique id of the publisher |

## Book_Series
| Attribute Name        | Data Type | Description |
|-|-|-|
| Name | TEXT | The name of the book series |
| ID | INT | The unique id of the book series |

## Director
| Attribute Name        | Data Type | Description |
|-|-|-|
| First_Name | TEXT | The first name of the director |
| Last_Name | TEXT | The last name of the director |
| ID | INT | The unique id of the director |

## Movie_Copy
| Attribute Name        | Data Type | Description |
|-|-|-|
| ID | INT | The unique id of the movie copy. This is a foreign key into the material table |

## Movie_Has
| Attribute Name        | Data Type | Description |
|-|-|-|
| Copy_ID | INT | The id of the movie instance which is a copy of the movie. This is a foreign key into the Movie Copy table |
| Movie_ID | INT | The id of the movie. This is a foreign key into the Movie table |

## Movie
| Attribute Name        | Data Type | Description |
|-|-|-|
| ISAN | TEXT | The unique ISAN id of the movie |
| Release_Date | DATE | The date the movie was released |
| Title | TEXT | The title of the movie |

## Studio
| Attribute Name        | Data Type | Description |
|-|-|-|
| Name | TEXT | The name of the studio |
| ID | INT | The unique id of the studio |
