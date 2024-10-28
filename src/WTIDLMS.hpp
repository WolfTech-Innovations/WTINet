#ifndef WTIDLMS_HPP
#define WTIDLMS_HPP

#include <string>
#include <unordered_map>

class WTIDLMS {
public:
    // Generates a WTIDLMS address based on the device name
    static std::string generateDeviceAddress(const std::string &deviceName) {
        return "WTI-" + deviceName + "-" + std::to_string(rand() % 10000); // basic ID generation
    }

    static void registerPeer(const std::string &peerName) {
        std::string address = generateDeviceAddress(peerName);
        peerMap[peerName] = address;
    }

    static std::string getPeerAddress(const std::string &peerName) {
        return peerMap[peerName];
    }

private:
    static std::unordered_map<std::string, std::string> peerMap;
};

// Initialize static variable
std::unordered_map<std::string, std::string> WTIDLMS::peerMap;

#endif
