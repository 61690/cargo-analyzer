use crate::types::{Warning, CategoryType};
use serde::{Serialize, Deserialize};

#[derive(Debug, Serialize, Deserialize)]
pub struct FixExample {
    pub description: String,
    pub before: String,
    pub after: String,
    pub explanation: String,
    pub additional_notes: Vec<String>,
}

pub fn get_fix_example(warning: &Warning) -> Option<FixExample> {
    let subcategory = warning.message.split_whitespace().next().unwrap_or("");
    
    match (warning.category, subcategory) {
        (CategoryType::Performance, "Locking") => Some(FixExample {
            description: "Efficient locking patterns".to_string(),
            before: r#"
// ❌ Inefficient: Long-held locks
fn process_data(data: &Arc<Mutex<Vec<String>>>) {
    let mut locked = data.lock().unwrap();
    for item in locked.iter_mut() {
        expensive_operation(item);  // Lock held during expensive operation
    }
}"#.to_string(),
            after: r#"
// ✅ Efficient: Minimize lock duration
fn process_data(data: &Arc<Mutex<Vec<String>>>) {
    // Option 1: Clone data under lock
    let items = {
        let locked = data.lock().unwrap();
        locked.clone()
    };
    for item in &items {
        expensive_operation(item);
    }

    // Option 2: Process items individually
    for i in 0..data.lock().unwrap().len() {
        let item = {
            let locked = data.lock().unwrap();
            locked[i].clone()
        };
        expensive_operation(&item);
    }
}"#.to_string(),
            explanation: "Minimize lock duration and consider alternative synchronization primitives".to_string(),
            additional_notes: vec![
                "Use RwLock for read-heavy workloads".to_string(),
                "Consider lock-free data structures".to_string(),
                "Minimize critical section size".to_string(),
            ],
        }),

        (CategoryType::Documentation, "Missing") => Some(FixExample {
            description: "Documentation examples".to_string(),
            before: r#"
pub struct Configuration {
    timeout: Duration,
    retries: u32,
}"#.to_string(),
            after: r#"/// Configuration for network operations
/// 
/// # Examples
/// 
/// ```
/// let config = Configuration::new(
///     Duration::from_secs(30),
///     3
/// )?;
/// ```
pub struct Configuration {
    timeout: Duration,
    retries: u32,
}"#.to_string(),
            explanation: "Add comprehensive examples to documentation".to_string(),
            additional_notes: vec![
                "Include practical usage examples".to_string(),
                "Show error handling".to_string(),
                "Demonstrate common use cases".to_string(),
            ],
        }),

        // Add more examples as needed...
        _ => None,
    }
}
