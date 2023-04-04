/**
 * This code needs to be injected to html served
 */

/**
 * This code rewrites all href attributes (later: urls) in html 
 * (also dynamically created) to point to jsrack
 * 
 */

const documentElement = document.documentElement

/**
 * Rewrite all hrefs in target
 * 
 * @param {HTMLElement} target
 */
const rewrite_HTMLElement_urls = (target) => {
  target.querySelectorAll('[href]').forEach(element => {
    element.href = rewrite_url(element.href)
  });
}

// rewrite all url's in document
rewrite_HTMLElement_urls(documentElement) 
window.addEventListener('load', function() {
  rewrite_HTMLElement_urls(documentElement) // rewrite all url's in document
})

// register observer for rewriting url's in dynamically loaded content
const observerCallback = (mutationList, observer) => {
  for (const mutation of mutationList) {
    if (mutation.type == 'childList') {
      // TODO mutation.target is excessively wide selector
      rewrite_HTMLElement_urls(mutation.target) 
    }
  }
};

const observer = new MutationObserver(observerCallback);

const mutationsToObserve = { attributes: false, childList: true, subtree: true };

observer.observe(documentElement, mutationsToObserve);


/**
 * Intercept all fetch/xmrhttprequests and point them to jsrack instance
 * 
 */
  
var _XMLHttpRequest = XMLHttpRequest;



XMLHttpRequest = function() {
var xhr = new _XMLHttpRequest();

var _open = xhr.open;
xhr.open = function() {
    arguments[1] = rewrite_url(arguments[1])
    return _open.apply(this, arguments);
}

return xhr;
}

window.XMLHttpRequest = XMLHttpRequest

/**
 * UTIL
 */

/**
 * 
 * @param {String} url URL in string format
 * @returns {URL}
 */
const string_to_url_object = (url) => {
    try {
    return new URL(url)
    } catch {
    return new URL(url, location.origin)
    }
}

/**
 * Rewrites URLs (relative, absolute, with origin or not) to point to jstrack
 * 
 * @param {String} url 
 * @returns {String}
 */
const rewrite_url = (url) => {
	url = string_to_url_object(url)
    if (url.origin != location.origin) {
        // link is pointing to external resource, need to convert it to point to jstrack
        // TODO replace hostname with origin? (for http and https support)
        let new_url = `${location.origin}/${url.hostname}${url.pathname}${url.search}`
        return new_url
    } else {
        // link is already pointing at jsrack instance no need to do anything. 
        // jstrack should have default mirrorsite
        return url
    }
}
 