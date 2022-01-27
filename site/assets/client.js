(function () {
    document.addEventListener('DOMContentLoaded', function () {
        // make drop-down only forms work automatically when
        // JS is enabled, but also ensure that the site works 100% without
        // the need for JS
        const form_el = document.querySelector("header form#site_header_locale_form");
        if (form_el) {
            const submit_el = form_el.querySelector("input.select");
            const select_el = form_el.querySelector("select#locale");
            if (submit_el && select_el) {
                submit_el.style.visibility = "hidden";
                select_el.onchange = function () {
                    form_el.submit();
                };
            }
        }
    });
})();