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
----+------------------------
 0  | /PATH/TO/FILE/file.txt
```

### Use file in external command
```
cb cp file2.txt
```
Equivalent to:
```
cp /PATH/TO/FILE/file.txt file2.txt
```
Runs any existing commands (like: `cp, cat, ls`) and substitute filepath as first argument.

### Alternatively, use the `-p` (`--pos`) flag and `$` to insert into a different location
```
cb add my_dir
cb -p rm -R -f $
```
will run
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
will run
```
mv /PATH/TO/X/x.txt /PATH/TO/X/x.txt
```


## Install
`cargo install --git https://github.com/FynnSu/clipboard`
