use tern_rs::user_management::domain::entities::user::User;
use uuid::Uuid;

#[test]
fn test_user_new() {
    let name = "Alice";
    let user = User::new(name);

    assert_eq!(user.name, "Alice");
    assert!(!user.id.is_nil());
}

#[test]
fn test_user_from_existing() {
    let id = Uuid::new_v4();
    let name = "Bob";
    let user = User::from_existing(id, name);

    assert_eq!(user.name, "Bob");
    assert_eq!(user.id, id);
}
