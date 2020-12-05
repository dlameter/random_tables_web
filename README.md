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
PUT /account/name/{string} - update account with given name
DELETE /account/id/{number} - delete account with given id
DELETE /account/name/{string} - delete account with given name
```
