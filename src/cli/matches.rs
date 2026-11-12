
use std::collections::HashMap;
use std::path::PathBuf;
use std::{fs, println};

use std::str::from_utf8;

use colored::Colorize;

use crate::cli::commands::Commands;
use crate::enki_vm::compiler::extr_udatype_string;
use crate::tools::sita::Sita;
use crate::core::error::ZigurateError;
use crate::enki_vm::enki::{EnkiVM, LunigData};

use crate::cli::{
    get_enki_data
};

use crate::core::{zigurage_compiler_simple, zigurate_compiler_enki};





#[derive(Debug)]
enum EnkiPipeline {
    Empty,
    Enki(LunigData)
}







fn exec_enki_pipeline(sita: &Sita, path: &PathBuf, version_zigurate: &String, return_enki: bool, exec_path: bool, cmd: String, action: String) -> Result<EnkiPipeline, ZigurateError> {

    let curr_path = sita.current_dir()
        .map_err(|e| ZigurateError::IOError(e))?;

    let enki_data = get_enki_data(&sita, &curr_path);

    match enki_data {
        Ok(enki_data) => {

            let content = 
                if exec_path {
                    sita.read_file(&path).map_err(|e| ZigurateError::IOError(e))?
                } else {
                    sita.read_file(&enki_data.main).map_err(|e| ZigurateError::IOError(e))?
                };

            let byte_content = content.as_bytes();

            let cmd = format!("enki {}", cmd);
            let msg_2 = format!("");

            println!();
            println!("{}", version_zigurate);
            println!("--- {} {} st.{}", "info:".bold().yellow(), enki_data.name.italic(), enki_data.stage.italic());
            println!("--- {} {} {}", "cmd:".bold().yellow(), cmd.yellow().italic(), "!executed".bold().green());
            println!("--- {} {}", "in:".bold().yellow(), curr_path.display().to_string().underline());
            println!();
            println!("{} {}", action.bright_green().bold(), msg_2);

            // MAIN
            let enki_instructions = zigurage_compiler_simple(byte_content)?;
            let mut enki_vm = EnkiVM::on(enki_instructions);
            
            zigurate_compiler_enki(&mut enki_vm)?;
            
            if return_enki {
                return Ok(EnkiPipeline::Enki(enki_vm.lunig_hash))
            }

            Ok(EnkiPipeline::Empty)

        }
        Err(_) => {

            let content = sita.read_file(&path).map_err(|e| ZigurateError::IOError(e))?;
            let byte_content = content.as_bytes();

            let cmd = format!("enki {} {}", cmd, path.display());
            let msg_2 = format!("");

            println!();
            println!("{}", version_zigurate);
            println!("--- {}", "single-file".italic());
            println!("--- {} {} {}", "cmd:".bold().yellow(), cmd.yellow().italic(), "!executed".bold().green());
            println!("--- {} {}", "in:".bold().yellow(), curr_path.display().to_string().underline());
            println!();
            println!("{} {}", action.bright_green().bold(), msg_2);

            // MAIN
            let enki_instructions = zigurage_compiler_simple(byte_content)?;
            let mut enki_vm = EnkiVM::on(enki_instructions);

            zigurate_compiler_enki(&mut enki_vm)?;

            if return_enki {
                return Ok(EnkiPipeline::Enki(enki_vm.lunig_hash))
            }

            Ok(EnkiPipeline::Empty)

        }
    }

}


fn exec_galam() -> Result<(), ZigurateError> {
    Ok(())
}


fn exec_enki_galam(pipeline: EnkiPipeline) -> Result<String, ZigurateError> {

    match pipeline {
        EnkiPipeline::Enki(enkies) => {

            let mut list = Vec::new();

            for (_, data) in enkies.iter() {
                
                let mut data_to_push: HashMap<String, String> = HashMap::new(); 
                
                for (keyname, keydata) in data.iter() { 
                    data_to_push.insert(keyname.to_owned(), extr_udatype_string(keydata)?);
                }

                list.push(data_to_push);


            }

            let data_json = serde_json::to_string_pretty(&list).unwrap();
            Ok(data_json)

        },
        EnkiPipeline::Empty => {
            Err(ZigurateError::ProjectNotFoundOrNotExist)
        }
    }

}








