#ifndef MECVIEWSERVERSDK_VERSION_H
#define MECVIEWSERVERSDK_VERSION_H

#include <string>

namespace mec {
namespace sdk {

const std::string VERSION_MAJOR = "0";
const std::string VERSION_MINOR = "1";
const std::string VERSION_PATCH = "0";
const std::string VERSION_SHORT = "0.1.0";

const std::string SVN_REVISION_NUMBER = "";

const std::string VERSION = "v" + VERSION_SHORT + "-" + SVN_REVISION_NUMBER;

}
}

#endif
