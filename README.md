# Random Tables
This web application will allow users to store, share, and roll on random tables.

## API Reference
The api looks like the following:
```
// Auth api
POST /signup - receives json to create a new account
POST /login - receives logon info as json and returns account info as well as sets a cookie on user browser
POST /logout - receives cookie from browser and ends user's current session
GET /whois - takes cookie information and returns account info for logged in user

// Account api
GET /account/id/{number} - read account with given id
PUT /account/id/{number} - update account with given id
DELETE /account/id/{number} - delete account with given id

// Table api
POST /table - create table
GET /table/{number} - read table with given id
PUT /table/{number} - update table with given id
DELETE /table/{number} - delete table with given id
GET /table/{number}/roll - get a random element from the table
```
