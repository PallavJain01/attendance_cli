# College Attendance command line tool
## Made for Personal, feel free to modify the subjects in `src/store.rs` for your use case

## USAGE
Clone this repo `https://github.com/PallavJain01/attendance_cli.git`
Run the following commands (or build a binary):
```
cargo run --
|-- add
│   + -d, --date <YYYY-MM-DD>
│   + -s, --subjects <SUBJECT[,SUBJECT...]>
│
*-- list
    |-- all
    |-- date
    │   + -d, --date <YYYY-MM-DD>
    |-- range
    │   + -r, --range <YYYY-MM-DD..YYYY-MM-DD>
    *-- subject
        + -s, --subject <SUBJECT>
```

## Examples:

```
cargo run -- add -d 2026-08-02 -s Dms,Tc
cargo run -- list all
cargo run -- list date -d 2026-08-02
cargo run -- list range -r 2026-08-01..2026-08-05
cargo run -- list subject -s Dms
```

## LICENSE
[MIT](LICENSE)