pub fn match_command(cmd: &Commands) -> Result<(), ZigurateError> {

    let sita = Sita::gibil();

    let zigurate_name            = format!("{}", "/|\\ ZIGURATE");
    let status_stable            = format!("{}", "(beta 6)".yellow().italic());
    let version_zigurate = format!("{} — UR1 {}", zigurate_name.bold(), status_stable);

    match cmd {

        Commands::Run { path } => {
            match path {
                Some(path_main) => {
                    let _ = exec_enki_pipeline(&sita, path_main, &version_zigurate, false, false, "run".to_string(), "Compiled".to_string())?;
                },
                None => {
                    let _ = exec_enki_pipeline(&sita, &PathBuf::from("NONE"), &version_zigurate, false, false, "run".to_string(), "Compiled".to_string())?;
                } 
            }
        },
    
        Commands::Create { project_name  } => {

            let curr_path = sita.current_dir()
                .map_err(|e| ZigurateError::IOError(e))?; 

            let project_already_exists = sita.path_exists(&curr_path.join(project_name))
                .map_err(|e| ZigurateError::IOError(e))?;

            if !project_already_exists {

                let fld_project = sita.create_dir(project_name, &curr_path)
                    .map_err(|e| ZigurateError::IOError(e))?;

                let src_project = sita.create_dir("world", &fld_project)
                    .map_err(|e| ZigurateError::IOError(e))?;

                let fil_main = sita.create_file("main.zi", src_project)
                    .map_err(|e| ZigurateError::IOError(e))?;

                let _ = sita.edit_file(fil_main, "silim(\"Hello World!\")".to_string());

                let fil_enki = sita.create_file("Enki.yaml", &fld_project)
                    .map_err(|e| ZigurateError::IOError(e))?;

                let enki_content = format!(
                    "# See more about Zigurate Projects in https://ziguratedsl.vercel.app/docs/projects/about.html\nname: \"{name}\"\ndescription: \"\"\ncreator: []\nstage: \"1\"\n\nmain: \"world/main.zi\"\n\ngalam:",
                    name = project_name
                );

                let _ = sita.edit_file(fil_enki, enki_content);

                let cmd = format!("enki create {}", project_name.yellow().italic());

                let cmd2 = format!("cd .\\{}\\", project_name);

                let msg_2 = format!("Project {} was created.", project_name.yellow().italic());
                let msg_3 = format!("Now, use {}", cmd2.yellow().italic());
                let msg_4 = format!("After, run {}", "enki run".yellow().italic());

                println!();
                println!("{}", version_zigurate);
                println!("--- {} {} {}", "cmd:".bold().yellow(), cmd.yellow().italic(), "!executed".bold().green());
                println!("--- {} {}", "in:".bold().yellow(), curr_path.display().to_string().underline());
                println!();
                println!("{} {}", "success:".bright_green().bold(), msg_2);
                
                println!();
                println!("{} {}", "tip:".bright_yellow().bold(), msg_3);
                println!("{} {}", "tip:".bright_yellow().bold(), msg_4);
                println!();

            } else {

                let cmd = format!("enki create {}", project_name);

                let msg_2 = format!("Already exists a project called {}", project_name.yellow().italic());
                
                println!();
                println!("{}", version_zigurate);
                println!("--- {} {} {}", "cmd:".bold().yellow(), cmd.yellow().italic(), "!not executed".bold().red());
                println!("--- {} {}", "in:".bold().yellow(), curr_path.display().to_string().underline());
                println!();
                println!("{} {}", "warning:".bright_yellow().bold(), msg_2);
                println!();

                return Ok(())

            }
        
        }

        Commands::Galam  { path } => {
                        
            let curr_path = sita.current_dir()
                .map_err(|e| ZigurateError::IOError(e))?; 
            
            let enki_data = get_enki_data(&sita, &curr_path);

            match enki_data {
                Ok(enkidata) => {
                    
                    let main_path = PathBuf::from(enkidata.main);
                    let galam_paths = enkidata.galam;

                    if galam_paths.len() > 0 {

                        for (i, path_zi) in galam_paths.iter().enumerate() {

                            let path_buff = PathBuf::from(path_zi.replace("/", "\\"));

                            let files = sita.read_dir(&path_buff)
                                .map_err(|e| ZigurateError::IOError(e))?;

                            if files.len() == 0 {
                                return Err(ZigurateError::IOError("Galam list is empty in Enki.yaml".to_string()))
                            }

                            if !path_buff.is_dir() {
                                return Err(ZigurateError::IOError("Expected a directory. Got unknown file.".to_string()))
                            } 

                            for file in files.iter() {

                                let forma_path = curr_path.join(file);

                                let pipeline = exec_enki_pipeline(&sita, &forma_path, &version_zigurate, true, true, "galam".to_string(), "Exported".to_string())?;
                                let json = exec_enki_galam(pipeline)?;

                                let base_path = sita.create_dir("base", &curr_path)
                                    .map_err(|e| ZigurateError::IOError(e))?; 

                                let singlefile_name = file
                                    .file_stem()
                                    .and_then(|s| s.to_str())
                                    .map(|s| format!("{}.json", s))
                                    .unwrap_or_else(|| "base.json".to_string());
                        
                                let jsonfile = sita.create_file(&singlefile_name, base_path)
                                    .map_err(|e| ZigurateError::IOError(e))?;

                                sita.edit_file(jsonfile, json)
                                    .map_err(|e| ZigurateError::IOError(e))?;

                            }

                        }

                    } else {
                        
                        let pipeline = exec_enki_pipeline(&sita, &main_path, &version_zigurate, true, false, "galam".to_string(), "Exported".to_string())?;
                        let json = exec_enki_galam(pipeline)?;

                        let base_path = sita.create_dir("base", &curr_path)
                            .map_err(|e| ZigurateError::IOError(e))?; 

                        let singlefile_name = main_path
                            .file_stem()
                            .and_then(|s| s.to_str())
                            .map(|s| format!("{}.json", s))
                            .unwrap_or_else(|| "base.json".to_string());
                
                        let jsonfile = sita.create_file(&singlefile_name, base_path)
                            .map_err(|e| ZigurateError::IOError(e))?;

                        sita.edit_file(jsonfile, json)
                            .map_err(|e| ZigurateError::IOError(e))?;

                    }


                    

                },
                Err(_) => {

                    match path {
                        Some(path_main) => {

                            let pipeline = exec_enki_pipeline(&sita, &path_main, &version_zigurate, true, false, "galam".to_string(), "Exported".to_string())?;
                            let json = exec_enki_galam(pipeline)?;

                            let base_path = sita.create_dir("base", &curr_path)
                                .map_err(|e| ZigurateError::IOError(e))?; 

                            let singlefile_name = path_main
                                .file_stem()
                                .and_then(|s| s.to_str())
                                .map(|s| format!("{}.json", s))
                                .unwrap_or_else(|| "base.json".to_string());
                    
                            let jsonfile = sita.create_file(&singlefile_name, base_path)
                                .map_err(|e| ZigurateError::IOError(e))?;

                            sita.edit_file(jsonfile, json)
                                .map_err(|e| ZigurateError::IOError(e))?;

                        },
                        None => {
                            return Err(ZigurateError::ParametersNotProvided("Path not provided.".to_string()))
                        }
                    }

                }
            }

        }

    }

    Ok(())
}