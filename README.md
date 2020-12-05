# Random Tables
This web application will allow users to store, share, and roll on random tables.

## API Reference
The api looks like the following:
```
// Account api
POST /account - create account
GET /account/id/{number} - read account with given id
GET /account/name/{string} - read account with given name
PUT /account/id/{number} - update account with given id
DELETE /account/id/{number} - delete account with given id

// Table api
POST /table - create table
GET /table/{number} - read table with given id
PUT /table/{number} - update table with given id
DELETE /table/{number} - delete table with given id
```
