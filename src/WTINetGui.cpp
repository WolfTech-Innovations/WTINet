#include "WTINetGui.hpp"
#include <QVBoxLayout>

WTINetGui::WTINetGui(QWidget *parent) : QWidget(parent) {
    QVBoxLayout *layout = new QVBoxLayout;
    statusLabel = new QLabel("Disconnected");
    htmlDisplay = new QTextEdit;
    
    layout->addWidget(new QLabel("Your Peer Status:"));
    layout->addWidget(statusLabel);
    layout->addWidget(new QLabel("HTML Displayed over WTINet:"));
    layout->addWidget(htmlDisplay);
    
    setLayout(layout);
}

void WTINetGui::setStatus(const QString &status) {
    statusLabel->setText(status);
}

void WTINetGui::setHtml(const QString &html) {
    htmlDisplay->setText(html);
}
