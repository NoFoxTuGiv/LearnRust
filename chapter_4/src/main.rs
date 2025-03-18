fn main() {
    // Structs
    // 1. Definition: Structs are defined using the `struct` keyword.
    struct User {
        username: String,
        email: String,
        sign_in_count: u64,
        active: bool,
    }

    // 2. Instantiation: Create an instance of a struct.
    let user1 = User {
        username: String::from("example_user"),
        email: String::from("user@example.com"),
        sign_in_count: 1,
        active: true,
    };

    // 3. Field Access: Access fields using dot notation.
    println!("Username: {}", user1.username);

    // 4. Mutability: Entire instance must be mutable to modify fields.
    let mut user2 = User {
        username: String::from("another_user"),
        email: String::from("another@example.com"),
        sign_in_count: 2,
        active: false,
    };
    user2.active = true; // Modify a field

    // 5. Update Syntax: Use struct update syntax to create a new instance from an existing one.
    let user3 = User {
        email: String::from("new_email@example.com"),
        ..user2
    };

    // 6. Tuple Structs: Structs without named fields, useful for simple configurations.
    struct Color(i32, i32, i32);
    let black = Color(0, 0, 0);

    // 7. Unit-like Structs: Structs without fields, typically used for traits.
    struct AlwaysEqual;
    let _subject = AlwaysEqual;

    println!("Color: ({}, {}, {})", black.0, black.1, black.2);
    println!("User3: ({}, {}, {}, {})", user3.username, user3.email, user3.sign_in_count, user3.active);
}
