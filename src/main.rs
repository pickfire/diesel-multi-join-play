use diesel::prelude::*;

fn main() {
    use hello::schema::{comments, posts, users};

    let conn = hello::establish_connection();

    let query = comments::table
        .inner_join(posts::table)
        .inner_join(users::table)
        .select((comments::body, posts::title, users::name))
        .filter(users::name.eq("John"));
    println!(
        "{}",
        diesel::debug_query::<diesel::sqlite::Sqlite, _>(&query).to_string()
    );
    let data: Vec<(String, String, String)> = query.load(&conn).unwrap();

    dbg!(data);
}
