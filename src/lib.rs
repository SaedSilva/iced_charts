pub mod bar_chart;
pub mod pie_chart;


fn get_max_value(values: &Vec<f32>) -> f32 {
    if values.is_empty() { 
        return 0.0;
    }
    
    let mut max = values[0];
    
    for &item in values.iter() {
        if item > max {
            max = item;
        }
    }
    
    max
}