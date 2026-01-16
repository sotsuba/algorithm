.PHONY: rust cpp py

rust: 
	rustc main.rs -o bin/rust && bin/rust 

cpp:
	g++ -g main.cpp -o bin/cpp && bin/cpp

py:
	python main.py	

move:
	@echo "Target Directory is: [$(DIR)]"
	@read -p "Are you sure? [y/N] " ans && [ $${ans:-N} = y ]
	cp -v main.* $(DIR)/