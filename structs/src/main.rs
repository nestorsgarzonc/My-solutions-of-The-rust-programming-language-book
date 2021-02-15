struct User {
    username: String,
    name: String,
    email: String,
    sign_in_count: u64,
    active: bool,
}

fn main() {
    let mut user = User {
        active: true,
        email: String::from("nestorsgarzonc@gmail.com"),
        name: String::from("Nestor Sebastian Garzon Contreras"),
        username: String::from("nestorsgarzonc"),
        sign_in_count: 1,
    };
    println!("active: {}", user.active);
    println!("email: {}", user.email);
    println!("name: {}", user.name);
    println!("username: {}", user.username);
    println!("sign_in_count: {}", user.sign_in_count);
    // Mutate attributes
    user.sign_in_count += 1;
    println!("sign_in_count: {}", user.sign_in_count);
    //Instanciate an user using a function:
    let user_fn = build_user(
        String::from("negarzonc"),
        String::from("Nestor Sebastian Garzon Contreras"),
        String::from("negarzonc@unal.edu.co"),
    );
    print_user(user_fn);
}

// OUTPUT:
//   active: true
//   email: nestorsgarzonc@gmail.com
//   name: Nestor Sebastian Garzon Contreras
//   username: nestorsgarzonc
//   sign_in_count: 1

fn build_user(email: String, name: String, username: String) -> User {
    User {
        email,
        name,
        username,
        sign_in_count: 1,
        active: true,
    }
}

fn print_user(user: User) {
    println!("active: {}", user.active);
    println!("email: {}", user.email);
    println!("name: {}", user.name);
    println!("username: {}", user.username);
    println!("sign_in_count: {}", user.sign_in_count);
}
