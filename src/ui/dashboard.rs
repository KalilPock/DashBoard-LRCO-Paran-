use crate::models::{school::School, class::Class, assessment::Assessment};

pub fn render_dashboard(schools: &[School], classes: &[Class], assessments: &[Assessment]) {
    println!("=== LRCO Unified Dashboard ===");
    for school in schools {
        println!("\nSchool: {}", school.name);
        
        let school_classes: Vec<&Class> = classes.iter().filter(|c| c.school_id == school.id).collect();
        for class in school_classes {
            println!("  Class: {} | Schedule: {}", class.subject, class.schedule);
            
            let class_assessments: Vec<&Assessment> = assessments.iter().filter(|a| a.class_id == class.id).collect();
            for assessment in class_assessments {
                println!("    Assessment: {} | Date: {}", assessment.r#type, assessment.date);
            }
        }
    }
}
