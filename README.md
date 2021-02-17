Diesel multi-join experiment
============================

After getting footguns in sequelize and gorm, just want to try out diesel and
state of diesel. Next https://github.com/pickfire/diesel-update-from

So far, a bit hard to get started as error message are a bit confusing, for
example you get "failed to resolve: use of undeclared crate or module
`new_users`" instead of missing `#[table_name = "users"]` if you removed that
line from `pub struct NewUser`.

Sometimes compile type errors are also a bit overwhelming. Especially when you
got into a case not supported by diesel, like keeping original type when
joining. I wish these are mentioned in the docs too, like what was not
supported but beginners might try it out when using it.

```
error[E0277]: the trait bound `(i32, String): Queryable<(diesel::sql_types::Integer, diesel::sql_types::Text, diesel::sql_types::Integer, diesel::sql_types::Integer), Sqlite>` is not satisfied
  --> src/main.rs:17:50
   |
17 |     let data: Vec<(Comment, Post, User)> = query.load(&conn).unwrap();
   |                                                  ^^^^ the trait `Queryable<(diesel::sql_types::Integer, diesel::sql_types::Text, diesel::sql_types::Integer, diesel::sql_types::Integer), Sqlite>` is not implemented for `(i32, String)`
   |
   = help: the following implementations were found:
             <(A, B) as Queryable<(SA, SB), __DB>>
   = note: required because of the requirements on the impl of `Queryable<(diesel::sql_types::Integer, diesel::sql_types::Text, diesel::sql_types::Integer, diesel::sql_types::Integer), Sqlite>` for `hello::models::Comment`
   = note: required because of the requirements on the impl of `Queryable<((diesel::sql_types::Integer, diesel::sql_types::Text, diesel::sql_types::Integer, diesel::sql_types::Integer), (diesel::sql_types::Integer, diesel::sql_types::Text, diesel::sql_types::Nullable<diesel::sql_types::Text>, diesel::sql_types::Integer), (diesel::sql_types::Integer, diesel::sql_types::Text)), Sqlite>` for `(hello::models::Comment, hello::models::Post, hello::models::User)`
   = note: required because of the requirements on the impl of `LoadQuery<SqliteConnection, (hello::models::Comment, hello::models::Post, hello::models::User)>` for `SelectStatement<JoinOn<diesel::query_source::joins::Join<JoinOn<diesel::query_source::joins::Join<hello::schema::comments::table, hello::schema::posts::table, Inner>, diesel::expression::operators::Eq<diesel::expression::nullable::Nullable<post_id>, diesel::expression::nullable::Nullable<hello::schema::posts::id>>>, hello::schema::users::table, Inner>, diesel::expression::operators::Eq<diesel::expression::nullable::Nullable<hello::schema::comments::user_id>, diesel::expression::nullable::Nullable<hello::schema::users::id>>>>`
```

Docs examples are also hard to get by at first. For example, how to establish a
new connection, I looked into `SqliteConnection` but did not noticed that it is
covered within `Connection` trait implemented by `SqliteConnection`, maybe
`diesel::connection` could mention it?

But until then, I never experienced any weird behaviors like incorrect
behavior, still getting the experience of "when it compiles, it works" but note
that it may take some effort for that, once someone did a few tries in diesel I
think it will be easier.

I think the type-safe effort is still worth it, rather than figure out why the
type magic does not work, now just need to get it compile and it works.

The example I tested out here is to have 3 struct, and have multiple inner
joins. I use sqlite for this for easy testing. Even though it works but I wish
it have better ergonomics when dealing with joins.  Like being able to just
extract original struct instead of each field.

    +---------+     +---------+     +---------+
    | User    |<-+  | Post    |     | Comment |
    +---------+  |  +---------+     +---------+
    | id      |  |  | id      |     | id      |
    | name    |  |  | title   |     | body    |
    |         |  |  | body    |<----+ post_id |
    |         |  |  | user_id +--+--+ user_id |
    +---------+  |  +---------+  |  +---------+
                 +---------------+

I was just looking to get all comments commented by user John. The query,

```rust
let query = comments
    .inner_join(posts)
    .inner_join(users)
    .select((comments::columns::body, title, name))
    .filter(name.eq("John"));
```

Which results in the SQL query,

```sql
SELECT
  `comments`.`body`,
  `posts`.`title`,
  `users`.`name`
 FROM ((`comments`
INNER JOIN `posts`
   ON `comments`.`post_id` = `posts`.`id`)
INNER JOIN `users`
   ON `comments`.`user_id` = `users`.`id`)
WHERE `users`.`name` = ? -- binds: ["John"]
```

## Get started

Rust, diesel_cli (with `sqlite` feature) is required.

```
$ diesel migration run
$ cargo run --bin init  # populate database
$ cargo run --bin hello  # multi join query
```
