# Database Output
## Entities
### Author
#### Description
```
+------------+---------+------+-----+---------+----------------+
| Field      | Type    | Null | Key | Default | Extra          |
+------------+---------+------+-----+---------+----------------+
| ID         | int(11) | NO   | PRI | NULL    | auto_increment |
| First_Name | text    | NO   |     | NULL    |                |
| Last_Name  | text    | NO   |     | NULL    |                |
+------------+---------+------+-----+---------+----------------+
```
#### Content
```
+----+------------+---------------+
| ID | First_Name | Last_Name     |
+----+------------+---------------+
|  1 | Bernardo   | Wiza          |
|  2 | Brady      | Gutkowski     |
|  3 | Ettie      | Spinka        |
|  4 | Mireille   | O'Reilly      |
|  5 | Zander     | Volkman       |
|  6 | Deshaun    | O'Hara        |
|  7 | Jaylan     | Emmerich      |
|  8 | Fleta      | Koss          |
|  9 | Alfred     | Klocko        |
| 10 | Fred       | Emmerich      |
| 11 | Amaya      | Koch          |
| 12 | Roxanne    | Casper        |
| 13 | Marisa     | Mitchell      |
| 14 | Darron     | Bruen         |
| 15 | Narciso    | Parisian      |
| 16 | Della      | Schuster      |
| 17 | Vincent    | Harber        |
| 18 | Candido    | Osinski       |
| 19 | Vincent    | Predovic      |
| 20 | Braden     | Gutkowski     |
| 21 | Reece      | Davis         |
| 22 | Antonia    | Smith         |
| 23 | Etha       | Kuhlman       |
| 24 | Waino      | Pfeffer       |
| 25 | Dean       | VonRueden     |
| 26 | Terry      | Goldner       |
| 27 | Oren       | Feeney        |
| 28 | Frankie    | Roberts       |
| 29 | Dayne      | Armstrong     |
| 30 | Gia        | Gibson        |
| 31 | Mark       | Ledner        |
| 32 | Tyshawn    | Hegmann       |
| 33 | Alec       | Becker        |
| 34 | Layne      | Beier         |
| 35 | Dudley     | Abernathy     |
| 36 | Gerda      | Marquardt     |
| 37 | Dayne      | Parisian      |
| 38 | Merl       | Rath          |
| 39 | Odie       | Beier         |
| 40 | Orland     | O'Connell     |
| 41 | Freida     | Roob          |
| 42 | Breanna    | Kling         |
| 43 | Tito       | Ferry         |
| 44 | Devon      | Streich       |
| 45 | Carroll    | Ritchie       |
| 46 | Tomasa     | Tromp         |
| 47 | Domingo    | Runolfsdottir |
| 48 | Brennan    | Bartell       |
| 49 | Rusty      | Hoeger        |
+----+------------+---------------+
```
### Book
#### Description
```
+-------+-------------+------+-----+---------+-------+
| Field | Type        | Null | Key | Default | Extra |
+-------+-------------+------+-----+---------+-------+
| ISBN  | varchar(20) | NO   | PRI | NULL    |       |
| Title | text        | NO   |     | NULL    |       |
+-------+-------------+------+-----+---------+-------+
```
#### Content
```
+-------------------+------------------------------------------------------------------------------------------------------------------------------------------------------+
| ISBN              | Title                                                                                                                                                |
+-------------------+------------------------------------------------------------------------------------------------------------------------------------------------------+
| 0-01-471197-4     | ex assumenda.                                                                                                                                        |
| 0-14-907210-4     | iure voluptate nobis enim tempora.                                                                                                                   |
| 0-298-72994-6     | aut.                                                                                                                                                 |
| 0-314-19944-6     | voluptas repellendus maxime consectetur ullam velit voluptatem temporibus provident.                                                                 |
| 0-347-56901-3     | libero reiciendis facere officia soluta harum magnam alias quis esse ipsum sit id.                                                                   |
| 0-496-63609-X     | eligendi nisi quia est ea consequatur.                                                                                                               |
| 0-505-63937-8     | quae aperiam placeat aliquid magni facere et explicabo enim rerum dolorem aliquid.                                                                   |
| 0-551-99401-0     | qui harum et sunt.                                                                                                                                   |
| 0-649-29188-3     | et rem laudantium aut officiis et ut aliquid et velit illum dolor quibusdam sit mollitia.                                                            |
| 0-654-38702-8     | laboriosam et impedit id exercitationem sed dolorem perspiciatis voluptatem natus animi molestiae vel dolores voluptatem distinctio sunt sint.       |
| 0-675-38841-4     | dolorum explicabo necessitatibus suscipit nulla qui odio mollitia delectus amet est odit ea nemo quasi rerum rerum natus aperiam.                    |
| 0-8412-3607-0     | soluta saepe nulla eaque ea.                                                                                                                         |
| 0-8438-7832-0     | consectetur possimus id nihil esse dolorum itaque deleniti voluptatem sed sed iste magnam ratione voluptatem enim ipsum.                             |
| 0-938424-86-6     | vel quo et dolore odio.                                                                                                                              |
| 0-9697154-3-9     | perferendis est quo reprehenderit.                                                                                                                   |
| 0-9726173-6-1     | quis magnam dolorem tenetur facere commodi non quod aut explicabo praesentium consequuntur ipsum eligendi itaque maxime aut nesciunt iusto.          |
| 0-9878606-5-8     | soluta in ea quisquam quod cum maiores eum illo et ut optio qui enim dolorum ut.                                                                     |
| 1-05-085601-5     | ducimus quibusdam unde necessitatibus ut ipsam minus.                                                                                                |
| 1-220-27561-1     | error quas commodi quisquam et quae numquam molestiae eveniet eum corrupti veniam alias quasi occaecati eius ipsam amet dicta.                       |
| 1-263-09547-X     | provident.                                                                                                                                           |
| 1-299-46704-0     | et quod aut excepturi rerum repudiandae quia magnam eius quod qui iste atque voluptatem nobis.                                                       |
| 1-331-95778-8     | consequatur neque aut magnam odit.                                                                                                                   |
| 1-360-87117-9     | id pariatur labore molestiae quia corrupti repellendus.                                                                                              |
| 1-372-47761-6     | cumque repellendus ad voluptates magni qui fugiat dolores voluptatem sed labore eveniet sed molestias enim eveniet laborum exercitationem est.       |
| 1-4778-9472-1     | facere assumenda qui sunt deserunt unde temporibus assumenda sit quia mollitia.                                                                      |
| 1-4951-6981-2     | labore est beatae fugit illo fugiat expedita qui impedit soluta exercitationem nulla illum deleniti iste rem nesciunt.                               |
| 1-5106-6450-5     | dolorem.                                                                                                                                             |
| 1-5151-7685-1     | eveniet officia culpa quia eligendi odio illum commodi consequatur optio facere dolorem.                                                             |
| 1-5309-8605-2     | quam vitae fugiat labore sit voluptatem natus ut occaecati ex porro excepturi praesentium sed.                                                       |
| 1-61572-427-3     | aut repudiandae et.                                                                                                                                  |
| 1-64729-932-2     | consequuntur beatae tempora et aperiam expedita possimus repudiandae ipsam vel.                                                                      |
| 1-72055-353-X     | ipsum dignissimos quia placeat dolores.                                                                                                              |
| 1-72367-006-5     | accusamus voluptas voluptas amet.                                                                                                                    |
| 1-77440-129-0     | praesentium et nam ut itaque qui est possimus accusamus necessitatibus est qui et ea.                                                                |
| 1-78661-754-4     | dolores assumenda.                                                                                                                                   |
| 1-80763-286-5     | dicta iste accusamus est odio facere aut nam.                                                                                                        |
| 1-80919-042-8     | doloremque nulla fugiat suscipit maxime quos dolorem odio.                                                                                           |
| 1-935324-90-X     | esse velit fuga nisi sint quis tenetur eos voluptas et praesentium a repudiandae nulla.                                                              |
| 1-947003-47-X     | recusandae modi quia officia harum ut rerum omnis dolores ut aspernatur sit qui non sint alias et odio est.                                          |
| 1-966584-72-5     | quia voluptates maiores voluptatibus qui deleniti fugit nobis voluptas magni quis voluptate rem odio.                                                |
| 978-0-06-677638-5 | quasi ut corporis ab quis et voluptatem reprehenderit qui cupiditate.                                                                                |
| 978-0-07-009456-7 | nesciunt debitis quia debitis accusantium delectus dolorem.                                                                                          |
| 978-0-09-023373-1 | omnis ea mollitia.                                                                                                                                   |
| 978-0-219-65345-7 | voluptas dignissimos nam.                                                                                                                            |
| 978-0-280-94938-1 | harum quae nostrum perspiciatis velit facilis.                                                                                                       |
| 978-0-323-88167-8 | eum quam et harum perferendis ut sed explicabo ex eos nisi velit quia.                                                                               |
| 978-0-345-57469-5 | est et beatae illo et illo possimus voluptate aliquid ut in quaerat sint aperiam deleniti unde.                                                      |
| 978-0-377-00384-2 | tempore aut qui provident impedit quia iusto maiores quis itaque quam nesciunt ipsam perferendis sunt rem.                                           |
| 978-0-434-23996-2 | animi impedit rerum quia amet dicta nostrum recusandae cupiditate saepe.                                                                             |
| 978-0-435-43469-4 | consequatur consequuntur quia qui nostrum iure.                                                                                                      |
| 978-0-480-83447-0 | nulla perspiciatis optio qui iure accusamus nulla velit provident explicabo eligendi explicabo delectus voluptas pariatur reprehenderit.             |
| 978-0-541-40934-0 | rerum illo est est aspernatur quas nostrum.                                                                                                          |
| 978-0-586-34040-0 | harum aut autem accusamus et voluptatem nesciunt eius omnis autem delectus.                                                                          |
| 978-0-611-54137-6 | ut non reprehenderit molestiae in rerum.                                                                                                             |
| 978-0-638-23282-0 | soluta delectus qui odit iste minus voluptatem doloremque occaecati magnam.                                                                          |
| 978-0-640-63652-4 | quia nam qui sequi minus aut qui cumque in animi adipisci rerum delectus asperiores doloremque laborum quibusdam qui aperiam.                        |
| 978-0-696-33945-5 | quod tempore ullam rem quasi sapiente.                                                                                                               |
| 978-0-7050-4059-4 | et voluptas maxime.                                                                                                                                  |
| 978-0-7277-4317-6 | error eum veniam enim doloribus facere dolorum corrupti et quas quam quasi nobis possimus ullam debitis qui impedit enim.                            |
| 978-0-7605-0675-2 | fugit ipsum id.                                                                                                                                      |
| 978-0-8023-9013-4 | numquam quo qui sit.                                                                                                                                 |
| 978-0-8285-4502-0 | repellendus eligendi.                                                                                                                                |
| 978-0-8418-9540-9 | blanditiis voluptates odit et neque neque maiores ea iusto est dolores repellendus at dolor et.                                                      |
| 978-0-8466-4313-5 | voluptatem vel voluptas magnam non odio iusto quos id in.                                                                                            |
| 978-0-86211-818-1 | vitae.                                                                                                                                               |
| 978-0-87297-552-1 | vitae harum sed at consectetur ratione accusamus nihil minima incidunt quia placeat illum labore corporis natus nulla.                               |
| 978-0-89081-240-2 | id cum laborum porro.                                                                                                                                |
| 978-0-904609-01-1 | et nobis commodi iusto fugit voluptas blanditiis doloribus numquam.                                                                                  |
| 978-0-9804213-4-7 | eum omnis voluptas sunt porro reprehenderit quia dolores iusto at et veritatis est ut rerum dolorem molestiae numquam ullam.                         |
| 978-0-9919647-3-4 | amet recusandae quas voluptatem.                                                                                                                     |
| 978-1--11904656-1 | ipsum nam aut optio magnam recusandae asperiores incidunt sed et eius.                                                                               |
| 978-1--12587362-5 | doloremque sint autem minus ut consequuntur aliquid hic praesentium quasi voluptatem mollitia veritatis quam consequatur nesciunt quae rerum labore. |
| 978-1--13239474-8 | natus sunt.                                                                                                                                          |
| 978-1--14203497-9 | et blanditiis iusto placeat soluta officia fugit quas est sint mollitia tempore perspiciatis vitae inventore voluptatem.                             |
| 978-1--18263299-9 | quidem voluptas necessitatibus sed amet hic voluptatibus eum dolore et et magnam harum sunt non.                                                     |
| 978-1-05-867724-6 | quia libero omnis officiis aut distinctio qui consequuntur repellendus deleniti repellendus.                                                         |
| 978-1-05-925385-2 | ut facere delectus.                                                                                                                                  |
| 978-1-06-212878-9 | sit porro saepe eos qui omnis aspernatur.                                                                                                            |
| 978-1-08-411225-4 | quia sunt quas.                                                                                                                                      |
| 978-1-320-56609-4 | doloremque dolorum in accusantium modi expedita qui amet vitae eveniet explicabo ut numquam est quisquam eum sint.                                   |
| 978-1-330-99133-6 | vel praesentium aut quis neque natus.                                                                                                                |
| 978-1-333-17082-0 | alias nam dolores praesentium asperiores molestiae perspiciatis voluptatem accusantium dolor et.                                                     |
| 978-1-4009-1379-4 | reprehenderit voluptatum velit dolor numquam et magnam aut in perferendis at voluptatibus.                                                           |
| 978-1-4400-4430-4 | distinctio suscipit laborum.                                                                                                                         |
| 978-1-4560-6895-0 | quia animi placeat voluptates fugit dolorum sed molestiae explicabo consequuntur ut sint ratione.                                                    |
| 978-1-4757-2864-4 | tempore est tenetur expedita ea nihil ut quasi corrupti veniam ipsam fugit itaque accusamus dignissimos perferendis cupiditate fugiat voluptate.     |
| 978-1-4860-5511-1 | incidunt cumque cumque dicta qui nostrum assumenda expedita eos recusandae.                                                                          |
| 978-1-5005-6548-0 | commodi ut sapiente quos culpa et eius incidunt pariatur sed aut quia aut eius.                                                                      |
| 978-1-62307-486-9 | omnis quasi natus ipsam autem sed suscipit sint quis nam quo sapiente sunt et aliquid nobis et.                                                      |
| 978-1-62408-449-2 | dignissimos.                                                                                                                                         |
| 978-1-65413-256-9 | dolor eligendi recusandae reiciendis.                                                                                                                |
| 978-1-68115-487-9 | enim error ut laboriosam nesciunt sed corrupti non nostrum in veritatis est veniam quis accusantium corrupti.                                        |
| 978-1-77471-150-7 | cumque repellendus provident molestias expedita qui et dolores voluptas iure repellendus qui quo sed possimus eius laboriosam.                       |
| 978-1-79593-932-4 | suscipit consequatur sunt reiciendis sint et facilis voluptates vitae recusandae autem possimus et.                                                  |
| 978-1-83190-273-2 | doloremque ut qui.                                                                                                                                   |
| 978-1-86720-939-3 | dolorem neque rerum non labore.                                                                                                                      |
| 978-1-901806-54-0 | sit minus nisi ut.                                                                                                                                   |
| 978-1-902975-11-5 | ratione qui possimus nostrum quis voluptatem ut voluptatibus.                                                                                        |
| 978-1-968768-95-9 | sit commodi tempora praesentium aspernatur minima eveniet ut aut occaecati dolor sed ab quia magni recusandae voluptatem.                            |
+-------------------+------------------------------------------------------------------------------------------------------------------------------------------------------+
```
### Book Copy
#### Description
```
CREATE TABLE IF NOT EXISTS book_copy 
  (
  ID INT NOT NULL,
  PRIMARY KEY (ID),
  FOREIGN KEY (ID) REFERENCES material(ID) ON DELETE CASCADE
  )
```
#### Content
```
+-----+
| ID  |
+-----+
|   1 |
|   3 |
|   4 |
|   6 |
|   8 |
|  10 |
|  12 |
|  13 |
|  15 |
|  16 |
|  19 |
|  22 |
|  23 |
|  27 |
|  33 |
|  35 |
|  36 |
|  39 |
|  40 |
|  42 |
|  43 |
|  44 |
|  46 |
|  47 |
|  50 |
|  53 |
|  54 |
|  55 |
|  56 |
|  57 |
|  59 |
|  60 |
|  62 |
|  67 |
|  69 |
|  71 |
|  72 |
|  73 |
|  74 |
|  75 |
|  76 |
|  81 |
|  82 |
|  83 |
|  85 |
|  88 |
|  90 |
|  95 |
|  96 |
|  97 |
|  99 |
| 100 |
| 101 |
| 103 |
| 104 |
| 105 |
| 106 |
| 107 |
| 109 |
| 115 |
| 119 |
| 120 |
| 121 |
| 122 |
| 127 |
| 128 |
| 132 |
| 133 |
| 134 |
| 136 |
| 138 |
| 139 |
| 142 |
| 143 |
| 144 |
| 145 |
| 146 |
| 149 |
| 150 |
| 152 |
| 154 |
| 157 |
| 158 |
| 159 |
| 160 |
| 162 |
| 163 |
| 164 |
| 165 |
| 167 |
| 168 |
| 169 |
| 171 |
| 172 |
| 176 |
| 178 |
| 180 |
| 181 |
| 182 |
| 183 |
| 184 |
| 185 |
| 186 |
| 188 |
| 191 |
| 192 |
| 193 |
| 194 |
| 195 |
| 198 |
| 200 |
| 201 |
| 202 |
| 204 |
| 205 |
| 208 |
| 210 |
| 211 |
| 212 |
| 215 |
| 217 |
| 218 |
| 219 |
| 220 |
| 224 |
| 225 |
| 226 |
| 228 |
| 229 |
| 231 |
| 232 |
| 237 |
| 240 |
| 242 |
| 243 |
| 248 |
| 249 |
| 250 |
| 251 |
| 252 |
| 253 |
| 255 |
| 256 |
| 258 |
| 259 |
| 260 |
| 261 |
| 267 |
| 268 |
| 272 |
| 273 |
| 274 |
| 275 |
| 279 |
| 281 |
| 284 |
| 285 |
| 287 |
| 288 |
| 289 |
| 290 |
| 294 |
| 295 |
| 297 |
| 299 |
| 307 |
| 309 |
| 313 |
| 320 |
| 321 |
| 324 |
| 325 |
| 326 |
| 327 |
| 331 |
| 332 |
| 333 |
| 334 |
| 335 |
| 339 |
| 342 |
| 343 |
| 344 |
| 345 |
| 346 |
| 348 |
| 349 |
| 350 |
| 355 |
| 358 |
| 359 |
| 360 |
| 365 |
| 366 |
| 367 |
| 368 |
| 370 |
| 371 |
| 373 |
| 374 |
| 375 |
| 376 |
| 377 |
| 381 |
| 382 |
| 383 |
| 384 |
| 385 |
| 387 |
| 388 |
| 390 |
| 391 |
| 392 |
| 393 |
| 395 |
| 397 |
| 400 |
| 403 |
| 406 |
| 408 |
| 409 |
| 410 |
| 413 |
| 414 |
| 417 |
| 419 |
| 424 |
| 426 |
| 429 |
| 430 |
| 437 |
| 439 |
| 445 |
| 447 |
| 448 |
| 451 |
| 453 |
| 454 |
| 456 |
| 458 |
| 459 |
| 461 |
| 463 |
| 464 |
| 467 |
| 471 |
| 472 |
| 474 |
| 475 |
| 476 |
| 477 |
| 478 |
| 479 |
| 483 |
| 484 |
| 485 |
| 488 |
| 491 |
| 495 |
| 496 |
| 497 |
| 499 |
| 500 |
| 502 |
| 503 |
| 504 |
| 506 |
| 507 |
| 508 |
| 510 |
| 511 |
| 514 |
| 519 |
| 526 |
| 529 |
| 532 |
| 535 |
| 537 |
| 538 |
| 540 |
| 542 |
| 543 |
| 545 |
| 547 |
| 549 |
| 550 |
| 551 |
| 553 |
| 554 |
| 555 |
| 556 |
| 557 |
| 559 |
| 560 |
| 562 |
| 563 |
| 566 |
| 568 |
| 570 |
| 571 |
| 572 |
| 579 |
| 580 |
| 582 |
| 583 |
| 584 |
| 592 |
| 593 |
| 594 |
| 595 |
| 596 |
| 597 |
| 598 |
| 603 |
| 608 |
| 609 |
| 614 |
| 615 |
| 617 |
| 619 |
| 620 |
| 621 |
| 622 |
| 624 |
| 626 |
| 627 |
| 630 |
| 637 |
| 639 |
| 640 |
| 643 |
| 645 |
| 646 |
| 648 |
| 649 |
| 651 |
| 652 |
| 656 |
| 663 |
| 665 |
| 666 |
| 667 |
| 668 |
| 670 |
| 671 |
| 672 |
| 674 |
| 675 |
| 676 |
| 681 |
| 682 |
| 683 |
| 685 |
| 688 |
| 689 |
| 690 |
| 692 |
| 694 |
| 695 |
| 698 |
| 699 |
| 700 |
| 702 |
| 706 |
| 708 |
| 709 |
| 710 |
| 712 |
| 717 |
| 719 |
| 720 |
| 724 |
| 725 |
| 727 |
| 728 |
| 729 |
| 730 |
| 732 |
| 733 |
| 735 |
| 737 |
| 738 |
| 739 |
| 740 |
| 741 |
| 742 |
| 743 |
| 744 |
| 747 |
| 750 |
| 752 |
| 753 |
| 755 |
| 757 |
| 759 |
| 762 |
| 763 |
| 766 |
| 767 |
| 768 |
| 769 |
| 770 |
| 773 |
| 774 |
| 777 |
| 778 |
| 779 |
| 782 |
| 788 |
| 789 |
| 790 |
| 793 |
| 800 |
| 804 |
| 805 |
| 806 |
| 807 |
| 808 |
| 809 |
| 813 |
| 814 |
| 815 |
| 816 |
| 819 |
| 820 |
| 821 |
| 822 |
| 823 |
| 824 |
| 826 |
| 828 |
| 829 |
| 834 |
| 836 |
| 842 |
| 843 |
| 847 |
| 848 |
| 849 |
| 851 |
| 852 |
| 853 |
| 854 |
| 859 |
| 861 |
| 863 |
| 864 |
| 868 |
| 869 |
| 870 |
| 871 |
| 874 |
| 875 |
| 876 |
| 879 |
| 883 |
| 884 |
| 886 |
| 887 |
| 888 |
| 889 |
| 899 |
| 902 |
| 903 |
| 904 |
| 905 |
| 907 |
| 908 |
| 911 |
| 913 |
| 914 |
| 916 |
| 917 |
| 919 |
| 920 |
| 921 |
| 923 |
| 924 |
| 928 |
| 930 |
| 933 |
| 935 |
| 944 |
| 946 |
| 947 |
| 952 |
| 953 |
| 954 |
| 955 |
| 958 |
| 962 |
| 964 |
| 966 |
| 967 |
| 968 |
| 969 |
| 971 |
| 972 |
| 974 |
| 975 |
| 977 |
| 983 |
| 985 |
| 987 |
| 990 |
| 992 |
| 993 |
| 997 |
| 998 |
| 999 |
+-----+
```
### Book Series
#### Description
```
+-------+---------+------+-----+---------+----------------+
| Field | Type    | Null | Key | Default | Extra          |
+-------+---------+------+-----+---------+----------------+
| ID    | int(11) | NO   | PRI | NULL    | auto_increment |
| NAME  | text    | NO   |     | NULL    |                |
+-------+---------+------+-----+---------+----------------+
```
#### Content
```
+----+---------------------------------------------------------------------------------------------------------------------+
| ID | NAME                                                                                                                |
+----+---------------------------------------------------------------------------------------------------------------------+
|  1 | harum animi nostrum omnis quidem ut odit voluptate qui et sunt ut fuga aliquam facere dignissimos illo.             |
|  2 | consequatur omnis.                                                                                                  |
|  3 | sequi accusantium at veritatis quia harum unde nobis magni officia accusantium.                                     |
|  4 | fugit et aliquid eius aut architecto animi ipsa ea sunt.                                                            |
|  5 | laborum.                                                                                                            |
|  6 | unde veniam hic.                                                                                                    |
|  7 | ipsa in qui dolores omnis.                                                                                          |
|  8 | illum reiciendis eos ut pariatur cumque ipsam.                                                                      |
|  9 | eum.                                                                                                                |
| 10 | laudantium.                                                                                                         |
| 11 | sed dolor aspernatur deleniti voluptatum fuga.                                                                      |
| 12 | quia recusandae quod minima est ab enim sed deleniti.                                                               |
| 13 | et odio aut rem et architecto sapiente voluptatem earum quas optio dolore vitae qui non similique pariatur.         |
| 14 | error.                                                                                                              |
| 15 | officiis voluptate rerum error eligendi officiis accusamus voluptas in dolorum qui possimus veritatis corrupti sit. |
| 16 | suscipit libero eum et alias corrupti fuga.                                                                         |
| 17 | incidunt.                                                                                                           |
| 18 | voluptatibus qui quis eum mollitia ut quidem vitae aliquam ipsa.                                                    |
| 19 | qui in vel dolores accusamus voluptas vel officiis harum possimus magni voluptas esse.                              |
+----+---------------------------------------------------------------------------------------------------------------------+
```

