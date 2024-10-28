// HTTPOWTIN.hpp
#ifndef HTTPOWTIN_HPP
#define HTTPOWTIN_HPP

#include <string>

class HTTPOWTIN {
public:
    // Updated to accept an HTML data parameter
    std::string receiveHtmlContent(const std::string &htmlData);
};

#endif // HTTPOWTIN_HPP
