#ifndef MLMLS_HPP
#define MLMLS_HPP

#include <string>
#include <iostream>

class MLMLS {
public:
    std::string requestData(const std::string &resource) {
        return "MLMLS_REQUEST:" + resource;
    }

    void receiveData(const std::string &data) {
        if (data.rfind("MLMLS_REQUEST:", 0) == 0) {
            std::cout << "Received MLMLS data for resource: " << data.substr(14) << std::endl;
        } else {
            std::cout << "Invalid data format received on MLMLS." << std::endl;
        }
    }
};

#endif