### Community Event
#### Description
```
+------------+----------+------+-----+---------+----------------+
| Field      | Type     | Null | Key | Default | Extra          |
+------------+----------+------+-----+---------+----------------+
| ID         | int(11)  | NO   | PRI | NULL    | auto_increment |
| Start_Time | datetime | NO   |     | NULL    |                |
| End_Time   | datetime | NO   |     | NULL    |                |
| Longitude  | float    | NO   |     | NULL    |                |
| Latitude   | float    | NO   |     | NULL    |                |
+------------+----------+------+-----+---------+----------------+
```

#### Content
```
+----+---------------------+---------------------+-----------+----------+
| ID | Start_Time          | End_Time            | Longitude | Latitude |
+----+---------------------+---------------------+-----------+----------+
|  1 | 2024-12-08 07:54:00 | 2024-12-08 11:18:00 |    63.571 |  82.4708 |
|  2 | 2023-07-06 08:46:00 | 2023-07-07 01:18:00 |    123.88 |  87.7609 |
|  3 | 2024-12-06 01:03:00 | 2024-12-06 14:29:00 |   31.3696 |  66.9987 |
|  4 | 2023-11-11 06:54:00 | 2023-11-11 14:59:00 |   78.4102 |  46.0614 |
|  5 | 2023-12-24 07:03:00 | 2023-12-24 20:59:00 |    125.71 | -80.7764 |
|  6 | 2023-02-09 04:43:00 | 2023-02-09 15:16:00 |  -36.3685 |  50.7873 |
|  7 | 2025-01-02 09:53:00 | 2025-01-02 22:50:00 |  -155.076 | -66.7787 |
|  8 | 2024-07-08 10:22:00 | 2024-07-08 18:53:00 |   26.0607 | -33.5587 |
|  9 | 2023-12-29 02:37:00 | 2023-12-29 13:46:00 |   149.827 | -40.4836 |
+----+---------------------+---------------------+-----------+----------+
```

