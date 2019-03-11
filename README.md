# Tycho
C package Manager

## PRE

you need to install `rust` with command `curl https://sh.rustup.rs -sSf | sh`

## Install

`git clone -b 'v0.10' --single-branch --depth 1 https://github.com/glegendr/Tycho.git $HOME/.tycho; cd $HOME/.tycho; make; export TYCHO_PATH=$HOME/.tycho ; export PATH=$HOME/.tycho/bin:$PATH ; cd -`

## Commands
### Init
Init your project with command `tycho init [FLAGS] <name>`

He can be launch by git-hub's vessel by adding `-g` flag

### Deploy
This command will deploy pods in your pods directory with commands `tycho deploy <url>` and add the url in pod.toml
Care it will not be deployed in your `Makefile` !

### Update
Tycho update is why the vessel has been construct.

Tycho update can update your makefile and your dependencies with command `tycho update [FLAGS]`

* You can update your makefile
     * `tycho update -m` or `tycho update` will make `tycho update -is`
     * `tycho update -i` will add your dependencies (`pods/*`) in `Makefile`
     * `tycho update -s` will add your sources (`src/*`) in `Makefile`
     * `tycho update -r` will reset your makefile with only `main.c`
* You can also deploy and update librairies (inverse of `tycho deploy`)
     * `tycho update -p` work with `pod.toml` that you have to create at the root
     * This `pod.toml` work like this:
     ```
     [dependencies]
     name="url"
     libvec="https://github.com/glegendr/libvec.git"
     ```
     * He will clone your dependencies and add them to Makefile
     * You have to create `libvec.a` manualy by going in `./pods/libvec` and lanch `make`
* You can use `tycho update -h` to get help with update operation

### Other
* You can use `tycho -h` to get help
* You can use `tycho -V` to get version
