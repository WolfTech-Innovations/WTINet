// HTTPOWTIN.hpp
#ifndef HTTPOWTIN_HPP
#define HTTPOWTIN_HPP

#include <string>

class HTTPOWTIN {
public:
    // Function to prepare HTML data
    std::string prepareHtmlData(const std::string &htmlContent);

    // Function to send HTML content over WTINet
    void sendHtmlContent(const std::string &htmlData);

    // Function to receive HTML content, accepting HTML data as a parameter
    std::string receiveHtmlContent(const std::string &htmlData);
};

#endif // HTTPOWTIN_HPP
