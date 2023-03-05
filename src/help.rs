pub struct Help {}

impl Help {
    /// Prints help to console.
    pub fn get_help() -> () {
        let help: &str = "
usage: vault <command> <arg>
-- <arg> if applicable

1. See help
   vault help

2. Get value from vault
   vault get <key>

3. Add new value
   vault add
   -- the command will then prompt to provide the key
      and value for new entry

4. Print all values
   vault ls
        ";
        println!("{}", help);
    }
}