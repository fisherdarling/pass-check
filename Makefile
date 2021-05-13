
examples/loops.bc: examples/loops.rs
	rustc -C panic=abort examples/loops.rs --emit=llvm-bc -o examples/loops.bc

examples/loops.O.bc: examples/loops.rs
	rustc -O -C panic=abort examples/loops.rs --emit=llvm-bc -o examples/loops.O.bc

loops: examples/loops.bc examples/loops.O.bc

examples/const_folding.bc: examples/const_folding.rs
	rustc -C panic=abort examples/const_folding.rs --emit=llvm-bc -o examples/const_folding.bc

examples/const_folding.O.bc: examples/const_folding.rs
	rustc -O -C panic=abort examples/const_folding.rs --emit=llvm-bc -o examples/const_folding.O.bc

const_folding: examples/const_folding.O.bc examples/const_folding.bc

examples/sum.bc: examples/sum.rs
	rustc -C panic=abort examples/sum.rs --emit=llvm-bc -o examples/sum.bc

examples/sum.O.bc: examples/sum.rs
	rustc -O -C panic=abort examples/sum.rs --emit=llvm-bc -o examples/sum.O.bc

sum: examples/sum.bc examples/sum.O.bc

examples: const_folding sum loops


clean:
	rm -rf examples/*.ll
	rm -rf examples/*.bc