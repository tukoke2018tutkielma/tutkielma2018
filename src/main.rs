#![allow(dead_code)]
use std::{env, process::{self, Command}, path::{Path, PathBuf}, fs::{self, File, DirEntry, create_dir_all},
                    io::{self, Error, ErrorKind, Read, Write, Seek}};
mod tokenizer;
use tokenizer::{tokenize, to_code};
extern crate walkdir;
use walkdir::{WalkDir};


fn help() {
    println!("{}", "
    Usage:

        command path_to_input_dir [path_to_output_dir]

        If no output directory specified output directory will be next to input directory and named
            /path/to/original_rust/
        ".to_string());
    process::exit(0)
}

fn visit_dirs(dir: &Path, cb: &Fn(&DirEntry)) -> io::Result<()> {
    if dir.is_dir() {
        for entry in fs::read_dir(dir)? {
            let entry = entry?;
            let path = entry.path();
            if path.is_dir() {
                visit_dirs(&path, cb)?;
            } else {
                cb(&entry);
            }
        }
    }
    Ok(())
}

fn handle_files(input_dir: String, output_dir: String) -> Result<(), Error>{
    let input_base_path = PathBuf::from(input_dir).canonicalize().unwrap();
    let output_base_path = PathBuf::from(&output_dir);
    let mut output_parent = PathBuf::from(&output_dir).parent()
        .expect("Could not get parent from output path")
        .to_path_buf();

    create_dir_all(&output_parent).unwrap();

    output_parent = output_parent.canonicalize()
        .expect("Could not make absolute path");
    let temp_current_dir = env::current_dir().unwrap();
    env::set_current_dir(&output_parent).unwrap();

    println!("{:?}", output_base_path);

    let filename = output_base_path.file_name()
        .expect("Could not get project name from output path")
        .to_str()
        .expect("Could not transform project name to str");

    let output = Command::new("cargo")
            .arg("new")
            .arg("--bin")
            .arg(filename)
            .output()
            .expect("Failed to run cargo. Ensure you have cargo installed on system");

    println!("Current directory {:?}", &temp_current_dir);

    if !output.status.success(){
        let error_msg = format!("Could not create project automatically! See original error from below!
        \n{}", String::from_utf8_lossy(&output.stderr));
        Err(Error::new(ErrorKind::Other, error_msg))
    } else {

        env::set_current_dir(temp_current_dir).unwrap();
        let mut output_base_path = PathBuf::from(output_dir).parent().unwrap().canonicalize().expect("Error in output_base_path canonicalize");
        output_base_path.push(format!("{}/src", filename));

        println!("Copying project from directory {:?}", input_base_path);
        println!("Copying project to directory {:?}", output_base_path);

        let walker = WalkDir::new(&input_base_path).into_iter();
        let mut paths: Vec<(String, String)> = Vec::new();

        for entry in walker {
            let entry = entry.unwrap();
            if is_java_file(&entry) {
                let input_path = entry.path().to_str().unwrap();
                let output_path_as_str = &input_path.replace(
                            input_base_path.to_str().unwrap(),
                            output_base_path.to_str().unwrap());
                let mut output_path = PathBuf::from(output_path_as_str).with_extension("rs");

                paths.push((input_path.to_string(), output_path.to_str().unwrap().to_string()));
            }
        }
        for tuple in paths {
            println!("File {} contents will be moved to {}", tuple.0, tuple.1);
            move_file(tuple)?
        }

        Ok(())
    }
}

fn move_file(paths: (String, String)) -> io::Result<()> {
    let (input, output) = paths;
    let mut file = File::open(input)?;
    let mut contents = String::new();
    file.read_to_string(&mut contents)?;
    let token_list = tokenize(&contents);
    let code = to_code(token_list)?;
    create_dir_all(Path::new(&output).parent().unwrap())?;
    let mut file = File::create(output)?;
    file.write_all(code.as_bytes())?;
    file.sync_all()?;
    Ok(())
}


fn is_java_file(entry: &walkdir::DirEntry) -> bool {
    entry.file_name()
         .to_str()
         .map(|s| s.ends_with(".java"))
         .unwrap_or(true)
}

fn main() {
    let args: Vec<String> = env::args().collect();
    println!("This software is NOT guaranteed to work!");
    //let args = ["asd".to_string(), "../../Downloads".to_string(), "../../Projects/testijuttu/asd/moi/asdaefs".to_string()];
    let (input_dir, output_dir) = match args.len() {
        1 => {
            eprintln!("Not enough arguments!");
            help();
            (String::new(), String::new())
        },
        2 => {
            let input_dir = Path::new(&args[1]);
            if !input_dir.exists() && !input_dir.is_dir() {
                eprintln!("Input directory does not exist or is not a directory!");
                help();
            }
            let input_dir = input_dir.canonicalize()
                .expect("Could not make input directory path absolute");
            let input_dir_as_string = input_dir.to_str()
                .expect("Failed parsing input directory to str")
                .to_string();
            let mut output_dir = PathBuf::from(&input_dir_as_string);
            output_dir.pop();
            output_dir.push(format!("{}_rust", input_dir.file_name().unwrap().to_str().unwrap()));
            let output_dir = output_dir.to_str()
                .expect("Failed parsing output directory to str")
                .to_string();
            (input_dir_as_string, output_dir)
        },
        3 => {
            let input_dir = PathBuf::from(&args[1]);
            if !input_dir.as_path().exists() && !input_dir.is_dir() {
                eprintln!("Input directory does not exist or is not a directory!");
                help();
            }
            let input_dir = input_dir.canonicalize()
                .unwrap()
                .to_str()
                .expect("Failed parsing input directory to str")
                .to_string();
            let output_dir = args[2].to_string();
            (input_dir, output_dir)
        }
        _ => {
            help();
            (String::new(), String::new())
        }
    };
    match handle_files(input_dir, output_dir.to_string()) {
        Ok(_)    => println!("Everything went well"),
        Err(err) => eprintln!("Error: {}", err.to_string()),
    }
    //println!("{:#?}", tokenizer::tokenize("int a;\t abba.cd(); mui='a' \n /* * /\"mui.\" /as */ x = 2/3; // asd \n private z>=\"a\"; asd=false;/**///kkk"));

}
