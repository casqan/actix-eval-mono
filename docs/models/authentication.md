# Authentication

Authentication is purely handled using jwts (JSON Web Tokens). The jwt is generated when a user logs in and is sent back to the client. The client then sends the jwt in the header of every request to the server. The server then verifies the jwt and if it is valid, the user is authenticated.

For this project, this is handled by Supabase. Supabase is an open-source Firebase alternative. It provides a set of tools to build your app faster. It provides authentication, realtime, and storage services.