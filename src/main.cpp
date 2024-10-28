#include <iostream>
#include "WTIDLMS.hpp"
#include "HTTPOWTIN.hpp"
#include "MLMLS.hpp"

int main() {
    // Initialize protocol classes
    WTIDLMS wtidlms;
    HTTPOWTIN httpowtin;
    MLMLS mlmls;

    // Register and display a peer address
    wtidlms.registerPeer("Node1");
    std::string address = wtidlms.getPeerAddress("Node1");
    std::cout << "Generated WTIDLMS address for Node1: " << address << std::endl;

    // Prepare and send HTML data over HTTPOWTIN
    std::string htmlData = httpowtin.prepareHtmlData("<h1>Hello WTINet!</h1>");
    httpowtin.sendHtmlContent(htmlData);

    // Receive and display the parsed HTML content
    httpowtin.receiveHtmlContent(htmlData);

    // Request and receive data over MLMLS
    std::string request = mlmls.requestData("exampleResource");
    std::cout << "Requesting data for: " << request << std::endl;
    mlmls.receiveData(request);

    return 0;
}
