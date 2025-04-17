# Notes
To run this with the cycle servers, use the following commands

## On Terminal 1
`ssh -N -L 3307:mysql.eecs.ku.edu:3306 [ku_username]@cycle3.eecs.ku.edu`

## On Terminal 2 (For confirmation)
`mysql -h 127.0.0.1 -P 3307 -u [database_username] -p`

## For Env File
`DATABASE_URL=mysql://[database_username]:[database_password]@localhost:3307/[database_username]`