all: setup examples analysis
	rustup override set 1.51

setup:
	rustup update
	rustup override set 1.51
	cargo build --release
	cp ./target/release/main ./pass-check

examples/loops/no_opt/loops.bc: examples/loops/loops.rs
	rustc -C panic=abort examples/loops/loops.rs --emit=llvm-bc -o examples/loops/no_opt/loops.bc

examples/loops/opt/loops.O.bc: examples/loops/loops.rs
	rustc -O -C panic=abort examples/loops/loops.rs --emit=llvm-bc -o examples/loops/opt/loops.O.bc

loops: examples/loops/no_opt/loops.bc examples/loops/opt/loops.O.bc

examples/const_folding/no_opt/const_folding.bc: examples/const_folding/const_folding.rs
	rustc -C panic=abort examples/const_folding/const_folding.rs --emit=llvm-bc -o examples/const_folding/no_opt/const_folding.bc

examples/const_folding/opt/const_folding.O.bc: examples/const_folding/const_folding.rs
	rustc -O -C panic=abort examples/const_folding/const_folding.rs --emit=llvm-bc -o examples/const_folding/opt/const_folding.O.bc

const_folding: examples/const_folding/opt/const_folding.O.bc examples/const_folding/no_opt/const_folding.bc

examples/sum/no_opt/sum.bc: examples/sum/sum.rs
	rustc -C panic=abort examples/sum/sum.rs --emit=llvm-bc -o examples/sum/no_opt/sum.bc

examples/sum/opt/sum.O.bc: examples/sum/sum.rs
	rustc -O -C panic=abort examples/sum/sum.rs --emit=llvm-bc -o examples/sum/opt/sum.O.bc

sum: examples/sum/no_opt/sum.bc examples/sum/opt/sum.O.bc

examples: const_folding sum loops

analysis: examples const_folding_analysis sum_analysis loops_analysis

loops_analysis: examples/loops/opt/analysis.json examples/loops/no_opt/analysis.json

examples/loops/no_opt/analysis.json: examples/loops/no_opt/loops.bc
	./pass-check -o examples/loops/no_opt/analysis.json -j -t examples/loops/no_opt/ analyze func loops::main

examples/loops/opt/analysis.json: examples/loops/opt/loops.O.bc
	./pass-check -o examples/loops/opt/analysis.json -j -t examples/loops/opt/ analyze func loops::main

sum_analysis: examples/sum/opt/analysis.json examples/sum/no_opt/analysis.json

examples/sum/no_opt/analysis.json: examples/sum/no_opt/sum.bc
	./pass-check -o examples/sum/no_opt/analysis.json -j -t examples/sum/no_opt/ analyze func sum::main

examples/sum/opt/analysis.json: examples/sum/opt/sum.O.bc
	./pass-check -o examples/sum/opt/analysis.json -j -t examples/sum/opt/ analyze func sum::main

const_folding_analysis: examples/const_folding/opt/analysis.json examples/const_folding/no_opt/analysis.json

examples/const_folding/no_opt/analysis.json: examples/const_folding/no_opt/const_folding.bc
	./pass-check -o examples/const_folding/no_opt/analysis.json -j -t examples/const_folding/no_opt/ analyze func const_folding::main

examples/const_folding/opt/analysis.json: examples/const_folding/opt/const_folding.O.bc
	./pass-check -o examples/const_folding/opt/analysis.json -j -t examples/const_folding/opt/ analyze func const_folding::main

clean:
	rm -rf examples/const_folding/opt/*.bc
	rm -rf examples/sum/opt/*.bc
	rm -rf examples/loops/opt/*.bc
	rm -rf examples/const_folding/no_opt/*.bc
	rm -rf examples/sum/no_opt/*.bc
	rm -rf examples/loops/no_opt/*.bc
	rm -rf examples/sum/opt/analysis.json
	rm -rf examples/sum/no_opt/analysis.json
	rm -rf examples/const_folding/no_opt/analysis.json
	rm -rf examples/const_folding/opt/analysis.json
	rm -rf examples/sum/no_opt/analysis.json
	rm -rf examples/sum/opt/analysis.json
	rm -rf ./pass-check
	cargo clean