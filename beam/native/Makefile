# beam/native/Makefile — Build Fortran prism operations as Erlang NIF.
#
# Usage: nix develop -c make -C beam/native

ERL_INCLUDE := $(shell erl -noshell -eval 'io:format("~s/erts-~s/include", [code:root_dir(), erlang:system_info(version)])' -s erlang halt)

PRIV_DIR := ../priv
TARGET := $(PRIV_DIR)/conversation_prism_nif.so

UNAME_S := $(shell uname -s)
ifeq ($(UNAME_S),Darwin)
  LDFLAGS := -dynamiclib -undefined dynamic_lookup
else
  LDFLAGS := -shared
endif

GFORTRAN_LIB := $(shell gfortran -print-file-name=libgfortran.dylib 2>/dev/null | xargs dirname)
ifeq ($(GFORTRAN_LIB),)
  GFORTRAN_LIB := $(shell gfortran -print-file-name=libgfortran.so | xargs dirname)
endif

.PHONY: all clean

all: $(TARGET)

prism.o: prism.f90
	gfortran -O2 -fPIC -c -o $@ $<

prism_nif.o: prism_nif.c
	cc -O2 -fPIC -I$(ERL_INCLUDE) -c -o $@ $<

$(TARGET): prism.o prism_nif.o | $(PRIV_DIR)
	cc $(LDFLAGS) -L$(GFORTRAN_LIB) -lgfortran -o $@ $^

$(PRIV_DIR):
	mkdir -p $(PRIV_DIR)

clean:
	rm -f *.o *.mod $(TARGET)
