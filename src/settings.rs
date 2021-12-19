use dotrix::ecs::{ Mut, };
use dotrix::{ Window, };
use dotrix::math::{ Vec2u, };

pub fn startup(
    mut window: Mut<Window>,
) {
    window.set_inner_size(Vec2u::new(1280, 720));

    if let Err(e) = window.set_cursor_grab(true) {
        println!("Cannot grab cursor! {}", e);
    }
    window.set_cursor_visible(false);
}
