#include <iostream>
#include <string>
#include <QApplication>
#include "WTINetGui.hpp"
#include "WTIDLMS.hpp"
#include "HTTPOWTIN.hpp"
#include "MLMLS.hpp"

int main(int argc, char *argv[]) {
    QApplication app(argc, argv);
    WTINetGui gui;

    // Example usage of WTIDLMS, HTTPOWTIN, MLMLS
    WTIDLMS::registerPeer("User1");
    std::string address = WTIDLMS::getPeerAddress("User1");
    std::cout << "Generated address for User1: " << address << std::endl;

    // Display HTML content in the GUI
    HTTPOWTIN httpowtin;
    std::string htmlData = httpowtin.prepareHtmlData("<h1>Hello WTINet!</h1>");
    gui.setHtml(QString::fromStdString(htmlData));

    // Set initial status
    gui.setStatus("Connected");

    // Display GUI
    gui.show();
    return app.exec();
}
