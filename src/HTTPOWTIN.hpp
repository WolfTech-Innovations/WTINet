#ifndef HTTPOWTIN_HPP
#define HTTPOWTIN_HPP

#include <string>

class HTTPOWTIN {
public:
    // Encapsulates HTML content for WTINet transmission
    std::string prepareHtmlData(const std::string &htmlContent);
    std::string parseReceivedData(const std::string &data);

    // Transport HTML content
    void sendHtmlContent(const std::string &htmlContent);
    std::string receiveHtmlContent();
};

#endif
