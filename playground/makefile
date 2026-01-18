.PHONY: rust cpp py move

EXTS ?= cpp rs py

rust: 
	rustc draft.rs -o bin/rust && bin/rust 

cpp:
	g++ -g draft.cpp -o bin/cpp && bin/cpp

py:
	python draft.py	

move:
	@echo "Target Directory is: [$(DIR)]"
	@read -p "Are you sure? [y/N] " ans && [ $${ans:-N} = y ]
	mkdir $(DIR) -p
	@for ext in $(EXTS); do \
		if [ -f draft.$$ext ]; then \
			cp -v draft.$$ext $(DIR)/; \
		fi \
	done