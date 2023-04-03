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
   - https://site.com/xyz -> /site.com/xyz
   - /xyz -> /site.com/xyz
   - (not poc) is there risk of collisions?
- (not poc) break har into files (?)
- (not poc) methods for rewriting sensitive information
- (not poc) sha checks, domain checks...?
- (not poc) filter unnecessary files from har
- (not poc) save duplicate content only once

## Server ##
- match based on url path, method
- not poc: match based on request body, based on get params, based on cookies... configurable by request (extend har format?)

# Tutorials #
## How to deploy mirror of the webpage ##
Presumptions:
- you have mirrored site.com
- want to deplay mirror at domain mirrorsite.com. 
- You have jsrack server instance running at localhost:8000/

nginx
```
{
  listen 80;
  server_name mirrorsite.com;
  
  location /site.com {
    // todo remove site.com suffix from path and redirect user to mirrorsite.com
  }
  
  location {
    // todo add site.com to path suffix and pass that to jsrack instance
  }
}
```

### Mirroring multiple domains ###
Presumptions:
- you have mirrored site.com and their forum bbs.site.com (second domain can be arbitrary, doesn't need to be subdomain)
- want to deplay mirror at domain mirrorsite.com and bbs.mirrorsite.com
- You have jsrack server instance running at localhost:8000/

TODO

## Including your own dynamic content to the mirrored website ##
Ways to implement this:
- proxy server which intercepts requests to jstrack server and populates/replaces it with required data
- import jsrack as default middleware into some http routing libraries (not very programming language agnostic)

