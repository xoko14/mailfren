use barrel::{types, Migration};
use barrel::backend::Sqlite;

pub fn migration() -> String{
    let mut m = Migration::new();

    println!("Running {}", file!());

    m.create_table("fren", |t| {
        t.add_column("id", types::integer().primary(true).increments(true));
        t.add_column("code", types::varchar(255).nullable(false));
        t.add_column("description", types::text().nullable(false));
        t.add_column("custom_image", types::text().nullable(true));
    });

    m.create_table("greeting", |t| {
        t.add_column("id", types::integer().primary(true).increments(true));
        t.add_column("date", types::datetime().nullable(false));
        t.add_column("ip", types::text().nullable(false));
        t.add_column("headers", types::text().nullable(false));
        t.add_column("fren_id", types::foreign(
            "fren",
            "id",
            types::ReferentialAction::Unset,
            types::ReferentialAction::Unset,

        ));
    });

    m.make::<Sqlite>()
}