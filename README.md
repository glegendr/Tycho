# Tycho
C package Manager

## PRE

you need to install `rust`

## Install

`git clone -b 'v0.9' --single-branch --depth 1 https://github.com/glegendr/Tycho.git $HOME/.tycho; cd $HOME/.tycho; make; export TYCHO_PATH=$HOME/.tycho ; export PATH=$HOME/.tycho/bin:$PATH ; cd -`

## Commands
### Init: 
Init your project with command `tycho init [FLAGS] <name>`

He can be launch by git-hub's vessel by adding `-g` flag

### Deploy
This command will deploy pods in your pods directory with commands `tycho deploy <url>`

## Update
Tycho update is why the vessel has been construct.

Tycho update can update your makefile and your dependencies with command `tycho update [FLAGS]`

* You can update your makefile
     * `tycho update -m` will update your makefile with all your `.c` in `src/` and your `.h` in `inc/`
     * `tycho update -r` will reset your makefile with only `main.c`
     * Care `-r` flag it will replace your binary by a.out
* You can also deploy and update librairies (better than `tycho deploy`)
     * `tycho update -p` work with `pod.toml` that you have to create at the root
     * This `pod.toml` work like this:
     ```
     [dependencies]
     name="url"
     libvec="https://github.com/glegendr/libvec.git"
     ```
     * He will clone your dependencies and add them to Makefile
     * You have to create `libvec.a` manualy by going in `./pods/libvec` and lanch `make`
     * Care in this version you have to lanch `tycho update -p` a second time for makefile
 * You can use `tycho update -h` to get help with update operation
## Other
* You can use `tycho -h` to get help
* You can use `tycho -V` to get version
