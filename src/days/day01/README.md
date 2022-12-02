I wanted to include the most exciting things about rust to me. But I couldn't fit them all in. Here is just some highlight about what I could fit in the first day.


# How I got started
first i created a lib project with default test.
```
cargo new day01 --lib
```
then I took the sample file content and created a string of that for testing.
I also put the same content int a file. Loop through to determine if they were the same.
I create another test with possible issues, like a letter in the number. 

I then created a github project. cloned it
```
git clone git@github.com:tedsiy/adventofcode2020.git
cd adventofcode2020/src
mkdir days
cargo new day01 --lib # should have done this but i just copied day01 in
cd ../../
```
creted a new branch locally
```
git checkout -b day01
```
commit changes
```
git add .
git commit -am "day01 add changes"
gh pr create
```



# Testing
See `src/lib.rs` for test examples. I included both unit tests and doc-tests. The doc-test is what I wanted to call out here. Doc-test can be in the comment of your function. And they show up as function tips when you hover over them. And they show as an example in your docs.

You can run all test (unit and doc-test) with
```
cargo test
```

## Unit tests
You can see I have two tests in `mod tests`, but `ut_elf_calories()` can be removed since I have it in the doc test. 

## Doc-tests 
See `src/lib.rs` cargodoc contain three \ and example code contain three ` the doc example code must run.

You can run only the doc test for one function like this.
```
cargo test --doc --package day01 -- file_content_to_top_elf_calories
```

## creating crate docs
So you can have docs just like on crate.io by just running `rustdoc`. 

The command to create doc for the lib.rs
```
rustdoc src/lib.rs --crate-name day01
```
you will find the doc in 
```
doc/day01/index.html
```


## also included example build with docker
make sure you in the `day01` folder
```
docker build -t test .
docker run test
```


##  you can also add github action
github action can be used to test project in pr
https://github.com/actions-rs/cargo

# VSCode
in vscode I had to add the extention `CodeLLDB` to get the debugger to work. I also added  `rust-analyzer` extention. After adding this you can use VScode to add `launch.json` and config. Here is my config 

```
{
	"type": "lldb",
	"request": "launch",
	"name": "Debug executable 'day01'",
	"cargo": {
		"args": [
			"build",
			"--bin=day01",
			"--package=day01"
		],
		"filter": {
			"name": "day01",
			"kind": "bin"
		}
	},
	"args": ["sample.txt"],
	"cwd": "${workspaceFolder}"
},
```