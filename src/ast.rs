// Representaion of Spring Controller in Java
#[derive(Debug, PartialEq, Eq, Clone)]
pub struct Controller {
    pub name: String,
    pub class_mapping: Option<String>,
    pub methods: Vec<ControllerMethod>,
}
// Representation of Controller Method in Spring Java
#[derive(Debug, PartialEq, Eq, Clone)]
pub struct ControllerMethod {
    pub annotation: Option<String>,
    pub annotation_args: Option<String>,
    pub header: String,
}
