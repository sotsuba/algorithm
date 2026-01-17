.PHONY: rust cpp py move

EXTS ?= cpp rs py

rust: 
	rustc main.rs -o bin/rust && bin/rust 

cpp:
	g++ -g main.cpp -o bin/cpp && bin/cpp

py:
	python main.py	

move:
	@echo "Target Directory is: [$(DIR)]"
	@read -p "Are you sure? [y/N] " ans && [ $${ans:-N} = y ]
	mkdir $(DIR) -p
	@for ext in $(EXTS); do \
		if [ -f main.$$ext ]; then \
			cp -v main.$$ext $(DIR)/; \
		fi \
	done