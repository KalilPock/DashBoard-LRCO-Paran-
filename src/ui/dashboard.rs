use crate::models::{school::School, class::Class, assessment::Assessment};

pub fn render_dashboard(schools: &[School], classes: &[Class], assessments: &[Assessment]) {
    // A interface Leptos será reconstruída aqui futuramente
    println!("Dashboard renderizado com {} escolas, {} classes e {} avaliações.", schools.len(), classes.len(), assessments.len());
}
