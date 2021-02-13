#[derive(Clone, Debug)]
pub enum UserInterfaceScreen {
    Home = 0,
    AskHackerNews = 1,
    ShowHackerNews = 2,
    Jobs = 3,
}

impl From<UserInterfaceScreen> for usize {
    fn from(value: UserInterfaceScreen) -> Self {
        value as usize
    }
}
