use rusqlite::Connection;

pub fn main() {
    mod embedded {
        refinery::embed_migrations!("./src/migrations");
    }

    let mut conn = Connection::open("./database.db")
        .unwrap();

    embedded::migrations::runner().run(&mut conn).unwrap();
}