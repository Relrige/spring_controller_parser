/// Representaion of Spring Controller in Java
/// It contains class name, class_mapping and all methods
#[derive(Debug, PartialEq, Eq, Clone)]
pub struct Controller {
    pub name: String,
    pub class_mapping: Option<String>,
    pub methods: Vec<ControllerMethod>,
}
/// Representation of Controller Method in Spring Java
/// It contains annotation, annotation args and header
#[derive(Debug, PartialEq, Eq, Clone)]
pub struct ControllerMethod {
    pub annotation: Option<String>,
    pub annotation_args: Option<String>,
    pub header: String,
}
