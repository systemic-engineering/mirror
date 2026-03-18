# beam/native/Makefile — Build native NIFs for the BEAM runtime.
#
# Usage: nix develop -c make -C beam/native
#
# Targets:
#   all           — build both prism and conversation NIFs
#   prism-nif     — Fortran prism operations
#   conv-nif      — Rust conversation parser

ERL_INCLUDE := $(shell erl -noshell -eval 'io:format("~s/erts-~s/include", [code:root_dir(), erlang:system_info(version)])' -s erlang halt)

PRIV_DIR := ../priv
PRISM_TARGET := $(PRIV_DIR)/conversation_prism_nif.so
CONV_TARGET  := $(PRIV_DIR)/conversation_nif.so

# Rust static library (built by cargo)
RUST_LIB := ../../target/release/libconversation.a

UNAME_S := $(shell uname -s)
ifeq ($(UNAME_S),Darwin)
  LDFLAGS := -dynamiclib -undefined dynamic_lookup
  # macOS system frameworks needed by Rust stdlib + dependencies
  RUST_LDFLAGS := -framework Security -framework CoreFoundation -lSystem -lm -liconv
else
  LDFLAGS := -shared
  RUST_LDFLAGS := -lm -ldl -lpthread
endif

GFORTRAN_LIB := $(shell gfortran -print-file-name=libgfortran.dylib 2>/dev/null | xargs dirname)
ifeq ($(GFORTRAN_LIB),)
  GFORTRAN_LIB := $(shell gfortran -print-file-name=libgfortran.so | xargs dirname)
endif

.PHONY: all clean prism-nif conv-nif rust-lib

all: prism-nif conv-nif

# ---- Prism NIF (Fortran) ----

prism-nif: $(PRISM_TARGET)

prism.o: prism.f90
	gfortran -O2 -fPIC -c -o $@ $<

prism_nif.o: prism_nif.c
	cc -O2 -fPIC -I$(ERL_INCLUDE) -c -o $@ $<

$(PRISM_TARGET): prism.o prism_nif.o | $(PRIV_DIR)
	cc $(LDFLAGS) -L$(GFORTRAN_LIB) -lgfortran -o $@ $^

# ---- Conversation NIF (Rust) ----

conv-nif: $(CONV_TARGET)

rust-lib: $(RUST_LIB)

$(RUST_LIB):
	cd ../.. && cargo build --release

conversation_nif.o: conversation_nif.c
	cc -O2 -fPIC -I$(ERL_INCLUDE) -c -o $@ $<

$(CONV_TARGET): conversation_nif.o $(RUST_LIB) | $(PRIV_DIR)
	cc $(LDFLAGS) -o $@ conversation_nif.o $(RUST_LIB) $(RUST_LDFLAGS)

# ---- Common ----

$(PRIV_DIR):
	mkdir -p $(PRIV_DIR)

clean:
	rm -f *.o *.mod $(PRISM_TARGET) $(CONV_TARGET)
