use bevy::prelude::*;

#[derive(Component)]
struct Position {
    x: f32,
    y: f32
}

fn main() {
    let hello = "heyooo";
    println!("Hello, {} {}!", hello, foo("gakljrhlejk"));
    println!("Well, this is neat: {} - {}", foo2(8), foo2(23));

    let mut app = App::new();
    app.add_plugins(DefaultPlugins)
        .run();
}

fn foo(_x: &'static str) -> &'static str {
    return "world-";
}

fn foo2(x: i32) -> &'static str {
    let result: &'static str;
    if x < 10 {
        result = "less than 10";
    } else {
        result = "10 or more";
    }
    return result;
}

fn bar(x: i32) -> &'static str {
    if x < 10 {
        "less than 10"
    } else {
        "10 or more"
    }
}

// asdawd
