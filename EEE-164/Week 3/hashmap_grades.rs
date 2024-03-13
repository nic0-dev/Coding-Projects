use std::collections::HashMap;

fn main() {
    let mut grade_scale = HashMap::from([
        (100, "Excellent".to_string()),
        (150, "ExcellentV2".to_string()),
        (300, "Pass".to_string()),
        (500, "Fail".to_string()),
    ]);

    println!("grade_scale {grade_scale:?}");

    // Insert
    grade_scale.insert(150, "Very Good".to_string());
    grade_scale.insert(200, "Good".to_string());
    grade_scale.insert(125, "Good".to_string()); // Typo
    grade_scale.insert(250, "Sastifactory".to_string()); // Typo

    println!("grade_scale (after insert) {grade_scale:?}");

    // Remove
    grade_scale.remove(&250);

    println!("grade_scale (after remove) {grade_scale:?}");

    // Update existing
    grade_scale.insert(125, "Excellent".to_string());

    println!("grade_scale (after overwrite) {grade_scale:?}");

    // Insert only if exists
    grade_scale.entry(100).or_insert("Poor".to_string());
    grade_scale.entry(175).or_insert("Very Good".to_string());

    println!("grade_scale (after insert if exists) {grade_scale:?}");

    // Loop over key-values
    for (k, v) in &grade_scale {
        println!("kv {} - {}", k, v);
    }

    // Loop over values
    for &each_val in grade_scale.keys() {
        println!("Grade {}", (each_val as f64) / 100.0);
    }

    // Mutating loop over values
    for (_, v) in &mut grade_scale {
        *v += "_";
    }

    println!("grade_scale (after mutating loop) {grade_scale:?}");
}