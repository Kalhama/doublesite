# jstrack
Alternative for httrack

# concept #
## (not poc) Client ##
- Responsible for visiting webpages and fetching raw data.
- cli tool
- run selenium and collect har of webpage
- (enchantment) make rules for which urls to visit automatically
- (enchantment) prompt user to fill in captchas
- (enchantment) change browser size, scroll page etc to ensure dynamic content loads
- (enchantment) ability for users to just give their own behavior client must run

## Archiver ##
- rewrite domains in har content 
   - https://site.com/xyz -> /site.com/xyz
   - /xyz -> /site.com/xyz
   - (not poc) is there risk of collisions?
- (enchantment) break har into files (?)
- (enchantment) methods for rewriting sensitive information
- (enchantment) sha checks, domain checks...?
- (enchantment) filter unnecessary files from har
- (enchantment) save duplicate content only once
- (enchantment) consider differenc file format from har

## Server ##
- match based on url path, method
- (enchantment): match based on request body, based on get params, based on cookies... configurable by request (extend har format?)
- (enchantment) serve multiple har files

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
Configure http-proxy for intercepting and mutating requests to jstrack
