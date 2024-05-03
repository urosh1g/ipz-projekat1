CREATE EXTENSION IF NOT EXISTS "uuid-ossp";
BEGIN;
CREATE TABLE IF NOT EXISTS countries (
    country_id uuid PRIMARY KEY DEFAULT uuid_generate_v4(),
    name varchar unique
);
CREATE TABLE IF NOT EXISTS directors (
    director_id uuid PRIMARY KEY DEFAULT uuid_generate_v4(),
    name varchar unique
);
CREATE TABLE IF NOT EXISTS writers (
    writer_id uuid PRIMARY KEY DEFAULT uuid_generate_v4(),
    name varchar unique
);
CREATE TABLE IF NOT EXISTS stars (
    star_id uuid PRIMARY KEY DEFAULT uuid_generate_v4(),
    name varchar unique
);
CREATE TABLE IF NOT EXISTS genres (
    genre_id uuid PRIMARY KEY DEFAULT uuid_generate_v4(),
    name varchar unique
);
CREATE TABLE IF NOT EXISTS countries (
    country_id uuid PRIMARY KEY DEFAULT uuid_generate_v4(),
    name varchar unique
);
CREATE TABLE IF NOT EXISTS companies (
    company_id uuid PRIMARY KEY DEFAULT uuid_generate_v4(),
    name varchar unique
);
CREATE TABLE IF NOT EXISTS movies (
    movie_id uuid PRIMARY KEY DEFAULT uuid_generate_v4(),
    name varchar,
    rating varchar,
    year integer,
    release_date date,
    score real,
    votes real,
    budget real,
    gross real,
    runtime real,
    director_id uuid,
    writer_id uuid,
    star_id uuid,
    country_id uuid,
    company_id uuid,
    genre_id uuid,
    FOREIGN KEY (director_id) REFERENCES directors(director_id),
    FOREIGN KEY (writer_id) REFERENCES writers(writer_id),
    FOREIGN KEY (star_id) REFERENCES stars(star_id),
    FOREIGN KEY (country_id) REFERENCES countries(country_id),
    FOREIGN KEY (company_id) REFERENCES companies(company_id),
    FOREIGN KEY (genre_id) REFERENCES genres(genre_id)
);
COMMIT;
