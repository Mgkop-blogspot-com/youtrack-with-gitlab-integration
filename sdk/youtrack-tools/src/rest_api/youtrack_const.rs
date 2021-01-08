pub type Endpoint = String;

pub enum Endpoints {
    PROJECTS,
    PROJECT,
    USERS,
    USER,
    TASKS,
    TASK,
}

impl Endpoints {
    pub fn get_path(&self) -> Endpoint {
        let empty: Endpoint = "".to_string();
        match self {
            Endpoints::PROJECTS => empty,
            Endpoints::PROJECT => empty,
            Endpoints::USERS => empty,
            Endpoints::USER => empty,
            Endpoints::TASKS => empty,
            Endpoints::TASK => empty,
        }
    }
}