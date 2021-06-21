type UserName = String;
type Id = i64;
type Timestamp = i64;
type User = (Id, UserName, Timestamp);

fn main() {
    let id = 400;
    let now = 123456789;
    //let user = new_user(String::from("hoge"), id, now);
    let user = new_user(String::from("hoge"), now, id); // エラーにならない

    println!("{:?}", user);
}

fn new_user(name: UserName, id: Id, created: Timestamp) -> User {
    (id, name, created)
}
