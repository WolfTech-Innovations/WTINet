#ifndef WTIDLMS_HPP
#define WTIDLMS_HPP

#include <string>
#include <unordered_map>

class WTIDLMS {
public:
    // Simple mapping for WTIDLMS address structure
    static std::string generateDeviceAddress(const std::string &deviceName);
    
    // Storage for peer-to-address mappings
    static std::unordered_map<std::string, std::string> peerMap;
    
    static void registerPeer(const std::string &peerName);
    static std::string getPeerAddress(const std::string &peerName);
};

#endif
