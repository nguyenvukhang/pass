PASS := ./target/debug/pass
LINE := "------------------------------------------------------------"

run:
	cargo build
	$(PASS)
