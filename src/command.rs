use std::process::Command as OsCommand;

pub struct Command<'a> {
    name: &'a str,
    args: Vec<String>, // Because converting a vector of &str to Strings is really annoying
    cmd: &'a str
}
impl<'a> Command<'a> {
    pub fn new(name: &'a str, args: Vec<String>, cmd: &'a str) -> Command<'a> {
        Command {
            name,
            args,
            cmd
        }
    }
    pub fn run(command: &str) -> Result<(), String> {
        // Run the given command (accounting for architecture)
        let cmd_data;
        if cfg!(target_os = "windows") {
            cmd_data = OsCommand::new("cmd")
                    .args(&["/C", &command])
                    .spawn();
        } else {
            cmd_data = OsCommand::new("sh")
                    .arg("-c")
                    .arg(&command)
                    .spawn();
        };

        match cmd_data {
            Ok(_) => Ok(()),
            Err(_) => Err(
                format!(
                    "Command '{}' failed to run. This doesn't mean the command produced an error, but that the process couldn't even be initialised.",
                    &command
                )
            )
        }
    }
    pub fn insert_args(&self, arg_values: &[String]) -> Result<String, String> {
        // Check if the correct number of arguments was provided
        // Return an error if there are too few
        // Warn if there are too many
        if self.args.len() > arg_values.len() {
            return Err(
                format!(
                    "The command '{command}' requires {num_required_args} argument(s), but {num_given_args} argument(s) were provided (too few). Please provide all the required arguments.",
                    command=&self.name,
                    num_required_args=&self.args.len(),
                    num_given_args=&arg_values.len()
                )
            );
        }
        if self.args.len() < arg_values.len() {
            println!(
                "Warning: The command '{command}' only needs {num_required_args} argument(s), but {num_given_args} argument(s) were provided (too many). Your command will still run, this warning is just here to save time in debugging!",
                command=&self.name,
                num_required_args=&self.args.len(),
                num_given_args=&arg_values.len()
            );
        }
        let mut command_with_args: String = self.cmd.to_string();
        // Loop through all the arguments the command takes, substituting in each one
        for (idx, arg) in self.args.iter().enumerate() {
            let arg_value = &arg_values[idx]; // The arrays are the same length, see above check
            // All arguments are shown in the command string as `%name` or the like, so we get that whole string
            let arg_with_sign = "%".to_string() + arg;
            let new_command = command_with_args.replace(&arg_with_sign, &arg_value);
            // Run a quick check to make sure we've changed something (otherwise there's probably a typo in the command)
            // We panic here because substituting '%arg' into the command may result in undefined behaviour
            if new_command == command_with_args {
                return Err(
                    format!(
                        "The argument '{arg_name}' could not be substituted into the command '{command}'. This probably means there's a typo somewhere in your command definition.",
                        arg_name=&arg,
                        command=&self.name
                    )
                );
            }
            command_with_args = new_command;
        };

        Ok(command_with_args)
    }
}
