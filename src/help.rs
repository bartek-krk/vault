pub struct Help {}

impl Help {
    pub fn get_help() -> () {
        let help: &str = "
usage: vault <command> <arg>
-- <arg> if applicable

1. Get value from vault
   vault get <key>

2. See help
   vault help

3. Add new value
   vault add
   -- the command will then prompt to provide the key
      and value for new entry
        ";
        println!("{}", help);
    }
}