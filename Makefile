all: setup examples
	rustup override set 1.51
	cargo build --release
	cp ./target/release/main ./pass-check

setup:
	rustup update
	rustup override set 1.51

examples/loops/loops.bc: examples/loops/loops.rs
	rustc -C panic=abort examples/loops/loops.rs --emit=llvm-bc -o examples/loops/loops.bc

examples/loops/opt/loops.O.bc: examples/loops/loops.rs
	rustc -O -C panic=abort examples/loops/loops.rs --emit=llvm-bc -o examples/loops/opt/loops.O.bc

loops: examples/loops/loops.bc examples/loops/opt/loops.O.bc

examples/const_folding/const_folding.bc: examples/const_folding/const_folding.rs
	rustc -C panic=abort examples/const_folding/const_folding.rs --emit=llvm-bc -o examples/const_folding/const_folding.bc

examples/const_folding/opt/const_folding.O.bc: examples/const_folding/const_folding.rs
	rustc -O -C panic=abort examples/const_folding/const_folding.rs --emit=llvm-bc -o examples/const_folding/opt/const_folding.O.bc

const_folding: examples/const_folding/opt/const_folding.O.bc examples/const_folding/const_folding.bc

examples/sum/sum.bc: examples/sum/sum.rs
	rustc -C panic=abort examples/sum/sum.rs --emit=llvm-bc -o examples/sum/sum.bc

examples/sum/opt/sum.O.bc: examples/sum/sum.rs
	rustc -O -C panic=abort examples/sum/sum.rs --emit=llvm-bc -o examples/sum/opt/sum.O.bc

sum: examples/sum/sum.bc examples/sum/opt/sum.O.bc

examples: const_folding sum loops


clean:
	rm -rf examples/const_folding/*.ll
	rm -rf examples/sum/*.ll
	rm -rf examples/loops/*.ll
	rm -rf examples/const_folding/*.bc
	rm -rf examples/sum/*.bc
	rm -rf examples/loops/*.bc
	rm -rf ./pass-check
	cargo clean