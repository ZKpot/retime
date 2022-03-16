use dotrix::{
    Input,
    input::{ ActionMapper, Button, KeyCode, Mapper, },
};

pub fn init_actions(input: &mut Input) {
    input.set_mapper(Box::new(Mapper::<Action>::new()));

    input.mapper_mut::<Mapper<Action>>()
        .set(vec![
            (Action::MoveForward, Button::Key(KeyCode::W)),
            (Action::MoveBackward, Button::Key(KeyCode::S)),
            (Action::MoveLeft, Button::Key(KeyCode::A)),
            (Action::MoveRight, Button::Key(KeyCode::D)),
            (Action::TurnLeft, Button::Key(KeyCode::Q)),
            (Action::TurnRight, Button::Key(KeyCode::E)),
            (Action::Pause, Button::Key(KeyCode::Escape)),
            (Action::RotateCamera, Button::MouseRight),
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
    Pause,
    RotateCamera,
}

// Bind Inputs and Actions
impl ActionMapper<Action> for Input {
    fn action_mapped(&self, action: Action) -> Option<&Button> {
        let mapper = self.mapper::<Mapper<Action>>();
        mapper.get_button(action)
    }
}