## Relationships
### Book Has
#### Description
```
+---------+-------------+------+-----+---------+-------+
| Field   | Type        | Null | Key | Default | Extra |
+---------+-------------+------+-----+---------+-------+
| Copy_ID | int(11)     | NO   | PRI | NULL    |       |
| Book_ID | varchar(20) | NO   | MUL | NULL    |       |
+---------+-------------+------+-----+---------+-------+
```
#### Content
```
+---------+-------------------+
| Copy_ID | Book_ID           |
+---------+-------------------+
|      10 | 0-01-471197-4     |
|     542 | 0-01-471197-4     |
|     962 | 0-01-471197-4     |
|      13 | 0-14-907210-4     |
|     463 | 0-14-907210-4     |
|     689 | 0-14-907210-4     |
|     172 | 0-298-72994-6     |
|     426 | 0-298-72994-6     |
|       3 | 0-314-19944-6     |
|     334 | 0-314-19944-6     |
|     688 | 0-314-19944-6     |
|     168 | 0-347-56901-3     |
|     290 | 0-347-56901-3     |
|     474 | 0-347-56901-3     |
|     495 | 0-347-56901-3     |
|     700 | 0-347-56901-3     |
|     766 | 0-347-56901-3     |
|     777 | 0-347-56901-3     |
|     180 | 0-496-63609-X     |
|     243 | 0-496-63609-X     |
|     299 | 0-496-63609-X     |
|     385 | 0-496-63609-X     |
|      33 | 0-505-63937-8     |
|     188 | 0-505-63937-8     |
|     218 | 0-505-63937-8     |
|     219 | 0-505-63937-8     |
|     339 | 0-505-63937-8     |
|     553 | 0-505-63937-8     |
|     774 | 0-505-63937-8     |
|     889 | 0-505-63937-8     |
|     163 | 0-551-99401-0     |
|     240 | 0-551-99401-0     |
|     307 | 0-551-99401-0     |
|     327 | 0-551-99401-0     |
|     346 | 0-551-99401-0     |
|     694 | 0-551-99401-0     |
|     733 | 0-551-99401-0     |
|     158 | 0-649-29188-3     |
|     215 | 0-649-29188-3     |
|     289 | 0-649-29188-3     |
|     485 | 0-649-29188-3     |
|     806 | 0-649-29188-3     |
|     815 | 0-649-29188-3     |
|     968 | 0-649-29188-3     |
|      72 | 0-654-38702-8     |
|     414 | 0-654-38702-8     |
|     656 | 0-654-38702-8     |
|     868 | 0-654-38702-8     |
|     905 | 0-654-38702-8     |
|     100 | 0-675-38841-4     |
|     220 | 0-675-38841-4     |
|     478 | 0-675-38841-4     |
|     479 | 0-675-38841-4     |
|     500 | 0-675-38841-4     |
|     543 | 0-675-38841-4     |
|     990 | 0-675-38841-4     |
|     119 | 0-8412-3607-0     |
|     596 | 0-8412-3607-0     |
|     886 | 0-8412-3607-0     |
|     916 | 0-8412-3607-0     |
|     953 | 0-8412-3607-0     |
|     152 | 0-8438-7832-0     |
|     184 | 0-8438-7832-0     |
|     367 | 0-8438-7832-0     |
|     391 | 0-8438-7832-0     |
|     419 | 0-8438-7832-0     |
|     467 | 0-8438-7832-0     |
|     545 | 0-8438-7832-0     |
|     583 | 0-8438-7832-0     |
|     652 | 0-8438-7832-0     |
|     977 | 0-8438-7832-0     |
|      81 | 0-938424-86-6     |
|     273 | 0-938424-86-6     |
|     475 | 0-938424-86-6     |
|     614 | 0-938424-86-6     |
|     778 | 0-938424-86-6     |
|     853 | 0-938424-86-6     |
|     913 | 0-938424-86-6     |
|     101 | 0-9697154-3-9     |
|     408 | 0-9697154-3-9     |
|     671 | 0-9697154-3-9     |
|     724 | 0-9697154-3-9     |
|     955 | 0-9697154-3-9     |
|       1 | 0-9726173-6-1     |
|     326 | 0-9726173-6-1     |
|     615 | 0-9726173-6-1     |
|      75 | 0-9878606-5-8     |
|     488 | 0-9878606-5-8     |
|     670 | 0-9878606-5-8     |
|     993 | 0-9878606-5-8     |
|      54 | 1-05-085601-5     |
|     183 | 1-05-085601-5     |
|     250 | 1-05-085601-5     |
|     375 | 1-05-085601-5     |
|     725 | 1-05-085601-5     |
|     804 | 1-05-085601-5     |
|     871 | 1-05-085601-5     |
|     966 | 1-05-085601-5     |
|      43 | 1-220-27561-1     |
|     506 | 1-220-27561-1     |
|     507 | 1-220-27561-1     |
|     169 | 1-263-09547-X     |
|     510 | 1-263-09547-X     |
|     595 | 1-263-09547-X     |
|     637 | 1-263-09547-X     |
|     698 | 1-263-09547-X     |
|     823 | 1-263-09547-X     |
|     884 | 1-263-09547-X     |
|     998 | 1-263-09547-X     |
|      71 | 1-299-46704-0     |
|     260 | 1-299-46704-0     |
|     568 | 1-299-46704-0     |
|     676 | 1-299-46704-0     |
|     875 | 1-299-46704-0     |
|     954 | 1-299-46704-0     |
|     997 | 1-299-46704-0     |
|     122 | 1-331-95778-8     |
|     297 | 1-331-95778-8     |
|     335 | 1-331-95778-8     |
|     667 | 1-331-95778-8     |
|     682 | 1-331-95778-8     |
|     788 | 1-331-95778-8     |
|     805 | 1-331-95778-8     |
|     859 | 1-331-95778-8     |
|      36 | 1-360-87117-9     |
|     193 | 1-360-87117-9     |
|     349 | 1-360-87117-9     |
|     162 | 1-372-47761-6     |
|     146 | 1-4778-9472-1     |
|     186 | 1-4778-9472-1     |
|     272 | 1-4778-9472-1     |
|     447 | 1-4778-9472-1     |
|     627 | 1-4778-9472-1     |
|     692 | 1-4778-9472-1     |
|     719 | 1-4778-9472-1     |
|     911 | 1-4778-9472-1     |
|     917 | 1-4778-9472-1     |
|      56 | 1-4951-6981-2     |
|     365 | 1-4951-6981-2     |
|     382 | 1-4951-6981-2     |
|     563 | 1-4951-6981-2     |
|     672 | 1-4951-6981-2     |
|     975 | 1-4951-6981-2     |
|      19 | 1-5106-6450-5     |
|     200 | 1-5106-6450-5     |
|     409 | 1-5106-6450-5     |
|     699 | 1-5106-6450-5     |
|     763 | 1-5106-6450-5     |
|     808 | 1-5106-6450-5     |
|     902 | 1-5106-6450-5     |
|     933 | 1-5106-6450-5     |
|     127 | 1-5151-7685-1     |
|     477 | 1-5151-7685-1     |
|     603 | 1-5151-7685-1     |
|     728 | 1-5151-7685-1     |
|     770 | 1-5151-7685-1     |
|     819 | 1-5151-7685-1     |
|     921 | 1-5151-7685-1     |
|     958 | 1-5151-7685-1     |
|      90 | 1-5309-8605-2     |
|     559 | 1-5309-8605-2     |
|     144 | 1-61572-427-3     |
|     255 | 1-61572-427-3     |
|     410 | 1-61572-427-3     |
|     899 | 1-61572-427-3     |
|     157 | 1-64729-932-2     |
|     204 | 1-64729-932-2     |
|     333 | 1-64729-932-2     |
|     883 | 1-64729-932-2     |
|     904 | 1-64729-932-2     |
|       8 | 1-72055-353-X     |
|     226 | 1-72055-353-X     |
|     294 | 1-72055-353-X     |
|     320 | 1-72055-353-X     |
|     579 | 1-72055-353-X     |
|     923 | 1-72055-353-X     |
|     132 | 1-72367-006-5     |
|     231 | 1-72367-006-5     |
|     342 | 1-72367-006-5     |
|     453 | 1-72367-006-5     |
|     458 | 1-72367-006-5     |
|     550 | 1-72367-006-5     |
|     592 | 1-72367-006-5     |
|     769 | 1-72367-006-5     |
|     106 | 1-77440-129-0     |
|     562 | 1-77440-129-0     |
|     848 | 1-77440-129-0     |
|     854 | 1-77440-129-0     |
|      96 | 1-78661-754-4     |
|     584 | 1-78661-754-4     |
|     800 | 1-78661-754-4     |
|     964 | 1-78661-754-4     |
|     164 | 1-80763-286-5     |
|     195 | 1-80763-286-5     |
|     360 | 1-80763-286-5     |
|     437 | 1-80763-286-5     |
|     549 | 1-80763-286-5     |
|     580 | 1-80763-286-5     |
|     594 | 1-80763-286-5     |
|     779 | 1-80763-286-5     |
|     793 | 1-80763-286-5     |
|     874 | 1-80763-286-5     |
|     985 | 1-80763-286-5     |
|      99 | 1-80919-042-8     |
|     217 | 1-80919-042-8     |
|     366 | 1-80919-042-8     |
|     674 | 1-80919-042-8     |
|     741 | 1-80919-042-8     |
|     946 | 1-80919-042-8     |
|      44 | 1-935324-90-X     |
|     358 | 1-935324-90-X     |
|     376 | 1-935324-90-X     |
|     384 | 1-935324-90-X     |
|     390 | 1-935324-90-X     |
|     928 | 1-935324-90-X     |
|      73 | 1-947003-47-X     |
|     551 | 1-947003-47-X     |
|     730 | 1-947003-47-X     |
|     742 | 1-947003-47-X     |
|     178 | 1-966584-72-5     |
|     181 | 978-0-06-677638-5 |
|     406 | 978-0-06-677638-5 |
|     430 | 978-0-06-677638-5 |
|     828 | 978-0-06-677638-5 |
|     887 | 978-0-06-677638-5 |
|      35 | 978-0-07-009456-7 |
|     208 | 978-0-07-009456-7 |
|     388 | 978-0-07-009456-7 |
|     537 | 978-0-07-009456-7 |
|     759 | 978-0-07-009456-7 |
|     807 | 978-0-07-009456-7 |
|      23 | 978-0-09-023373-1 |
|     259 | 978-0-09-023373-1 |
|     514 | 978-0-09-023373-1 |
|     555 | 978-0-09-023373-1 |
|     107 | 978-0-219-65345-7 |
|      59 | 978-0-280-94938-1 |
|     251 | 978-0-280-94938-1 |
|     261 | 978-0-280-94938-1 |
|     359 | 978-0-280-94938-1 |
|     508 | 978-0-280-94938-1 |
|     720 | 978-0-280-94938-1 |
|     136 | 978-0-323-88167-8 |
|     344 | 978-0-323-88167-8 |
|     429 | 978-0-323-88167-8 |
|     511 | 978-0-323-88167-8 |
|     554 | 978-0-323-88167-8 |
|     888 | 978-0-323-88167-8 |
|     987 | 978-0-323-88167-8 |
|     171 | 978-0-345-57469-5 |
|     267 | 978-0-345-57469-5 |
|     345 | 978-0-345-57469-5 |
|     377 | 978-0-345-57469-5 |
|     572 | 978-0-345-57469-5 |
|      42 | 978-0-377-00384-2 |
|     229 | 978-0-377-00384-2 |
|     454 | 978-0-377-00384-2 |
|     503 | 978-0-377-00384-2 |
|     649 | 978-0-377-00384-2 |
|     732 | 978-0-377-00384-2 |
|     747 | 978-0-377-00384-2 |
|      67 | 978-0-434-23996-2 |
|     532 | 978-0-434-23996-2 |
|     753 | 978-0-434-23996-2 |
|     983 | 978-0-434-23996-2 |
|     999 | 978-0-434-23996-2 |
|      53 | 978-0-435-43469-4 |
|     343 | 978-0-435-43469-4 |
|     476 | 978-0-435-43469-4 |
|     496 | 978-0-435-43469-4 |
|     738 | 978-0-435-43469-4 |
|     969 | 978-0-435-43469-4 |
|     160 | 978-0-480-83447-0 |
|     268 | 978-0-480-83447-0 |
|     324 | 978-0-480-83447-0 |
|     497 | 978-0-480-83447-0 |
|     757 | 978-0-480-83447-0 |
|     944 | 978-0-480-83447-0 |
|      57 | 978-0-541-40934-0 |
|     281 | 978-0-541-40934-0 |
|     374 | 978-0-541-40934-0 |
|     393 | 978-0-541-40934-0 |
|     484 | 978-0-541-40934-0 |
|     744 | 978-0-541-40934-0 |
|     847 | 978-0-541-40934-0 |
|      50 | 978-0-586-34040-0 |
|     325 | 978-0-586-34040-0 |
|      95 | 978-0-611-54137-6 |
|     622 | 978-0-611-54137-6 |
|     750 | 978-0-611-54137-6 |
|     816 | 978-0-611-54137-6 |
|      69 | 978-0-638-23282-0 |
|     417 | 978-0-638-23282-0 |
|     535 | 978-0-638-23282-0 |
|     540 | 978-0-638-23282-0 |
|     571 | 978-0-638-23282-0 |
|     683 | 978-0-638-23282-0 |
|     727 | 978-0-638-23282-0 |
|      97 | 978-0-640-63652-4 |
|     249 | 978-0-640-63652-4 |
|     556 | 978-0-640-63652-4 |
|     597 | 978-0-640-63652-4 |
|     709 | 978-0-640-63652-4 |
|     849 | 978-0-640-63652-4 |
|      85 | 978-0-696-33945-5 |
|     624 | 978-0-696-33945-5 |
|     737 | 978-0-696-33945-5 |
|     768 | 978-0-696-33945-5 |
|     149 | 978-0-7050-4059-4 |
|     348 | 978-0-7050-4059-4 |
|     752 | 978-0-7050-4059-4 |
|      83 | 978-0-7277-4317-6 |
|     456 | 978-0-7277-4317-6 |
|     499 | 978-0-7277-4317-6 |
|     630 | 978-0-7277-4317-6 |
|      22 | 978-0-7605-0675-2 |
|     228 | 978-0-7605-0675-2 |
|     176 | 978-0-8023-9013-4 |
|     205 | 978-0-8023-9013-4 |
|     256 | 978-0-8023-9013-4 |
|     392 | 978-0-8023-9013-4 |
|     557 | 978-0-8023-9013-4 |
|     739 | 978-0-8023-9013-4 |
|     809 | 978-0-8023-9013-4 |
|     870 | 978-0-8023-9013-4 |
|     120 | 978-0-8285-4502-0 |
|     529 | 978-0-8285-4502-0 |
|     972 | 978-0-8285-4502-0 |
|     974 | 978-0-8285-4502-0 |
|      82 | 978-0-8418-9540-9 |
|     210 | 978-0-8418-9540-9 |
|     224 | 978-0-8418-9540-9 |
|     383 | 978-0-8418-9540-9 |
|     863 | 978-0-8418-9540-9 |
|     971 | 978-0-8418-9540-9 |
|     103 | 978-0-8466-4313-5 |
|     194 | 978-0-8466-4313-5 |
|     237 | 978-0-8466-4313-5 |
|     439 | 978-0-8466-4313-5 |
|     639 | 978-0-8466-4313-5 |
|     710 | 978-0-8466-4313-5 |
|     743 | 978-0-8466-4313-5 |
|     914 | 978-0-8466-4313-5 |
|     952 | 978-0-8466-4313-5 |
|     109 | 978-0-86211-818-1 |
|     321 | 978-0-86211-818-1 |
|     413 | 978-0-86211-818-1 |
|      76 | 978-0-87297-552-1 |
|     192 | 978-0-87297-552-1 |
|     279 | 978-0-87297-552-1 |
|     619 | 978-0-87297-552-1 |
|     651 | 978-0-87297-552-1 |
|     735 | 978-0-87297-552-1 |
|     834 | 978-0-87297-552-1 |
|     138 | 978-0-89081-240-2 |
|     820 | 978-0-89081-240-2 |
|     143 | 978-0-904609-01-1 |
|     646 | 978-0-904609-01-1 |
|     829 | 978-0-904609-01-1 |
|     142 | 978-0-9804213-4-7 |
|     198 | 978-0-9804213-4-7 |
|     284 | 978-0-9804213-4-7 |
|     472 | 978-0-9804213-4-7 |
|     483 | 978-0-9804213-4-7 |
|     504 | 978-0-9804213-4-7 |
|     626 | 978-0-9804213-4-7 |
|     790 | 978-0-9804213-4-7 |
|      74 | 978-0-9919647-3-4 |
|     309 | 978-0-9919647-3-4 |
|     822 | 978-0-9919647-3-4 |
|     836 | 978-0-9919647-3-4 |
|     154 | 978-1--11904656-1 |
|     274 | 978-1--11904656-1 |
|     350 | 978-1--11904656-1 |
|     403 | 978-1--11904656-1 |
|     445 | 978-1--11904656-1 |
|     570 | 978-1--11904656-1 |
|     640 | 978-1--11904656-1 |
|     648 | 978-1--11904656-1 |
|     762 | 978-1--11904656-1 |
|     864 | 978-1--11904656-1 |
|     924 | 978-1--11904656-1 |
|      39 | 978-1--12587362-5 |
|     133 | 978-1--13239474-8 |
|     225 | 978-1--13239474-8 |
|     560 | 978-1--13239474-8 |
|     663 | 978-1--13239474-8 |
|     121 | 978-1--14203497-9 |
|     253 | 978-1--14203497-9 |
|     285 | 978-1--14203497-9 |
|     313 | 978-1--14203497-9 |
|     395 | 978-1--14203497-9 |
|     582 | 978-1--14203497-9 |
|     621 | 978-1--14203497-9 |
|     645 | 978-1--14203497-9 |
|     755 | 978-1--14203497-9 |
|     773 | 978-1--14203497-9 |
|      12 | 978-1--18263299-9 |
|     685 | 978-1--18263299-9 |
|     708 | 978-1--18263299-9 |
|     903 | 978-1--18263299-9 |
|     128 | 978-1-05-867724-6 |
|     712 | 978-1-05-867724-6 |
|      62 | 978-1-05-925385-2 |
|     665 | 978-1-05-925385-2 |
|     702 | 978-1-05-925385-2 |
|      15 | 978-1-06-212878-9 |
|     695 | 978-1-06-212878-9 |
|     139 | 978-1-08-411225-4 |
|     287 | 978-1-08-411225-4 |
|     371 | 978-1-08-411225-4 |
|     608 | 978-1-08-411225-4 |
|     609 | 978-1-08-411225-4 |
|     767 | 978-1-08-411225-4 |
|     930 | 978-1-08-411225-4 |
|      47 | 978-1-320-56609-4 |
|     617 | 978-1-320-56609-4 |
|     907 | 978-1-320-56609-4 |
|     935 | 978-1-320-56609-4 |
|     182 | 978-1-330-99133-6 |
|     355 | 978-1-330-99133-6 |
|     620 | 978-1-330-99133-6 |
|     821 | 978-1-330-99133-6 |
|     842 | 978-1-330-99133-6 |
|     105 | 978-1-333-17082-0 |
|     538 | 978-1-333-17082-0 |
|     593 | 978-1-333-17082-0 |
|     643 | 978-1-333-17082-0 |
|     134 | 978-1-4009-1379-4 |
|     332 | 978-1-4009-1379-4 |
|     681 | 978-1-4009-1379-4 |
|     690 | 978-1-4009-1379-4 |
|     947 | 978-1-4009-1379-4 |
|     150 | 978-1-4400-4430-4 |
|     331 | 978-1-4400-4430-4 |
|     526 | 978-1-4400-4430-4 |
|     789 | 978-1-4400-4430-4 |
|     115 | 978-1-4560-6895-0 |
|     185 | 978-1-4560-6895-0 |
|     451 | 978-1-4560-6895-0 |
|     547 | 978-1-4560-6895-0 |
|     861 | 978-1-4560-6895-0 |
|     908 | 978-1-4560-6895-0 |
|     159 | 978-1-4757-2864-4 |
|     275 | 978-1-4757-2864-4 |
|     459 | 978-1-4757-2864-4 |
|     851 | 978-1-4757-2864-4 |
|     104 | 978-1-4860-5511-1 |
|     211 | 978-1-4860-5511-1 |
|     471 | 978-1-4860-5511-1 |
|     519 | 978-1-4860-5511-1 |
|     782 | 978-1-4860-5511-1 |
|     967 | 978-1-4860-5511-1 |
|      88 | 978-1-5005-6548-0 |
|     373 | 978-1-5005-6548-0 |
|     668 | 978-1-5005-6548-0 |
|     824 | 978-1-5005-6548-0 |
|      60 | 978-1-62307-486-9 |
|     400 | 978-1-62307-486-9 |
|     813 | 978-1-62307-486-9 |
|     167 | 978-1-62408-449-2 |
|     242 | 978-1-62408-449-2 |
|     248 | 978-1-62408-449-2 |
|     258 | 978-1-62408-449-2 |
|     370 | 978-1-62408-449-2 |
|     397 | 978-1-62408-449-2 |
|     566 | 978-1-62408-449-2 |
|     598 | 978-1-62408-449-2 |
|       4 | 978-1-65413-256-9 |
|     448 | 978-1-65413-256-9 |
|     706 | 978-1-65413-256-9 |
|     165 | 978-1-68115-487-9 |
|     424 | 978-1-68115-487-9 |
|     675 | 978-1-68115-487-9 |
|      16 | 978-1-77471-150-7 |
|     464 | 978-1-77471-150-7 |
|     491 | 978-1-77471-150-7 |
|     502 | 978-1-77471-150-7 |
|     843 | 978-1-77471-150-7 |
|     852 | 978-1-77471-150-7 |
|      27 | 978-1-79593-932-4 |
|     191 | 978-1-79593-932-4 |
|     232 | 978-1-79593-932-4 |
|     717 | 978-1-79593-932-4 |
|     869 | 978-1-79593-932-4 |
|      40 | 978-1-83190-273-2 |
|     252 | 978-1-83190-273-2 |
|     461 | 978-1-83190-273-2 |
|     729 | 978-1-83190-273-2 |
|     879 | 978-1-83190-273-2 |
|     992 | 978-1-83190-273-2 |
|     145 | 978-1-86720-939-3 |
|     201 | 978-1-86720-939-3 |
|     387 | 978-1-86720-939-3 |
|     666 | 978-1-86720-939-3 |
|     826 | 978-1-86720-939-3 |
|      55 | 978-1-901806-54-0 |
|     381 | 978-1-901806-54-0 |
|     919 | 978-1-901806-54-0 |
|       6 | 978-1-902975-11-5 |
|     202 | 978-1-902975-11-5 |
|     212 | 978-1-902975-11-5 |
|     368 | 978-1-902975-11-5 |
|     814 | 978-1-902975-11-5 |
|      46 | 978-1-968768-95-9 |
|     288 | 978-1-968768-95-9 |
|     295 | 978-1-968768-95-9 |
|     740 | 978-1-968768-95-9 |
|     876 | 978-1-968768-95-9 |
|     920 | 978-1-968768-95-9 |
+---------+-------------------+
```
