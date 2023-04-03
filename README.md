# jstrack
Alternative for httrack

# concept #
## Client ##
- Responsible for visiting webpages and fetching raw data.
- cli tool
- run selenium and collect har of webpage
- (not poc) make rules for which urls to visit automatically
- (not poc) prompt user to fill in captchas

## Archiver ##
- rewrite domains in har content
- break har into files (?)
- (not poc) attempt rewrite sensitive information
- (not poc) sha checks, domain checks...?
- (not poc) filter unnecessary files from har
- (not poc) save duplicate content only once

## Server webpage ##
- match based on url path, method
- not poc: match based on request body, based on get params, based on cookies... configurable by request (extend har format?)

# Tutorials #
## How to deploy mirror of the webpage ##
TODO

## Including your own dynamic content to the mirrored website ##
Ways to implement this:
- proxy server which intercepts requests to jstrack server and populates/replaces it with required data
- import jsrack as default middleware into some http routing libraries (not very programming language agnostic)
-
