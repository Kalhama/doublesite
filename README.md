# Doublesite web mirroring tool
## Introduction ## 
Doublesite web mirroring tool is CLI tool for serving mirrors including dynamically loaded content.

Doublesite web mirroring tool shares similar goals with popular applications `wget` and `httrack` but takes a step further by preserving also dynamically loaded content that is not included in the original page load.

This type of content is becoming more and more dominant with client side rendering is becoming more popular.

## Installation ##
1. [Install rust](https://rustup.rs/)
2. Run `cargo install --git https://github.com/Kalhama/doublesite.git`
    - please mind that the software is still in under contruction and api is excepted to break
3. Run `doublesite --help`

## Usage ##
1. Get HAR (HTTP ARchive) of website(s) you want to backup
    - [Tutorial from microsoft](https://learn.microsoft.com/en-us/azure/azure-portal/capture-browser-trace) on how to get har file
    - This tool cannot (yet) generate HAR file [Discussion #16](https://github.com/Kalhama/doublesite/discussions/16)
2. Run `doublesite website.har`
3. Navigate to http://127.0.0.1:8080 on yoor browser to view the mirror
    - TODO load content through mirror by default [#1](https://github.com/Kalhama/doublesite/issues/1) and [#2](https://github.com/Kalhama/doublesite/issues/2)

## Features ##
- Serve mirror of webpage with dynamic content from HAR file

## Contributing & support ##
This project follows [semver] and [conventional commits]. 

Pull requests, bug reports, questions and feature requests: [Github issues](https://github.com/Kalhama/doublesite/issues)
