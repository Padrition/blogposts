This is an Axum web application that uses Axum for backend, Postgres as a database and nginx to server web content.

For running this application you will need to have Docker and Docker compose.

To run:
```
ducker compose up -d --build
```

This will create and image for Axum application and pull needed images for nginx and Postgres.

After the building phase is finished you can visit `localhost` or `localhost/home`.
There you can add a new blog post.

Data in the containers is persisted.

Fill free to contact me via my email `padrition@gmail.com`