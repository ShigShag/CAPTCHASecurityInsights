// Object intercept
// Object.defineProperty(navigator,"webdriver",{get:()=>!0}),console.log(navigator.webdriver);

(function () {
    // Function to intercept and log accesses for any object and property
    function interceptAndLogAccess(obj, propertyName) {
        const originalProperty = Object.getOwnPropertyDescriptor(obj, propertyName);

        if (originalProperty && originalProperty.get) {
            Object.defineProperty(obj, propertyName, {
                get: function () {
                    const message = `${propertyName} was accessed ---> ${new Error().stack}`;
                    const message_2 = `${propertyName} was accessed `;

                    console.log(message_2);

                    return originalProperty.get.call(this);
                },
                enumerable: originalProperty.enumerable,
                configurable: originalProperty.configurable
            });
        }
    }

    // Check all properties of an object
    function interceptAndLogAllAccess(obj) {
        Object.getOwnPropertyNames(obj).forEach(propertyName => {
            interceptAndLogAccess(obj, propertyName);
        });

    }

    interceptAndLogAllAccess(Navigator.prototype);
    interceptAndLogAllAccess(Window.prototype);
    interceptAndLogAllAccess(Document.prototype);
    interceptAndLogAllAccess(HTMLCanvasElement.prototype);
    interceptAndLogAllAccess(WebGLRenderingContext.prototype);
    interceptAndLogAllAccess(Event.prototype);
    interceptAndLogAllAccess(AudioContext.prototype);
})();

// Dom mutation
// (function () {
//     function initObserver() {
//         const observer = new MutationObserver(mutations => {
//             mutations.forEach(mutation => {
//                 console.log('DOM mutation observed:', mutation);
//                 console.trace();
//             });
//         });
//         observer.observe(document.body, { attributes: true, childList: true, subtree: true });
//     }

//     if (document.readyState === 'loading') {
//         document.addEventListener('DOMContentLoaded', initObserver);
//     } else {
//         initObserver();
//     }
// })();

// Fetch
(function () {
    const originalFetch = window.fetch;
    window.fetch = async function (...args) {
        console.log('Fetch called with:', args);
        const response = await originalFetch(...args);
        console.log('Fetch response:', response);
        console.trace();
        // Clone the response so you can read it and still allow the calling code to read it
        const clone = response.clone();
        clone.text().then(text => console.log('Fetch response body:', text));
        return response;
    };
})();

// Xml request
(function () {
    var originalOpen = XMLHttpRequest.prototype.open;
    XMLHttpRequest.prototype.open = function (method, url, async, user, pass) {
        console.log('Request made with XMLHttpRequest to URL:', url, 'Method:', method);
        this.addEventListener('load', function () {
            console.log('Response received from URL:', url, 'Status:', this.status);
        });
        originalOpen.call(this, method, url, async, user, pass);
    };
})();

// JQuery Ajax requests
(function () {
    if (window.jQuery) {
        var originalAjax = jQuery.ajax;
        jQuery.ajax = function (url, options) {
            if (typeof url === 'object') {
                options = url;
                url = undefined;
            }
            console.log('Request made with jQuery.ajax to URL:', options.url || url);
            var originalSuccess = options.success;
            options.success = function (data, textStatus, jqXHR) {
                console.log('Response received from URL:', options.url || url, 'Status:', jqXHR.status);
                if (originalSuccess) originalSuccess(data, textStatus, jqXHR);
            };
            return originalAjax.call(this, url, options);
        };
    }
})();

console.log("Loaded observer.js");
