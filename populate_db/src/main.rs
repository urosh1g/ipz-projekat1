use sqlx::PgPool;
use std::fs;
mod macros;

generate_upsert_functions!(
    (upsert_director, directors, director_id),
    (upsert_writer, writers, writer_id),
    (upsert_star, stars, star_id),
    (upsert_country, countries, country_id),
    (upsert_company, companies, company_id),
    (upsert_genre, genres, genre_id)
);

#[tokio::main]
async fn main() -> Result<(), std::io::Error> {
    let db_connection = PgPool::connect("postgres://postgres:postgres@localhost:5432/movies")
        .await
        .expect("Should be able to connect to db");
    let movies = parse_movies("./movies.csv")?;
    populate_db(&db_connection, &movies).await;
    Ok(())
}

#[derive(Default, Debug)]
struct Movie {
    name: String,
    rating: String,
    genre: String,
    year: i32,
    released: String,
    score: f32,
    votes: f32,
    director: String,
    writer: String,
    star: String,
    country: String,
    budget: f32,
    gross: f32,
    company: String,
    runtime: f32,
}

impl TryFrom<&[&str]> for Movie {
    type Error = String;
    fn try_from(value: &[&str]) -> Result<Self, Self::Error> {
        if value.len() <= 15 {
            return Err("Unmatched number of params".to_owned());
        }
        Ok(Movie {
            name: value[0].to_owned(),
            rating: value[1].to_owned(),
            genre: value[2].to_owned(),
            year: value[3].to_owned().parse().unwrap_or(0),
            released: value[4][1..].to_owned(),
            score: value[6].to_owned().parse().unwrap_or(0.0),
            votes: value[7].to_owned().parse().unwrap_or(0.0),
            director: value[8].to_owned(),
            writer: value[9].to_owned(),
            star: value[10].to_owned(),
            country: value[11].to_owned(),
            budget: value[12].to_owned().parse().unwrap_or(0.0),
            gross: value[13].to_owned().parse().unwrap_or(0.0),
            company: value[14].to_owned(),
            runtime: value[15].to_owned().parse().unwrap_or(0.0),
        })
    }
}

fn parse_movies(path: &str) -> Result<Vec<Movie>, std::io::Error> {
    let movies_data = fs::read_to_string(path)?;
    let mut movies: Vec<Movie> = Vec::with_capacity(movies_data.lines().count());
    let mut err_count = 0;
    for line in movies_data.lines().skip(1) {
        let params: Vec<_> = line.split(',').collect();
        match Movie::try_from(&params[..]) {
            Ok(movie) => movies.push(movie),
            Err(_) => {
                err_count += 1;
                continue;
            }
        }
    }
    println!("Parsed: {len}\nFailed: {err_count}", len = movies.len());
    Ok(movies)
}

async fn insert_movie(executor: &PgPool, movie: &Movie) {
    let date = format!("{} {}", movie.released, movie.year);
    let release_date = chrono::NaiveDate::parse_from_str(&date, "%B %d %Y")
        .expect("Should be able to parse the date");
    let star_id = upsert_star(executor, &movie.star).await;
    let genre_id = upsert_genre(executor, &movie.genre).await;
    let writer_id = upsert_writer(executor, &movie.writer).await;
    let company_id = upsert_company(executor, &movie.company).await;
    let country_id = upsert_country(executor, &movie.country).await;
    let director_id = upsert_director(executor, &movie.director).await;
    sqlx::query!(
        r#"
            INSERT INTO movies (
                name, rating, year, release_date, score, votes, budget, gross, runtime, 
                director_id, writer_id, star_id, country_id, company_id, genre_id
            )
            VALUES (
                $1, $2, $3, $4, $5, $6, $7, $8, $9, $10, $11, $12, $13, $14, $15
            );
        "#,
        &movie.name,
        &movie.rating,
        &movie.year,
        release_date,
        &movie.score,
        &movie.votes,
        &movie.budget,
        &movie.gross,
        &movie.runtime,
        director_id,
        writer_id,
        star_id,
        country_id,
        company_id,
        genre_id
    )
    .execute(executor)
    .await
    .expect("Should be able to insert a movie");
}

async fn populate_db(executor: &PgPool, movies: &[Movie]) {
    for movie in movies {
        insert_movie(executor, movie).await;
    }
}
