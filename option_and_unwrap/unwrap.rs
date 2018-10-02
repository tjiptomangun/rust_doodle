// https://doc.rust-lang.org/rust-by-example/error/option_unwrap.html
// An enum called Option<T> in the std library is used when absence is a possibility.
// It manifest itself as one of "options":
// Some(T): An element of type T was found
// None: No element was found
// These cases can either be explicitly handled via match or implicitly with unwrap.
// Implicit handling will either return the inner element or panic
// Not that it's possible to manually customize panic with expect, but unwrap
// otherwise leave us with a less meaningful output than explicit handling.
// In the following example, explicit handling yields a more controlled result
// while retaining the option to panic if desired.

fn give_commoner(gift: Option<&str>) {
    // Specify a cource of action for each case
    match gift {
            Some("snake") => println!("Yuck"),
            Some(inner) => println!("Nice {} ", inner),
            None => println!("No gift"),
    }

}

fn give_princess(gift: Option<&str>) {
    let inside = gift.unwrap();
    if inside == "snake" {panic!("Bring me to my planet!!!")}
    else {println!("Hola {}", inside)}

}

fn main() {
        let food = Some("cabbage");
        let snake = Some("snake");
        let void = None;
        let bird = Some("robin");
        let nothing = None;

        give_commoner(food);
        give_commoner(snake);
        give_commoner(void);

        give_princess(bird);
        give_princess(nothing);
}
