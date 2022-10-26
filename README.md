# Syncr

For your own good, don't try to learn anything from this project or use it in any way, it's filled with bad practices.
Rust is a fun programming language though.

Just to make it extremely clear that I'm not a rust developer:
```rust
macro_rules! arg_cmd {
  (cmd $cmd_id:ident $(as $cmd_name:literal)?:
   {
   main will $main_help:literal $main_block:block
   $(arg $arg_id:ident $(as $arg_name:literal)? will $arg_help:literal $arg_block:block )*
   $(subcmd $sub_cmd:ident $(as $sub_cmd_name:literal)? can $sub_cmd_help:literal $sub_cmd_block:tt)*
   }) => {
    arg_cmd!{
      cmd $cmd_id $(as $cmd_name)*: main will $main_help $main_block
      $(arg $arg_id $(as $arg_name)* will $arg_help $arg_block)*
      $(subcmd $sub_cmd $(as $sub_cmd_name)* can $sub_cmd_help $sub_cmd_block)*
    }
  };

  (cmd $cmd_id:ident $(as $cmd_name:literal)?:
   main will $main_help:literal $main_block:block
   $(arg $arg_id:ident $(as $arg_name:literal)? will $arg_help:literal $arg_block:block )*
   $(subcmd $sub_cmd:ident $(as $sub_cmd_name:literal)? can $sub_cmd_help:literal $sub_cmd_block:tt)*
  ) => {
    #[allow(non_camel_case_types)]
    mod $cmd_id {

      $(
      arg_cmd!{cmd $sub_cmd $(as $sub_cmd_name)*: $sub_cmd_block}
      )*

      pub fn print_full_help() {
        #[allow(unused_assignments,unused_variables)]
        let have_sub_cmds = false;
        $(
        _ = $sub_cmd_help;
        #[allow(unused_assignments,unused_variables)]
        let have_sub_cmds = true;
        )*
        if have_sub_cmds {
          println!("commands:");
          $(
          self::$sub_cmd::print_cmd_help();
          )*
          println!();
        }
        #[allow(unused_assignments)]
        let cmd_name = stringify!($cmd_id).trim_start_matches("r#");
        $(
        let cmd_name = $cmd_name;
        )*
        println!("args:");
        println!("  {}\n  \t{}", cmd_name, $main_help);
        $(
        #[allow(unused_assignments)]
        let arg_name = stringify!($arg_id);
        $(
        let arg_name = $arg_name;
        )*
        println!("  {} {}\n  \t{}", cmd_name, arg_name, $arg_help);
        )*
        println!("  {} help, --help\n  \tprint help text", cmd_name);
      }

      pub fn print_cmd_help() {
        #[allow(unused_assignments)]
        let cmd_name = stringify!($cmd_id).trim_start_matches("r#");
        $(
        let cmd_name = $cmd_name;
        )*
        println!("  {}\n  \t{}", cmd_name, $main_help);
      }

      pub fn handle_args(args: &[&str]) -> Result<(), std::io::Error> {
        match args {
          $(
          [stringify!($sub_cmd), rest @ ..]
          $(
          | [$sub_cmd_name, rest @ ..]
          )* => self::$sub_cmd::handle_args(rest)?,
          )*
          $(
          [stringify!($arg_id)]
          $(
          | [$arg_name]
          )* => $arg_block,
          )*
          ["--help", ..] | ["help", ..] => {
            println!("Syncr {}\n", env!("CARGO_PKG_VERSION"));
            self::print_full_help();
          }
          [] => $main_block,
          remaining => {
            #[allow(unused_assignments)]
            let cmd_name = stringify!($cmd_id).trim_start_matches("r#");
            $(
            let cmd_name = $cmd_name;
            )*
            println!("Unknown args to {}: '{}'\n", cmd_name, remaining.join("', '"));
            self::print_full_help();
          }
        }
        Ok(())
      }
    }
  };

  (cmd $cmd_id:ident $(as $cmd_name:literal)?:
   {
   main with
   $(required value $required_id:ident $(as $required_name:literal)?)?
   $(optional value $optional_id:ident $(as $optional_name:literal)?)?
   will $main_help:literal $main_call:tt
   $(arg $arg_id:ident $(as $arg_name:literal)? will $arg_help:literal $arg_block:block )*
   }) => {
    arg_cmd!{
      cmd $cmd_id $(as $cmd_name)*:
      main with
       $(required value $required_id $(as $required_name)*)*
       $(optional value $optional_id $(as $optional_name)*)*
      will $main_help $main_call
      $(arg $arg_id $(as $arg_name)* will $arg_help $arg_block)*
    }
  };

  (cmd $cmd_id:ident $(as $cmd_name:literal)?:
   main with
   $(required value $required_id:ident $(as $required_name:literal)?)?
   $(optional value $optional_id:ident $(as $optional_name:literal)?)?
   will $main_help:literal $main_call:tt
   $(arg $arg_id:ident $(as $arg_name:literal)? will $arg_help:literal $arg_block:block )*
  ) => {
    #[allow(non_camel_case_types)]
    mod $cmd_id {
      pub fn print_full_help() {
        #[allow(unused_assignments)]
        let cmd_name = stringify!($cmd_id).trim_start_matches("r#");
        $(
        let cmd_name = $cmd_name;
        )*
        println!("args:");
        $(
        #[allow(unused_assignments)]
        let required_name = stringify!($required_id).trim_start_matches("r#").to_uppercase();
        $(
        let required_name = $required_name.to_uppercase();
        )*
        println!("  {} {}\n  \t{}", cmd_name, required_name, $main_help);
        )*
        $(
        #[allow(unused_assignments)]
        let optional_name = stringify!($optional_id).trim_start_matches("r#").to_uppercase();
        $(
        let optional_name = $optional_name.to_uppercase()
        )*
        println!("  {} [{}]\n  \t{}", cmd_name, optional_name, $main_help);
        )*
        $(
        #[allow(unused_assignments)]
        let arg_name = stringify!($arg_id);
        $(
        let arg_name = $arg_name;
        )*
        println!("  {} --{}\n  \t{}", cmd_name, arg_name, $arg_help);
        )*
        println!("  {} --help\n  \tprint help text", cmd_name);
      }

      pub fn print_cmd_help() {
        #[allow(unused_assignments)]
        let cmd_name = stringify!($cmd_id).trim_start_matches("r#");
        $(
        let cmd_name = $cmd_name;
        )*
        println!("  {}\n  \t{}",cmd_name, $main_help);
      }

      pub fn handle_args(args: &[&str]) -> Result<(), std::io::Error> {
        match args {
          $(
          [concat!("--", stringify!($arg_id))]
          $(
          | [concat!("--", $arg_name)]
          )* => $arg_block,
          )*
          ["--help", ..] => {
            println!("Syncr {}\n", env!("CARGO_PKG_VERSION"));
            self::print_full_help();
          }
          $(
          [] => {
            let $optional_id = None;
            $main_call;
          },
          [value] => {
            let $optional_id = Some(*value);
            $main_call;
          },
          )*
          $(
          [value] => {
            let $required_id = *value;
            $main_call;
          },
          )*
          remaining => {
            #[allow(unused_assignments)]
            let cmd_name = stringify!($cmd_id).trim_start_matches("r#");
            $(
            let cmd_name = $cmd_name;
            )*
            println!("Unknown args to {}: '{}'\n", cmd_name, remaining.join("', '"));
            self::print_full_help();
          }
        }
        Ok(())
      }
    }
  };
}
```
