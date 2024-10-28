// HTTPOWTIN.cpp
#include "HTTPOWTIN.hpp"
#include <iostream>

std::string HTTPOWTIN::prepareHtmlData(const std::string &htmlContent) {
    // Prepare or format HTML data as needed
    return "<html><body>" + htmlContent + "</body></html>";
}

void HTTPOWTIN::sendHtmlContent(const std::string &htmlData) {
    // Code to send HTML content over the WTINet protocol
    std::cout << "Sending HTML content over WTINet: " << htmlData << std::endl;
}

std::string HTTPOWTIN::receiveHtmlContent(const std::string &htmlData) {
    // Process and receive HTML content
    std::cout << "Receiving HTML content: " << htmlData << std::endl;
    return htmlData; // Simply returns the received data for now
}
