use diesel::prelude::*;

fn main() {
    use hello::schema::comments::dsl::*;
    use hello::schema::posts::dsl::*;
    use hello::schema::users::dsl::*;
    use hello::schema::comments;

    let conn = hello::establish_connection();

    let query = comments
        .inner_join(posts)
        .inner_join(users)
        .select((comments::columns::body, title, name))
        .filter(name.eq("John"));
    println!(
        "{}",
        diesel::debug_query::<diesel::sqlite::Sqlite, _>(&query).to_string()
    );
    let data: Vec<(String, String, String)> = query.load(&conn).unwrap();

    dbg!(data);
}
