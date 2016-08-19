AM_CPPFLAGS = @VMOD_INCLUDES@ -Wall -Werror

vmoddir = @VMOD_DIR@
vmod_LTLIBRARIES = libvmod_woothee.la

libvmod_woothee_la_LDFLAGS = -module -export-dynamic -avoid-version -shared

libvmod_woothee_la_LIBADD = ../target/release/libwoothee.a

libvmod_woothee_la_LDFLAGS += -Wc,../target/release/libwoothee.a -lpthread -lm -ldl

../target/release/libwoothee.a: woothee.rs
	cargo build --release

nodist_libvmod_woothee_la_SOURCES = \
	vcc_if.c \
	vcc_if.h

vmod_woothee.lo: vcc_if.c vcc_if.h

vcc_if.c: vcc_if.h

vcc_if.h: @VMODTOOL@ $(top_srcdir)/src/vmod_woothee.vcc
	@VMODTOOL@ $(top_srcdir)/src/vmod_woothee.vcc

VMOD_TESTS = $(top_srcdir)/src/tests/*.vtc
.PHONY: $(VMOD_TESTS)

$(top_srcdir)/src/tests/*.vtc: libvmod_woothee.la
	@VARNISHTEST@ -Dvarnishd=@VARNISHD@ -Dvmod_topbuild=$(abs_top_builddir) $@

check: $(VMOD_TESTS)

EXTRA_DIST = \
	vmod_woothee.vcc \
	$(VMOD_TESTS)

CLEANFILES = \
	$(builddir)/vcc_if.c \
	$(builddir)/vcc_if.h \
	$(builddir)/vmod_woothee.rst \
	$(builddir)/vmod_woothee.man.rst
