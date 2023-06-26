struct ia_error_info_struct {
    pb_module_name: String,
    ppb_class_names: Vec<String>,
    ppppb_error_msg_pointers: Vec<Vec<Vec<Vec<String>>>>,
}

impl ia_error_info_struct {
    fn new() -> Self {
        ia_error_info_struct {
            pb_module_name: "Ittiam mpegh_dec".to_string(),
            ppb_class_names: vec![
                "API".to_string(),
                "Configuration".to_string(),
                "Initialization".to_string(),
                "Execution".to_string(),
                "".to_string(),
                "".to_string(),
                "".to_string(),
                "".to_string(),
                "".to_string(),
                "MPEGHD".to_string(),
            ],
            ppppb_error_msg_pointers: vec![
                vec![
                    vec![vec!["".to_string(); 16]; 7],
                    vec![vec!["".to_string(); 16]; 7],
                    vec![vec!["".to_string(); 16]; 7],
                    vec![vec!["".to_string(); 16]; 7],
                    vec![vec!["".to_string(); 16]; 7],
                    vec![vec!["".to_string(); 16]; 7],
                    vec![vec!["".to_string(); 16]; 7],
                ],
                vec![
                    vec![vec!["".to_string(); 16]; 7],
                    vec![vec!["".to_string(); 16]; 7],
                    vec![vec!["".to_string(); 16]; 7],
                    vec![vec!["".to_string(); 16]; 7],
                    vec![vec!["".to_string(); 16]; 7],
                    vec![vec!["".to_string(); 16]; 7],
                    vec![vec!["".to_string(); 16]; 7],
                ],
            ],
        }
    }
}

fn main() {
    static MY_STRUCT: ia_error_info_struct = ia_error_info_struct::new();

    // Accessing and printing the fields of MY_STRUCT
    println!("Module Name: {}", MY_STRUCT.pb_module_name);
    println!("Class Names: {:?}", MY_STRUCT.ppb_class_names);
    println!("Error Message Pointers: {:?}", MY_STRUCT.ppppb_error_msg_pointers);
}
