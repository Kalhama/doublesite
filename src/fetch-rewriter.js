/**
 * This code intercepts all fetch/xmrhttprequests to point to jsrack instance
 * 
 * This needs to be injected to html's served
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
    return new URL(url, location.hostname)
  }
}

var _XMLHttpRequest = XMLHttpRequest;

const rewrite_url = (request_url) => {
  request_url = string_to_url_object(request_url)
  
  if (request_url.origin != location.origin) {
    // TODO request_url.hostname => request_url.origin ??
    let new_url = `${location.origin}/${request_url.hostname}${request_url.pathname}${request_url.search}`
    return new_url
  }
}

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