Today this is not meant to be a new WordPress or Drupal, but who knows?

The main idea is to have a blog in RUST.

To complete it I'll split this challenge in some parts
 - [ ] Create schema draw.io file
 - [ ] Create home page (nothing more than a blank page neede)
 - [ ] Create register page:
       - Must to require username, email and password
       - Email must be confirmed to login
       - A default user can only comment in posts
       - Obviusly the password needs to be hashed in the DB
 - [ ] Create login page:
       - The user should be able to login using username or email and password
 - [ ] Create reset password page:
       - Should be a link avail on the login page to reset the password
 - [ ] Create a profile page where the user can change it's own password and email (that must be confirmed)

 - [ ] Create a post interface where admin users can make posts:
       - Only title and body is required at first (in the beggining we just need to care about the content)
 - [ ] In the home page implement the list blog post function to list all past 10 posts paginated.

 - [ ] Create the post page, where you can read the post.

 - [ ] Create a comment box that anyone logged in will be able to comment, list all comments ordered by date.
 - [ ] Create a functionality to ban users to interact with posts (only admins can have)
