# day02
this is going to be a step by step

```
cargo new day02 --lib
```
create test in main then moved it to the lib.rs file

see day01 for more general info

```
git checkout -b day02

git add .
git commit -am "day02 add changes"
gh pr create
```



# Testing
```
cargo test
```

## creating crate docs
So you can have docs just like on crate.io by just running `rustdoc`. 

The command to create doc for the lib.rs
```
rustdoc src/lib.rs --crate-name day01
```
you will find the doc in 
```
doc/day02/index.html
```

## docker 
make sure you in the `day01` folder
```
docker build -t test .
docker run test
```

### is you don't have a rust env you can do this
you will need 2 terminals, one to run the rust container
```
docker run -ti rust:1.65.0-alpine3.16
```

second terminal will find the running container, copy content,
```
docker ps -l
docker cp . CONTAINER:/root/
```

you can even use VScode to edit. 
* extention Docker
find container, individual container, rust:1.65.0-alpine3.16
* right click `attach visual studio code`
* open folder
* hit debug, install `CodeLLDB`

you can open terminal and edit code, run cargo test, ...

