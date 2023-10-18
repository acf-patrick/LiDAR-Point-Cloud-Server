use juniper::GraphQLObject;

#[derive(GraphQLObject)]
pub struct Color {
    pub red: i32,
    pub green: i32,
    pub blue: i32,
}

impl From<las::Color> for Color {
    fn from(value: las::Color) -> Self {
        Color {
            red: value.red.into(),
            green: value.green.into(),
            blue: value.blue.into(),
        }
    }
}
