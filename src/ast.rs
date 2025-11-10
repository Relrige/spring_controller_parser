/// Representaion of Spring Controller in Java
/// It contains class name, class_mapping and all methods
#[derive(Debug, PartialEq, Eq, Clone)]
pub struct Controller {
    /// Name of the controller class
    pub name: String,
    /// Optional `@RequestMapping` value for the class
    pub class_mapping: Option<String>,
    /// Methods found inside this controller
    pub methods: Vec<ControllerMethod>,
}
/// Representation of Controller Method in Spring Java
/// It contains annotation, annotation args and header
#[derive(Debug, PartialEq, Eq, Clone)]
pub struct ControllerMethod {
    /// Name of the annotation
    pub annotation: Option<String>,
    /// Annotation args
    pub annotation_args: Option<String>,
    /// Header of method
    pub header: String,
}
