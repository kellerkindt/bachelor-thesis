# Overview
## Introduction

The MECViewServerSDK provides all needed information to write a client for the MECViewServer and to implement an extension for the MECViewServer. For these two purposes, the MECViewServerSDK contains two different interfaces. First, all ASN.1 files are stored in this project. A library with all the needed messages can be created. This library can be used in other C/C++ client to communicate with the MECViewServer. The second part is a collection of interfaces needed for extending the MECViewServer. The MECViewServer has two extension points: the fusion and prediction algorithm interface and the road clearance module interface, as well as general extension interface. These interfaces can be used to provide different implementations in binary form for the extension points. The MECViewServer can be linked against the provided libraries and the implemented functionality is used during the MECViewServer execution.

## The SDK build process

To build the message library and provide the corresponding interfaces for the extensions a cmake build process is provided with the SDK. The first step is to create a build directory. This can be done using the script "generate_eclipse_project.sh" in the "MECViewServerSDK" directory. This will create a new folder named "MECViewServerSDK-Build" on the same level as the "MECViewServerSDK" folder. To build the messages library change into the newly created folder and execute "make". After the make process succeeded, all files can be installed. To execute the installation run "make install". This will add a "lib" and "include" folder in the "MECViewServerSDK-Build" folder.

## The message library

The message library provides all C-struct to be able to communicate with the MECViewServer. These C-files are generated using the asn1c compiler and packaged into the static library libasn1_message.a. The corresponding include files can be found in the "include/proto" folder of the install path. The documentation of the C-structs and corresponding MACROS for the handling can be found in the compilers git repository https://github.com/vlm/asn1c in the doc directory.

To use the messages library, just link your executable against the libasn1_message.a library in the "lib/" path of the install directory and add the "include/proto" directory to your include directories. Important is, that this folder is included directly without prefix, because the asn1c generated files don't use a prefix for the internal includes.

## The extension interface

The starting point for all implementations of a fusion and predection algorithm or a road clearance module is the \ref mec.extension.Extension "mec::extension::Extension" interface. This interface can be found in the "include/extension" folder in the install path. All instances of an extension are started in a seperate thread in the MECViewServer with the \ref mec.extension.Extension.Run "Run()" operation as the threads entry point. If the \ref mec.extension.Extension.Run "Run()" returns, the thread will be stopped. The behavior is equal to the normal "main()" method behavior of a C/C++ application. If the \ref mec.extension.Extension.Stop "Stop()" operation is called, the extension must make sure, that the \ref mec.extension.Extension.Run "Run()" operation returns as fast as possible. This is used to shut down the MECViewServer properly without undefined behavior or corrupted data files.

### Implementing a fusion and prediction algorithm

To provide an algorithm implementation the interface \ref mec.extension.Extension "mec::extension::Extension" must be implemented. To give the MECViewServer the opportunity to create an instance of the algorithm implementation the interface \ref mec.algorithm.AlgorithmFactory "mec::algorithm::AlgorithmFactory" must be provided, too. The implementation of these two interfaces can be packed into a shared or static library. The header files for the two implementation must also be provided.

A sample implementation of an algorithm extension is provided by IT-Designers. The SampleAlgorithm is published together with the MECViewServer-TestApplication (version >= 2.0).

### Implementing a road clearance module

To provide a road clearance module implementation the interface \ref mec.extension.Extension "mec::extension::Extension" must be implemented. To give the MECViewServer the opportunity to create an instance of the road clearance module implementation the interface \ref mec.road_clearance.RoadClearanceModuleFactory "mec::road_clearance::RoadClearanceModuleFactory" must be provided, too. The implementation of these two interfaces can be packed into a shared or static library. The header files for the two implementation must also be provided.

A sample implementation of a road clearance module extension is provided by IT-Designers. The SampleRoadClearanceModule is published together with the MECViewServer-TestApplication (version >= 2.0).

# LICENSE INFORMATION
## asn1c

Copyright (c) 2003-2017  Lev Walkin <vlm@lionet.info> and contributors.
All rights reserved.

Redistribution and use in source and binary forms, with or without
modification, are permitted provided that the following conditions
are met:
1. Redistributions of source code must retain the above copyright
   notice, this list of conditions and the following disclaimer.
2. Redistributions in binary form must reproduce the above copyright
   notice, this list of conditions and the following disclaimer in the
   documentation and/or other materials provided with the distribution.

THIS SOFTWARE IS PROVIDED BY THE AUTHOR AND CONTRIBUTORS ``AS IS'' AND
ANY EXPRESS OR IMPLIED WARRANTIES, INCLUDING, BUT NOT LIMITED TO, THE
IMPLIED WARRANTIES OF MERCHANTABILITY AND FITNESS FOR A PARTICULAR PURPOSE
ARE DISCLAIMED.  IN NO EVENT SHALL THE AUTHOR OR CONTRIBUTORS BE LIABLE
FOR ANY DIRECT, INDIRECT, INCIDENTAL, SPECIAL, EXEMPLARY, OR CONSEQUENTIAL
DAMAGES (INCLUDING, BUT NOT LIMITED TO, PROCUREMENT OF SUBSTITUTE GOODS
OR SERVICES; LOSS OF USE, DATA, OR PROFITS; OR BUSINESS INTERRUPTION)
HOWEVER CAUSED AND ON ANY THEORY OF LIABILITY, WHETHER IN CONTRACT, STRICT
LIABILITY, OR TORT (INCLUDING NEGLIGENCE OR OTHERWISE) ARISING IN ANY WAY
OUT OF THE USE OF THIS SOFTWARE, EVEN IF ADVISED OF THE POSSIBILITY OF
SUCH DAMAGE.

