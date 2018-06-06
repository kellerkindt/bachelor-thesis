#!/bin/sh

cd build

if [ ! -d MECViewServerSDK ]; then
        mkdir MECViewServerSDK
fi

cd MECViewServerSDK

if [ -d release ]; then
	rm -r release
fi
mkdir release
cd release

cmake -DCMAKE_BUILD_TYPE=Release ../../../MECViewServerSDK
make -j3 all
make install
