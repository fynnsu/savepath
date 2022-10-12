# Clipboard

A Command Line Tool to simplify file operations. 

Note: Under development.

## Usage
### Add a file to the clipboard using:

```
cb add file.txt
```

### View items on clipboard using:
```
cb list
```
Output:
```
Clipboard:

 Id | Path
----+-------------------------------
 0  | /PATH/TO/FILE/file.txt
 1  | /PATH/TO/OLDER/FILE/older.txt
```

### Use most recently added path in external command
Runs any existing commands (like: `cp, cat, ls`) and substitute filepath as first argument.
```
cb cp file2.txt
```
Runs:
```
cp /PATH/TO/FILE/file.txt file2.txt
```

### Use `-i` (`--id`) to insert a different file from clipboard
```
cb -i 1 cp file2.txt
```
Runs:
```
cp /PATH/TO/OLDER/FILE/older.txt file2.txt
```


### Use `-p` (`--pos`) flag and `$` to insert into a different location
```
cb add my_dir
cb -p rm -R -f $
```
Runs:
```
rm -R -f /PATH/TO/MY/DIR/my_dir
```

Note: currently `$` will only be replaced with the path if it is passed as an argument (and not a substring of an argument).
i.e.
`cb -p ls $` works but `cb ls $/sub_dir` doesn't. 

### Or substitute multiple times
```
cb add x.txt
cb -p mv $ $
```
Runs:
```
mv /PATH/TO/X/x.txt /PATH/TO/X/x.txt
```


## Install
`cargo install --git https://github.com/FynnSu/clipboard`
