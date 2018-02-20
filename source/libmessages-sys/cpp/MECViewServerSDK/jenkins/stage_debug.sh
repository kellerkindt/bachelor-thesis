#!/bin/sh

cd build

if [ ! -d MECViewServerSDK ]; then
	mkdir MECViewServerSDK
fi

cd MECViewServerSDK

if [ -d debug ]; then
	rm -r debug
fi
mkdir debug
cd debug

cmake -DCMAKE_BUILD_TYPE=Debug ../../../MECViewServerSDK
make -j3 all
make install
