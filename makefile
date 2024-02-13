# Everyone gets their own make command. Woo!
# Shorthand can be useful for quick stacktraces.

########## TESTING ##########

# Useful for seeing println output during unit tests.
default:
	cargo test -- --nocapture

# Useful for seeing EVERYTHING. e stands for everything. :)
e:
	RUST_BACKTRACE=1 cargo test -- --nocapture


#############################