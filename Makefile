PASS := ./target/debug/pass
LINE := "------------------------------------------------------------"

run:
	cargo build
	$(PASS) insert meme
	@echo $(LINE)
	$(PASS)
