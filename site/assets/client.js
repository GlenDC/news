(function() {
    // make drop-down only forms work automatically when
    // JS is enabled, but also ensure that the site works 100% without
    // the need for JS
    
    const el = document.querySelector("header input.select");
    if (el) {
        el.onchange = function() {
            document.getElementById("site_header_locale_form").submit();
        };
        el.style.visibility = "hidden";
    }
})();