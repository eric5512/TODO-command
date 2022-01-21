all:
	rustc src/main.rs -o bin/TODOs.exe

test:
	rustc src/main.rs -o bin/TODOs.exe
	.\\bin\\TODOs.exe