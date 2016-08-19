vmod_woothee
============

Varnish Woothee(User-Agent parser) Module

INSTALLATION
------------

::

    ./autogen.sh
    ./configure
    make && make install

EXAMPELS
--------

::

    import woothee;

    sub vcl_recv {
        if (woothee.is_crawler(req.http.User-Agent)) {
            std.log(req.http.User-Agent + " is crawler");
        } else {
            std.log(req.http.User-Agent + " is not crawler");
        }
    }

