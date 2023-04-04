/**
 * This code rewrites all href attributes (later: urls) in html 
 * (also dynamically created) to point to jsrack
 * 
 * This needs to be injected to html's served
 */

const documentElement = document.documentElement

/**
 * 
 * @param {String} hostname 
 * @returns {URL}
 */
const href_to_url = (hostname) => {
  try {
    return new URL(hostname)
  } catch {
    return new URL(hostname, location.hostname)
  }
}

/**
 * Rewrite all hrefs in target
 * 
 * @param {HTMLElement} target
 */
const rewrite_urls = (target) => {
  let location = document.location
  target.querySelectorAll('[href]').forEach(element => {
    let href_url = href_to_url(element.href)
      
    if (href_url.origin != location.origin) {
      // link is pointing to external resource, need to convert it to point to jstrack
      // TODO replace hostname with origin? (for http and https support)
      let new_url = `${location.origin}/${href_url.hostname}${href_url.pathname}${href_url.search}`
      element.href = new_url
    } 
    // else {
    //    link is already pointing at jsrack instance no need to do anything. 
    //    jstrack should have default mirrorsite
    // }  
  });
}

// rewrite all url's in document
rewrite_urls(documentElement) 
window.addEventListener('load', function() {
  rewrite_urls(documentElement) // rewrite all url's in document
})

// register observer for rewriting url's in dynamically loaded content
const observerCallback = (mutationList, observer) => {
  for (const mutation of mutationList) {
    if (mutation.type == 'childList') {
      // TODO mutation.target is excessively wide selector
      rewrite_urls(mutation.target) 
		}
  }
};

const observer = new MutationObserver(observerCallback);

const mutationsToObserve = { attributes: false, childList: true, subtree: true };

observer.observe(documentElement, mutationsToObserve);