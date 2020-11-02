# Quickstart

## Installation



# Usage

## Help

#### _Show help on commandline_
```shell
pier help
```

#### _Show sub command help_
```shell
pier <subcmd>--help
```

## Init
_Initialize configuration_
```shell
pier init
```

## Run

#### _Run script_
```shell
pier run hello-pier

```

#### _Run script with arguments_
```shell
pier run <alias> <arg> <arg>
```


## Add

#### _Add script inline_
```shell
pier add "<command>" -a <alias>

```

#### _Add script and open command in editor_
Running the add command with only the `-a` option will cause it to open in your `$EDITOR`

```shell
pier add -a <alias>

```

## Edit
Edits script in `$EDITOR`

```shell
pier edit <alias>

```

## Show
Prints command

```shell
pier show <alias>

```

## Copy
```shell
pier cp <from> <to>
```

## Move
```shell
pier mv <from> <to>
```

## Remove
```shell
pier rm <alias>
```

## List

#### _List as table_
```shell
pier ls
```

#### _List only aliases_
```shell
pier ls -q
```

#### _List tags_
TODO implement this.
```shell
pier ls -T
```

#### _List aliases with tag_
```shell
pier ls -t system
```

## Execute pier scripts in any interpreted languages
Scripts starting with a shebang `#!` will be run with the specified interpeter just like it would in a normal script. Pier does this by creating a temp file from your script, executing it and then finally cleaning the file up. This allows you to write your pier script in python, node.js etc. even compiled languages can be run if using something like scriptisto.

### Shebang example config

```
[scripts.run_rust_script]
command = '''
#!/usr/bin/env node

console.log("hello world!")
'''
```

### Setting the default shell / interpreter
By default if no shebang is specified pier will try to use your default shell to execute the script inline. This can be overwritten with the variable interpreter. It needs to be a list with the first item being the binary and the rest is any flags necessary.

#### Default interpreter example config
```
# Sets the default interpreter, the first item in the list should be the binary and the rest are the arguments for the interpreter cli option.
[default]
interpreter = ["node", "-e"]

# Runs as the fallback interpreter nodejs as it's lacking a shebang
[scripts.hello_world_nodejs]
command = '''
console.log("Hello world!")

'''
```


