use DemDiscrete;
use base::Base;
use std::env;
use std::fs;
use std;

pub fn get_output_directory_name() -> String {
    let executable = std::env::current_exe().unwrap();
    let name =  executable.to_str().unwrap();
    let mut file_name = name.split("debug").skip(1);
    let crate_dir = env!("CARGO_MANIFEST_DIR");
    let dir_name = format!(
        "{}{}_output",
        crate_dir,
        file_name.next().unwrap(),
    );
    println!("{:?}", dir_name);
    dir_name.to_string()
}

pub fn create_output_directory() {
    let dir_name = get_output_directory_name();
    let _ = fs::create_dir_all(dir_name);
}

pub trait DumpData {
    fn save_data(&self, &str);
}

impl DumpData for DemDiscrete {
    fn save_data(&self, file_name: &str) {}
}

pub fn dump_output<T: DumpData>(entities: &mut Vec<&mut T>, t: f32) {
    for entity in entities {
        let file_name = 4;
        // entity.save_data(file_name);
    }
}
