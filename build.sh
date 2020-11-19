#!/bin/bash
make build-capi &&
make package-capi &&
/bin/cp package/include/wasmer.h /usr/include/ &&
/bin/cp package/include/wasmer.hh /usr/include/ &&
/bin/cp package/lib/libwasmer.so /usr/lib/ &&
/bin/cp package/lib/libwasmer.a /usr/lib/ &&
echo done!
