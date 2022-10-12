# Clipboard

A Command Line Tool to simplify file operations. 

Note: Under development.

## Usage
### Add a file to the clipboard using:

```
clipboard add file.txt
```

### View items on clipboard using:
```
clipboard list
```
Output:
```
Clipboard:

 Id | Path | Name 
----+------+----------
 0  | /    | file.txt 
```

### Use file in external command
```
clipboard cp file_copy.txt
```
Equivalent to:
```
cp /file.txt file_copy.txt
```
Runs other existing commands (like: `cp, cat, ls`) and substitute filepath as first argument.


## Install
`cargo install --git https://github.com/FynnSu/clipboard`
