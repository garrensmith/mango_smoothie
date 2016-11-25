
doc:
	cargo doc --no-deps
	echo "<meta http-equiv=refresh content=0;url=mango_smoothie/index.html>" > target/doc/index.html
	cp -R target/doc ./docs
