use dotrix::{
    Input,
    input::{ ActionMapper, Button, KeyCode, Mapper, Modifiers, },
};

pub fn init_actions(input: &mut Input) {
    input.set_mapper(Box::new(Mapper::<Action>::new()));

    input.mapper_mut::<Mapper<Action>>().set(&[
        (Action::MoveForward, Button::Key(KeyCode::W), Modifiers::empty()),
        (Action::MoveBackward, Button::Key(KeyCode::S), Modifiers::empty()),
        (Action::MoveLeft, Button::Key(KeyCode::A), Modifiers::empty()),
        (Action::MoveRight, Button::Key(KeyCode::D), Modifiers::empty()),
        (Action::TurnLeft, Button::Key(KeyCode::Q), Modifiers::empty()),
        (Action::TurnRight, Button::Key(KeyCode::E), Modifiers::empty()),
        (Action::RewindTime, Button::Key(KeyCode::Space), Modifiers::empty()),
        (Action::Pause, Button::Key(KeyCode::Escape), Modifiers::empty()),
        (Action::RotateCamera, Button::MouseRight, Modifiers::empty()),
        (Action::SelectActiveObjectRight, Button::Key(KeyCode::Right), Modifiers::empty()),
        (Action::SelectActiveObjectLeft, Button::Key(KeyCode::Left), Modifiers::empty()),
    ]);
}

#[derive(Debug, PartialEq, Eq, Hash, Copy, Clone)]
pub enum Action {
    MoveForward,
    MoveBackward,
    MoveLeft,
    MoveRight,
    TurnLeft,
    TurnRight,
    RewindTime,
    Pause,
    RotateCamera,
    SelectActiveObjectRight,
    SelectActiveObjectLeft,

}

// Bind Inputs and Actions
impl ActionMapper<Action> for Input {
    fn action_mapped(&self, action: Action) -> Option<(Button, Modifiers)> {
        let mapper = self.mapper::<Mapper<Action>>();
        mapper.get_button(action)
    }
}
