#[macro_use]
extern crate rocket;
use rocket::State;

#[get("/<key>/<value>")]
fn put(key: String, value: String, db: State<sled::Db>) -> String {
    let k: Vec<u8> = bincode::serialize(&key).unwrap();
    let v: Vec<u8> = bincode::serialize(&value).unwrap();
    db.insert(k, v).expect("could not save");
    format!("{}: {}", key, value)
}

#[get("/")]
fn all(db: State<sled::Db>) -> String {
    let mut r = db.scan_prefix(&[]);
    let mut output = vec![];

    while let Some(Ok((key, val))) = r.next() {
        let key: String = bincode::deserialize(&key[..]).unwrap();
        let val: String = bincode::deserialize(&val[..]).unwrap();

        output.push(format!("{}: {}", key, val));
    }

    output.join("\n")
}

#[launch]
fn rocket() -> rocket::Rocket {
    let db: sled::Db = sled::open("my_db").unwrap();
    rocket::ignite()
        .manage(db)
        .mount("/", routes![all])
        .mount("/put", routes![put])
}
