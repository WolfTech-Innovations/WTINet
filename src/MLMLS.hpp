#ifndef MLMLS_HPP
#define MLMLS_HPP

#include <string>

class MLMLS {
public:
    std::string requestData(const std::string &resource);
    std::string receiveData(const std::string &data);
};

#endif
