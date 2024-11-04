-- Your SQL goes here
CREATE TABLE "blogpost"(
	"id" SERIAL NOT NULL PRIMARY KEY,
	"text" VARCHAR NOT NULL,
	"publication_date" DATE NOT NULL,
	"post_image_path" VARCHAR,
	"username" VARCHAR NOT NULL,
	"avatar_path" VARCHAR
);

