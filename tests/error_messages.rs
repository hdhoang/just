use crate::common::*;

test! {
  name: expected_keyword,
  justfile: "foo := if '' == '' { '' } arlo { '' }",
  stderr: "
    error: Expected keyword `else` but found identifier `arlo`
      |
    1 | foo := if '' == '' { '' } arlo { '' }
      |                           ^^^^
  ",
  status: EXIT_FAILURE,
}

test! {
  name: unexpected_character,
  justfile: "!~",
  stderr: "
    error: Expected character `=`
      |
    1 | !~
      |  ^
  ",
  status: EXIT_FAILURE,
}

test! {
  name: subcommand_not_found,
  justfile: "x:
  not-a-command arg",
  stderr_regex: "not-a-command arg\n.*not (found|find subcommand)\nerror: Recipe.*line 2.*\n",
  status: 127,
}

test! {
  name: backtick_not_found,
  justfile: "x := `not-a-command arg`",
  stderr_regex: ".*not (found|find subcommand:)
error: Backtick failed.*
  |
1 | x := `not-a-command arg`
  |      ^^^^^^^^^^^^^^^^^^^
  ",
  status: 127,
}

test! {
  name: shell_not_found,
  justfile: "set shell := ['not-bash', '-c']
x:
  echo XYZ",
  stderr_regex: "echo XYZ\nerror: Recipe.*not find subcommand:.*\n",
  status: EXIT_FAILURE,
  shell: false,
}

#[test]
fn argument_count_mismatch() {
  Test::new()
    .justfile("foo a b:")
    .args(&["foo"])
    .stderr(
      "
      error: Recipe `foo` got 0 arguments but takes 2
      usage:
          just foo a b
    ",
    )
    .status(EXIT_FAILURE)
    .run();
}
