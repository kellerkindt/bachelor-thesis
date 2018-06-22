/* Accordion Function for SDG website, 27 Jan 2017 Added by Rajani Vepa */
$( function() {
    $( "#accordion-sdg" ).accordion();
    
     $("#accordion-sdg").show().accordion({
        autoHeight: false });

    $("#accordion-sdg div").css({ 'height': 'auto' });
    
    
  } );
  

 
/* ENDs here : Accordion Function for SDG website */


 
/* Adjust horizontal padding of the sub-nav items, and load dropdown menu (only ITU-R website) */
/* 2014-07-25 Added by Vincent Paroz */
function ITU_R_Tunings() {
    //current language:
    var strCrtLang = window.location.pathname.split('/')[1];
    //curent sector:
    var strCrtSect = window.location.pathname.split('/')[2];

    //if ITU-R website:
    if (strCrtSect == 'ITU-R') {
        //is on STAGING:
        var isStaging = (window.location.hostname.split('.')[0].toUpperCase() == 'STAGING');

        /* Check if webcast IFRAME - if yes: scroll to top on click */
        if ($("iframe#iframe").length > 0) {
            $("iframe#iframe").load(function () {
                $("iframe#iframe").contents().find("a.loadme").click(function () {
                    $("html").animate({ scrollTop: 0 }, "slow");
                });
            });
        };

        switch (strCrtLang) {
            case "fr":
                $('li.sub-nav-ddm').children('a').css({ 'padding': '10px 3px 0 16px' });
                break;
            case "es":
                $('li.sub-nav-ddm').children('a').css({ 'padding': '10px 5px 0 16px' });
                break;
            case "ar":
                $('li.sub-nav-ddm').children('a').css({ 'padding': '10px 15px 0 0px' });
                $('li.sub-nav-ddm').children('a').css({ 'font-size': '13px' });
                break;
            case "ru":
                $('li.sub-nav-ddm').children('a').css({ 'padding': '10px 0px 0 12px' });
                $('li.sub-nav-ddm').children('a').css({ 'font-size': '11px' });
                break;
            default:
                /*$('li.sub-nav-ddm').children('a').css({'font-size': '12px'});*/
        };
        $("head").append('<link rel="stylesheet" type="text/css" href="/en/ITU-R/ddmenu/ddmenu.css" />');
        $("head").append('<script type="text/javascript" src="/en/ITU-R/ddmenu/ddmenu.js"></script>');
        // 2015-02-27 ADD/FIX by Vincent Paroz: optimization of the CSS classes (overwriting those defined in itu-main.css)
        $("head").append('<link rel="stylesheet" type="text/css" href="/en/ITU-R/css/itu-r-main.css" />');

        /* Show hidden content from staging only -> as the "internal only" restriction of the old ITU-R CMS */
        /* 2014-10-03 ADD by Vincent Paroz */
        /* 2014-10-03 FIX by Vincent Paroz -> show on STAGING but also only if NOT in edit mode */
        if (isStaging == true) {
            var inDesignMode = document.forms[MSOWebPartPageFormName].MSOLayout_InDesignMode.value;
            if (inDesignMode != "1") {
                // page is NOT in edit mode
                $('div#InternalOnly').css('visibility', 'visible');
                $('div#InternalOnly').css('display', 'inline');
            };
        };
        /* Display page title on the entire width (col-32) if no image is set at the right of the header */
        /* 2014-05-25 ADD by Vincent Paroz */
        $('div#pagetitle>div').filter(function () {
            return $(this).is(':empty');
        }).remove();
        if (($('div#pagetitle div').find('img').length) == 0) {
            /*alert('title header does not contain an image');*/
            $('div#pagetitle>div.col-8').remove();
            $('div#pagetitle>div.col-24').removeClass("col-24").addClass("col-32");
        };
        /* Reduce space between breadcrumb levels */
        /* 2014-05-26 ADD by Vincent Paroz        */
        $('div#breadcrumb div[class^="col-2"] p a[href = "#ctl00_SiteMapPath1_SkipLink"]').remove();
        if (strCrtLang == "ar") {
            $('div#breadcrumb div[class^="col-2"] p a').css('margin-left', '5px');
            $('div#breadcrumb div[class^="col-2"] p a').css('margin-right', '0px');
        } else {
            $('div#breadcrumb div[class^="col-2"] p a').css('margin-right', '5px');
        };
        /* Set the breadcrumb layout as 2 divs side by side (text wrapping alignment) */
        /* 2014-05-26 ADD by Vincent Paroz */
        switch (strCrtLang) {
            case "ar":
            case "zh":
                $('div#breadcrumb div[class^="col-2"] p>em').css('width', '8%');
                break;
            case "es":
                $('div#breadcrumb div[class^="col-2"] p>em').css('width', '17%');
                break;
            default:
                $('div#breadcrumb div[class^="col-2"] p>em').css('width', '14%');
        };
        $('div#breadcrumb p span#ctl00_SiteMapPath1').css('display', 'block');
        switch (strCrtLang) {
            case "ar":
            case "zh":
                $('div#breadcrumb p span#ctl00_SiteMapPath1').css('width', '92%');
                break;
            case "es":
                $('div#breadcrumb p span#ctl00_SiteMapPath1').css('width', '83%');
                break;
            default:
                $('div#breadcrumb p span#ctl00_SiteMapPath1').css('width', '86%');
        };
        if (strCrtLang == "ar") {
            $('div#breadcrumb p span#ctl00_SiteMapPath1').css('float', 'left');
        } else {
            $('div#breadcrumb p span#ctl00_SiteMapPath1').css('float', 'right');
        };
        /*if last breadcrumb element contains '>' this means the previous element has been duplicated
		in order to provide a detailed page name (metadata used by the search engines), but this also
		means it can be removed from the last breadcrumb element*/
        var objLastBreadcrumbElem = $('div#breadcrumb p span#ctl00_SiteMapPath1 span').last();
        if (objLastBreadcrumbElem.text().indexOf('>') >= 0) {
            objLastBreadcrumbElem.html(objLastBreadcrumbElem.html().split('&gt;')[1].trim());
        };
    };
};

/* Function to find all nodes that DIRECTLY contain a specific text */
/* 2015-10-16 Added by Vincent Paroz */
function getNodesThatContain(text) {
    var textNodes = $(document).find(":not(iframe, script, style)").contents().filter(function () {
        return this.nodeType == 3 && this.textContent.indexOf(text) > -1;
    });
    return textNodes.parent();
};

$(document).ready(function () {


	// ITU-D Website Accordion - 24/05/2016 jaboulay
	if ($(".home-block .BDTMenu").length) { BDTMenuApply(); }


    // Tunings loaded only for ITU-R website:
    ITU_R_Tunings();

    if ($(".home-feature").length) { homepageFeature(); }
    if ($(".smallSlider").length) { SmallSliderFeature(); } //small Slider - Added by Rajani Vepa 07 May 2012
    if ($(".home-block .accordion").length) { homepageAccordion(); }
    if ($(".module-tabs").length) { moduleTabs(); }
    if ($(".back-to-top").length) { backToTop(); }
    if ($(".slider-careers").length) { sliderCareers(); }

    $(".hsearch-input").click(function () {
        $(this).attr("value", "").addClass("hsearch-input-active");
    });

    $(".home-feature-nav li a[title]").tooltip({ effect: 'fade' });

    function homepageFeature() {
        $('.home-feature-preview').css("opacity", "0");
        $('.home-feature').cycle({
            fx: 'fade', width: '900', fit: 'true', pause: 'true', timeout: '8000', speed: '2000', activePagerClass: 'activeSlide', pager: '.home-feature-nav ul',
            pagerAnchorBuilder: function (idx, slide) {
                // return selector string for existing anchor 
                return '.home-feature-nav li:eq(' + idx + ') a';
            }
        });
        $('.home-feature-nav li:eq(0) a').trigger('click');
    }



function testrajani() {
  
  alert('Welcome test by Rajani');
   
}

    //small slider start - Added by Rajani Vepa 07 May 2012

    function SmallSliderFeature() {
        $('.home-feature-preview').css("opacity", "0");
        $('.smallSlider').cycle({
            fx: 'fade', width: '640', fit: 'true', pause: 'true', pauseOnPagerHover: 'true', timeout: '8000', speed: '2000', activePagerClass: 'activeSlide', pager: '.home-feature-nav ul',
            pagerAnchorBuilder: function (idx, slide) {
                // return selector string for existing anchor 
                return '.home-feature-nav li:eq(' + idx + ') a';
            }
        });
        $('.home-feature-nav li:eq(0) a').trigger('click');

    }
    //small slider end

    //small slider modification - Added by Rajani Vepa 07 May 2012
    $(document).keydown(function (e) {
        if (e.keyCode == 37) {
            $(".home-feature").cycle('next');
            $(".smallSlider").cycle('next');

        } else if (e.keyCode == 39) {
            $(".home-feature").cycle('next');
            $(".smallSlider").cycle('next');
        }
    });
    //small slider modification ends here 





    function sliderCareers() {

        $(".slider-careers .scrollable .items").scrollable({ circular: true, speed: 2000 }).autoscroll({ autoplay: false, interval: 5000 });
    }

    function homepageAccordion() {
        //$(".accordion dl dd").slideUp("fast");
        //$(".accordion dl dt").toggleClass("accordion-open accordion-closed");	

        // Added by Rajani Vepa on 18 April 2012 to make accordian  collapsed at start //
        $(".accordion dl").accordion({ collapsible: true, autoHeight: false, navigation: true, active: false });


        $(".accordion dl").accordion({ header: 'dt', animated: "slide" });

        $('.accordion dl dt').click(function (event) {
            event.preventDefault();
            /*
            if($(this).hasClass("accordion-closed")){ //if this is shut
              $(this).next("dd").slideToggle("fast");
              $(this).toggleClass("accordion-open accordion-closed");
            } else if ($(this).hasClass("accordion-open")){ //if this is open
              $(this).next("dd").slideToggle("fast");
              $(this).toggleClass("accordion-open accordion-closed");
            }
            */
        });
    }


    function BDTMenuApply() {
        $(".BDTMenu dl").accordion({ collapsible: true, autoHeight: false, navigation: true, active:0});
        $(".BDTMenu dl").accordion({ header: 'dt', animated: "slide" });
        $('.BDTMenu dl dt').click(function (event) {
            event.preventDefault();
        });
    }


    function moduleTabs() {
        $(".module-tabs").tabs();
    }

    function backToTop() {
        $(".back-to-top").click(function () {
            $('body,html').animate({ scrollTop: 0 }, 1000, "easeInOutQuad");
            return false;
        });
    }

    function cleanEmptyElements2() {
    };

}); // end document.ready

/*!
 * jQuery Tools dev - The missing UI library for the Web
 * 
 * scrollable/scrollable.js
 * scrollable/scrollable.autoscroll.js
 * scrollable/scrollable.navigator.js
 * tooltip/tooltip.js
 * 
 * NO COPYRIGHTS OR LICENSES. DO WHAT YOU LIKE.
 * 
 * http://flowplayer.org/tools/
 * 
 */
(function (a) { a.tools = a.tools || { version: "dev" }, a.tools.scrollable = { conf: { activeClass: "active", circular: !1, clonedClass: "cloned", disabledClass: "disabled", easing: "swing", initialIndex: 0, item: "> *", items: ".items", keyboard: !0, mousewheel: !1, next: ".next", prev: ".prev", size: 1, speed: 400, vertical: !1, touch: !0, wheelSpeed: 0 } }; function b(a, b) { var c = parseInt(a.css(b), 10); if (c) return c; var d = a[0].currentStyle; return d && d.width && parseInt(d.width, 10) } function c(b, c) { var d = a(c); return d.length < 2 ? d : b.parent().find(c) } var d; function e(b, e) { var f = this, g = b.add(f), h = b.children(), i = 0, j = e.vertical; d || (d = f), h.length > 1 && (h = a(e.items, b)), e.size > 1 && (e.circular = !1), a.extend(f, { getConf: function () { return e }, getIndex: function () { return i }, getSize: function () { return f.getItems().size() }, getNaviButtons: function () { return n.add(o) }, getRoot: function () { return b }, getItemWrap: function () { return h }, getItems: function () { return h.find(e.item).not("." + e.clonedClass) }, move: function (a, b) { return f.seekTo(i + a, b) }, next: function (a) { return f.move(e.size, a) }, prev: function (a) { return f.move(-e.size, a) }, begin: function (a) { return f.seekTo(0, a) }, end: function (a) { return f.seekTo(f.getSize() - 1, a) }, focus: function () { d = f; return f }, addItem: function (b) { b = a(b), e.circular ? (h.children().last().before(b), h.children().first().replaceWith(b.clone().addClass(e.clonedClass))) : (h.append(b), o.removeClass("disabled")), g.trigger("onAddItem", [b]); return f }, seekTo: function (b, c, k) { b.jquery || (b *= 1); if (e.circular && b === 0 && i == -1 && c !== 0) return f; if (!e.circular && b < 0 || b > f.getSize() || b < -1) return f; var l = b; b.jquery ? b = f.getItems().index(b) : l = f.getItems().eq(b); var m = a.Event("onBeforeSeek"); if (!k) { g.trigger(m, [b, c]); if (m.isDefaultPrevented() || !l.length) return f } var n = j ? { top: -l.position().top } : { left: -l.position().left }; i = b, d = f, c === undefined && (c = e.speed), h.animate(n, c, e.easing, k || function () { g.trigger("onSeek", [b]) }); return f } }), a.each(["onBeforeSeek", "onSeek", "onAddItem"], function (b, c) { a.isFunction(e[c]) && a(f).bind(c, e[c]), f[c] = function (b) { b && a(f).bind(c, b); return f } }); if (e.circular) { var k = f.getItems().slice(-1).clone().prependTo(h), l = f.getItems().eq(1).clone().appendTo(h); k.add(l).addClass(e.clonedClass), f.onBeforeSeek(function (a, b, c) { if (!a.isDefaultPrevented()) { if (b == -1) { f.seekTo(k, c, function () { f.end(0) }); return a.preventDefault() } b == f.getSize() && f.seekTo(l, c, function () { f.begin(0) }) } }); var m = b.parents().add(b).filter(function () { if (a(this).css("display") === "none") return !0 }); m.length ? (m.show(), f.seekTo(0, 0, function () { }), m.hide()) : f.seekTo(0, 0, function () { }) } var n = c(b, e.prev).click(function () { f.prev() }), o = c(b, e.next).click(function () { f.next() }); e.circular || (f.onBeforeSeek(function (a, b) { setTimeout(function () { a.isDefaultPrevented() || (n.toggleClass(e.disabledClass, b <= 0), o.toggleClass(e.disabledClass, b >= f.getSize() - 1)) }, 1) }), e.initialIndex || n.addClass(e.disabledClass)), f.getSize() < 2 && n.add(o).addClass(e.disabledClass), e.mousewheel && a.fn.mousewheel && b.mousewheel(function (a, b) { if (e.mousewheel) { f.move(b < 0 ? 1 : -1, e.wheelSpeed || 50); return !1 } }); if (e.touch) { var p = {}; h[0].ontouchstart = function (a) { var b = a.touches[0]; p.x = b.clientX, p.y = b.clientY }, h[0].ontouchmove = function (a) { if (a.touches.length == 1 && !h.is(":animated")) { var b = a.touches[0], c = p.x - b.clientX, d = p.y - b.clientY; f[j && d > 0 || !j && c > 0 ? "next" : "prev"](), a.preventDefault() } } } e.keyboard && a(document).bind("keydown.scrollable", function (b) { if (!(!e.keyboard || b.altKey || b.ctrlKey || b.metaKey || a(b.target).is(":input"))) { if (e.keyboard != "static" && d != f) return; var c = b.keyCode; if (j && (c == 38 || c == 40)) { f.move(c == 38 ? -1 : 1); return b.preventDefault() } if (!j && (c == 37 || c == 39)) { f.move(c == 37 ? -1 : 1); return b.preventDefault() } } }), e.initialIndex && f.seekTo(e.initialIndex, 0, function () { }) } a.fn.scrollable = function (b) { var c = this.data("scrollable"); if (c) return c; b = a.extend({}, a.tools.scrollable.conf, b), this.each(function () { c = new e(a(this), b), a(this).data("scrollable", c) }); return b.api ? c : this } })(jQuery);
(function (a) { var b = a.tools.scrollable; b.autoscroll = { conf: { autoplay: !0, interval: 3e3, autopause: !0 } }, a.fn.autoscroll = function (c) { typeof c == "number" && (c = { interval: c }); var d = a.extend({}, b.autoscroll.conf, c), e; this.each(function () { var b = a(this).data("scrollable"); b && (e = b); var c, f = !0; b.play = function () { c || (f = !1, c = setInterval(function () { b.next() }, d.interval)) }, b.pause = function () { c = clearInterval(c) }, b.stop = function () { b.pause(), f = !0 }, d.autopause && b.getRoot().add(b.getNaviButtons()).hover(b.pause, b.play), d.autoplay && b.play() }); return d.api ? e : this } })(jQuery);
(function (a) { var b = a.tools.scrollable; b.navigator = { conf: { navi: ".navi", naviItem: null, activeClass: "active", indexed: !1, idPrefix: null, history: !1 } }; function c(b, c) { var d = a(c); return d.length < 2 ? d : b.parent().find(c) } a.fn.navigator = function (d) { typeof d == "string" && (d = { navi: d }), d = a.extend({}, b.navigator.conf, d); var e; this.each(function () { var b = a(this).data("scrollable"), f = d.navi.jquery ? d.navi : c(b.getRoot(), d.navi), g = b.getNaviButtons(), h = d.activeClass, i = d.history && history.pushState, j = b.getConf().size; b && (e = b), b.getNaviButtons = function () { return g.add(f) }, i && (history.pushState({ i: 0 }), a(window).bind("popstate", function (a) { var c = a.originalEvent.state; c && b.seekTo(c.i) })); function k(a, c, d) { b.seekTo(c), d.preventDefault(), i && history.pushState({ i: c }) } function l() { return f.find(d.naviItem || "> *") } function m(b) { var c = a("<" + (d.naviItem || "a") + "/>").click(function (c) { k(a(this), b, c) }); b === 0 && c.addClass(h), d.indexed && c.text(b + 1), d.idPrefix && c.attr("id", d.idPrefix + b); return c.appendTo(f) } l().length ? l().each(function (b) { a(this).click(function (c) { k(a(this), b, c) }) }) : a.each(b.getItems(), function (a) { a % j == 0 && m(a) }), b.onBeforeSeek(function (a, b) { setTimeout(function () { if (!a.isDefaultPrevented()) { var c = b / j, d = l().eq(c); d.length && l().removeClass(h).eq(c).addClass(h) } }, 1) }), b.onAddItem(function (a, c) { var d = b.getItems().index(c); d % j == 0 && m(d) }) }); return d.api ? e : this } })(jQuery);
(function (a) { a.tools = a.tools || { version: "dev" }, a.tools.tooltip = { conf: { effect: "toggle", fadeOutSpeed: "fast", predelay: 0, delay: 30, opacity: 1, tip: 0, fadeIE: !1, position: ["top", "center"], offset: [0, 0], relative: !1, cancelDefault: !0, events: { def: "mouseenter,mouseleave", input: "focus,blur", widget: "focus mouseenter,blur mouseleave", tooltip: "mouseenter,mouseleave" }, layout: "<div/>", tipClass: "tooltip" }, addEffect: function (a, c, d) { b[a] = [c, d] } }; var b = { toggle: [function (a) { var b = this.getConf(), c = this.getTip(), d = b.opacity; d < 1 && c.css({ opacity: d }), c.show(), a.call() }, function (a) { this.getTip().hide(), a.call() }], fade: [function (b) { var c = this.getConf(); !a.browser.msie || c.fadeIE ? this.getTip().fadeTo(c.fadeInSpeed, c.opacity, b) : (this.getTip().show(), b()) }, function (b) { var c = this.getConf(); !a.browser.msie || c.fadeIE ? this.getTip().fadeOut(c.fadeOutSpeed, b) : (this.getTip().hide(), b()) }] }; function c(b, c, d) { var e = d.relative ? b.position().top : b.offset().top, f = d.relative ? b.position().left : b.offset().left, g = d.position[0]; e -= c.outerHeight() - d.offset[0], f += b.outerWidth() + d.offset[1], /iPad/i.test(navigator.userAgent) && (e -= a(window).scrollTop()); var h = c.outerHeight() + b.outerHeight(); g == "center" && (e += h / 2), g == "bottom" && (e += h), g = d.position[1]; var i = c.outerWidth() + b.outerWidth(); g == "center" && (f -= i / 2), g == "left" && (f -= i); return { top: e, left: f } } function d(d, e) { var f = this, g = d.add(f), h, i = 0, j = 0, k = d.attr("title"), l = d.attr("data-tooltip"), m = b[e.effect], n, o = d.is(":input"), p = o && d.is(":checkbox, :radio, select, :button, :submit"), q = d.attr("type"), r = e.events[q] || e.events[o ? p ? "widget" : "input" : "def"]; if (!m) throw "Nonexistent effect \"" + e.effect + "\""; r = r.split(/,\s*/); if (r.length != 2) throw "Tooltip: bad events configuration for " + q; d.bind(r[0], function (a) { clearTimeout(i), e.predelay ? j = setTimeout(function () { f.show(a) }, e.predelay) : f.show(a) }).bind(r[1], function (a) { clearTimeout(j), e.delay ? i = setTimeout(function () { f.hide(a) }, e.delay) : f.hide(a) }), k && e.cancelDefault && (d.removeAttr("title"), d.data("title", k)), a.extend(f, { show: function (b) { if (!h) { l ? h = a(l) : e.tip ? h = a(e.tip).eq(0) : k ? h = a(e.layout).addClass(e.tipClass).appendTo(document.body).hide().append(k) : (h = d.next(), h.length || (h = d.parent().next())); if (!h.length) throw "Cannot find tooltip for " + d } if (f.isShown()) return f; h.stop(!0, !0); var o = c(d, h, e); e.tip && h.html(d.data("title")), b = a.Event(), b.type = "onBeforeShow", g.trigger(b, [o]); if (b.isDefaultPrevented()) return f; o = c(d, h, e), h.css({ position: "absolute", top: o.top, left: o.left }), n = !0, m[0].call(f, function () { b.type = "onShow", n = "full", g.trigger(b) }); var p = e.events.tooltip.split(/,\s*/); h.data("__set") || (h.unbind(p[0]).bind(p[0], function () { clearTimeout(i), clearTimeout(j) }), p[1] && !d.is("input:not(:checkbox, :radio), textarea") && h.unbind(p[1]).bind(p[1], function (a) { a.relatedTarget != d[0] && d.trigger(r[1].split(" ")[0]) }), e.tip || h.data("__set", !0)); return f }, hide: function (c) { if (!h || !f.isShown()) return f; c = a.Event(), c.type = "onBeforeHide", g.trigger(c); if (!c.isDefaultPrevented()) { n = !1, b[e.effect][1].call(f, function () { c.type = "onHide", g.trigger(c) }); return f } }, isShown: function (a) { return a ? n == "full" : n }, getConf: function () { return e }, getTip: function () { return h }, getTrigger: function () { return d } }), a.each("onHide,onBeforeShow,onShow,onBeforeHide".split(","), function (b, c) { a.isFunction(e[c]) && a(f).bind(c, e[c]), f[c] = function (b) { b && a(f).bind(c, b); return f } }) } a.fn.tooltip = function (b) { var c = this.data("tooltip"); if (c) return c; b = a.extend(!0, {}, a.tools.tooltip.conf, b), typeof b.position == "string" && (b.position = b.position.split(/,?\s/)), this.each(function () { c = new d(a(this), b), a(this).data("tooltip", c) }); return b.api ? c : this } })(jQuery);

/*
 * jQuery Cycle Plugin (with Transition Definitions)
 * Examples and documentation at: http://jquery.malsup.com/cycle/
 * Copyright (c) 2007-2010 M. Alsup
 * Version: 2.94 (20-DEC-2010)
 * Dual licensed under the MIT and GPL licenses.
 * http://jquery.malsup.com/license.html
 * Requires: jQuery v1.2.6 or later
 */
(function ($) { var ver = "2.94"; if ($.support == undefined) { $.support = { opacity: !($.browser.msie) }; } function debug(s) { if ($.fn.cycle.debug) { log(s); } } function log() { if (window.console && window.console.log) { window.console.log("[cycle] " + Array.prototype.join.call(arguments, " ")); } } $.fn.cycle = function (options, arg2) { var o = { s: this.selector, c: this.context }; if (this.length === 0 && options != "stop") { if (!$.isReady && o.s) { log("DOM not ready, queuing slideshow"); $(function () { $(o.s, o.c).cycle(options, arg2); }); return this; } log("terminating; zero elements found by selector" + ($.isReady ? "" : " (DOM not ready)")); return this; } return this.each(function () { var opts = handleArguments(this, options, arg2); if (opts === false) { return; } opts.updateActivePagerLink = opts.updateActivePagerLink || $.fn.cycle.updateActivePagerLink; if (this.cycleTimeout) { clearTimeout(this.cycleTimeout); } this.cycleTimeout = this.cyclePause = 0; var $cont = $(this); var $slides = opts.slideExpr ? $(opts.slideExpr, this) : $cont.children(); var els = $slides.get(); if (els.length < 2) { log("terminating; too few slides: " + els.length); return; } var opts2 = buildOptions($cont, $slides, els, opts, o); if (opts2 === false) { return; } var startTime = opts2.continuous ? 10 : getTimeout(els[opts2.currSlide], els[opts2.nextSlide], opts2, !opts2.backwards); if (startTime) { startTime += (opts2.delay || 0); if (startTime < 10) { startTime = 10; } debug("first timeout: " + startTime); this.cycleTimeout = setTimeout(function () { go(els, opts2, 0, !opts.backwards); }, startTime); } }); }; function handleArguments(cont, options, arg2) { if (cont.cycleStop == undefined) { cont.cycleStop = 0; } if (options === undefined || options === null) { options = {}; } if (options.constructor == String) { switch (options) { case "destroy": case "stop": var opts = $(cont).data("cycle.opts"); if (!opts) { return false; } cont.cycleStop++; if (cont.cycleTimeout) { clearTimeout(cont.cycleTimeout); } cont.cycleTimeout = 0; $(cont).removeData("cycle.opts"); if (options == "destroy") { destroy(opts); } return false; case "toggle": cont.cyclePause = (cont.cyclePause === 1) ? 0 : 1; checkInstantResume(cont.cyclePause, arg2, cont); return false; case "pause": cont.cyclePause = 1; return false; case "resume": cont.cyclePause = 0; checkInstantResume(false, arg2, cont); return false; case "prev": case "next": var opts = $(cont).data("cycle.opts"); if (!opts) { log('options not found, "prev/next" ignored'); return false; } $.fn.cycle[options](opts); return false; default: options = { fx: options }; } return options; } else { if (options.constructor == Number) { var num = options; options = $(cont).data("cycle.opts"); if (!options) { log("options not found, can not advance slide"); return false; } if (num < 0 || num >= options.elements.length) { log("invalid slide index: " + num); return false; } options.nextSlide = num; if (cont.cycleTimeout) { clearTimeout(cont.cycleTimeout); cont.cycleTimeout = 0; } if (typeof arg2 == "string") { options.oneTimeFx = arg2; } go(options.elements, options, 1, num >= options.currSlide); return false; } } return options; function checkInstantResume(isPaused, arg2, cont) { if (!isPaused && arg2 === true) { var options = $(cont).data("cycle.opts"); if (!options) { log("options not found, can not resume"); return false; } if (cont.cycleTimeout) { clearTimeout(cont.cycleTimeout); cont.cycleTimeout = 0; } go(options.elements, options, 1, !options.backwards); } } } function removeFilter(el, opts) { if (!$.support.opacity && opts.cleartype && el.style.filter) { try { el.style.removeAttribute("filter"); } catch (smother) { } } } function destroy(opts) { if (opts.next) { $(opts.next).unbind(opts.prevNextEvent); } if (opts.prev) { $(opts.prev).unbind(opts.prevNextEvent); } if (opts.pager || opts.pagerAnchorBuilder) { $.each(opts.pagerAnchors || [], function () { this.unbind().remove(); }); } opts.pagerAnchors = null; if (opts.destroy) { opts.destroy(opts); } } function buildOptions($cont, $slides, els, options, o) { var opts = $.extend({}, $.fn.cycle.defaults, options || {}, $.metadata ? $cont.metadata() : $.meta ? $cont.data() : {}); if (opts.autostop) { opts.countdown = opts.autostopCount || els.length; } var cont = $cont[0]; $cont.data("cycle.opts", opts); opts.$cont = $cont; opts.stopCount = cont.cycleStop; opts.elements = els; opts.before = opts.before ? [opts.before] : []; opts.after = opts.after ? [opts.after] : []; opts.after.unshift(function () { opts.busy = 0; }); if (!$.support.opacity && opts.cleartype) { opts.after.push(function () { removeFilter(this, opts); }); } if (opts.continuous) { opts.after.push(function () { go(els, opts, 0, !opts.backwards); }); } saveOriginalOpts(opts); if (!$.support.opacity && opts.cleartype && !opts.cleartypeNoBg) { clearTypeFix($slides); } if ($cont.css("position") == "static") { $cont.css("position", "relative"); } if (opts.width) { $cont.width(opts.width); } if (opts.height && opts.height != "auto") { $cont.height(opts.height); } if (opts.startingSlide) { opts.startingSlide = parseInt(opts.startingSlide); } else { if (opts.backwards) { opts.startingSlide = els.length - 1; } } if (opts.random) { opts.randomMap = []; for (var i = 0; i < els.length; i++) { opts.randomMap.push(i); } opts.randomMap.sort(function (a, b) { return Math.random() - 0.5; }); opts.randomIndex = 1; opts.startingSlide = opts.randomMap[1]; } else { if (opts.startingSlide >= els.length) { opts.startingSlide = 0; } } opts.currSlide = opts.startingSlide || 0; var first = opts.startingSlide; $slides.css({ position: "absolute", top: 0, left: 0 }).hide().each(function (i) { var z; if (opts.backwards) { z = first ? i <= first ? els.length + (i - first) : first - i : els.length - i; } else { z = first ? i >= first ? els.length - (i - first) : first - i : els.length - i; } $(this).css("z-index", z); }); $(els[first]).css("opacity", 1).show(); removeFilter(els[first], opts); if (opts.fit && opts.width) { $slides.width(opts.width); } if (opts.fit && opts.height && opts.height != "auto") { $slides.height(opts.height); } var reshape = opts.containerResize && !$cont.innerHeight(); if (reshape) { var maxw = 0, maxh = 0; for (var j = 0; j < els.length; j++) { var $e = $(els[j]), e = $e[0], w = $e.outerWidth(), h = $e.outerHeight(); if (!w) { w = e.offsetWidth || e.width || $e.attr("width"); } if (!h) { h = e.offsetHeight || e.height || $e.attr("height"); } maxw = w > maxw ? w : maxw; maxh = h > maxh ? h : maxh; } if (maxw > 0 && maxh > 0) { $cont.css({ width: maxw + "px", height: maxh + "px" }); } } if (opts.pause) { $cont.hover(function () { this.cyclePause++; }, function () { this.cyclePause--; }); } if (supportMultiTransitions(opts) === false) { return false; } var requeue = false; options.requeueAttempts = options.requeueAttempts || 0; $slides.each(function () { var $el = $(this); this.cycleH = (opts.fit && opts.height) ? opts.height : ($el.height() || this.offsetHeight || this.height || $el.attr("height") || 0); this.cycleW = (opts.fit && opts.width) ? opts.width : ($el.width() || this.offsetWidth || this.width || $el.attr("width") || 0); if ($el.is("img")) { var loadingIE = ($.browser.msie && this.cycleW == 28 && this.cycleH == 30 && !this.complete); var loadingFF = ($.browser.mozilla && this.cycleW == 34 && this.cycleH == 19 && !this.complete); var loadingOp = ($.browser.opera && ((this.cycleW == 42 && this.cycleH == 19) || (this.cycleW == 37 && this.cycleH == 17)) && !this.complete); var loadingOther = (this.cycleH == 0 && this.cycleW == 0 && !this.complete); if (loadingIE || loadingFF || loadingOp || loadingOther) { if (o.s && opts.requeueOnImageNotLoaded && ++options.requeueAttempts < 100) { log(options.requeueAttempts, " - img slide not loaded, requeuing slideshow: ", this.src, this.cycleW, this.cycleH); setTimeout(function () { $(o.s, o.c).cycle(options); }, opts.requeueTimeout); requeue = true; return false; } else { log("could not determine size of image: " + this.src, this.cycleW, this.cycleH); } } } return true; }); if (requeue) { return false; } opts.cssBefore = opts.cssBefore || {}; opts.animIn = opts.animIn || {}; opts.animOut = opts.animOut || {}; $slides.not(":eq(" + first + ")").css(opts.cssBefore); if (opts.cssFirst) { $($slides[first]).css(opts.cssFirst); } if (opts.timeout) { opts.timeout = parseInt(opts.timeout); if (opts.speed.constructor == String) { opts.speed = $.fx.speeds[opts.speed] || parseInt(opts.speed); } if (!opts.sync) { opts.speed = opts.speed / 2; } var buffer = opts.fx == "shuffle" ? 500 : 250; while ((opts.timeout - opts.speed) < buffer) { opts.timeout += opts.speed; } } if (opts.easing) { opts.easeIn = opts.easeOut = opts.easing; } if (!opts.speedIn) { opts.speedIn = opts.speed; } if (!opts.speedOut) { opts.speedOut = opts.speed; } opts.slideCount = els.length; opts.currSlide = opts.lastSlide = first; if (opts.random) { if (++opts.randomIndex == els.length) { opts.randomIndex = 0; } opts.nextSlide = opts.randomMap[opts.randomIndex]; } else { if (opts.backwards) { opts.nextSlide = opts.startingSlide == 0 ? (els.length - 1) : opts.startingSlide - 1; } else { opts.nextSlide = opts.startingSlide >= (els.length - 1) ? 0 : opts.startingSlide + 1; } } if (!opts.multiFx) { var init = $.fn.cycle.transitions[opts.fx]; if ($.isFunction(init)) { init($cont, $slides, opts); } else { if (opts.fx != "custom" && !opts.multiFx) { log("unknown transition: " + opts.fx, "; slideshow terminating"); return false; } } } var e0 = $slides[first]; if (opts.before.length) { opts.before[0].apply(e0, [e0, e0, opts, true]); } if (opts.after.length > 1) { opts.after[1].apply(e0, [e0, e0, opts, true]); } if (opts.next) { $(opts.next).bind(opts.prevNextEvent, function () { return advance(opts, 1); }); } if (opts.prev) { $(opts.prev).bind(opts.prevNextEvent, function () { return advance(opts, 0); }); } if (opts.pager || opts.pagerAnchorBuilder) { buildPager(els, opts); } exposeAddSlide(opts, els); return opts; } function saveOriginalOpts(opts) { opts.original = { before: [], after: [] }; opts.original.cssBefore = $.extend({}, opts.cssBefore); opts.original.cssAfter = $.extend({}, opts.cssAfter); opts.original.animIn = $.extend({}, opts.animIn); opts.original.animOut = $.extend({}, opts.animOut); $.each(opts.before, function () { opts.original.before.push(this); }); $.each(opts.after, function () { opts.original.after.push(this); }); } function supportMultiTransitions(opts) { var i, tx, txs = $.fn.cycle.transitions; if (opts.fx.indexOf(",") > 0) { opts.multiFx = true; opts.fxs = opts.fx.replace(/\s*/g, "").split(","); for (i = 0; i < opts.fxs.length; i++) { var fx = opts.fxs[i]; tx = txs[fx]; if (!tx || !txs.hasOwnProperty(fx) || !$.isFunction(tx)) { log("discarding unknown transition: ", fx); opts.fxs.splice(i, 1); i--; } } if (!opts.fxs.length) { log("No valid transitions named; slideshow terminating."); return false; } } else { if (opts.fx == "all") { opts.multiFx = true; opts.fxs = []; for (p in txs) { tx = txs[p]; if (txs.hasOwnProperty(p) && $.isFunction(tx)) { opts.fxs.push(p); } } } } if (opts.multiFx && opts.randomizeEffects) { var r1 = Math.floor(Math.random() * 20) + 30; for (i = 0; i < r1; i++) { var r2 = Math.floor(Math.random() * opts.fxs.length); opts.fxs.push(opts.fxs.splice(r2, 1)[0]); } debug("randomized fx sequence: ", opts.fxs); } return true; } function exposeAddSlide(opts, els) { opts.addSlide = function (newSlide, prepend) { var $s = $(newSlide), s = $s[0]; if (!opts.autostopCount) { opts.countdown++; } els[prepend ? "unshift" : "push"](s); if (opts.els) { opts.els[prepend ? "unshift" : "push"](s); } opts.slideCount = els.length; $s.css("position", "absolute"); $s[prepend ? "prependTo" : "appendTo"](opts.$cont); if (prepend) { opts.currSlide++; opts.nextSlide++; } if (!$.support.opacity && opts.cleartype && !opts.cleartypeNoBg) { clearTypeFix($s); } if (opts.fit && opts.width) { $s.width(opts.width); } if (opts.fit && opts.height && opts.height != "auto") { $s.height(opts.height); } s.cycleH = (opts.fit && opts.height) ? opts.height : $s.height(); s.cycleW = (opts.fit && opts.width) ? opts.width : $s.width(); $s.css(opts.cssBefore); if (opts.pager || opts.pagerAnchorBuilder) { $.fn.cycle.createPagerAnchor(els.length - 1, s, $(opts.pager), els, opts); } if ($.isFunction(opts.onAddSlide)) { opts.onAddSlide($s); } else { $s.hide(); } }; } $.fn.cycle.resetState = function (opts, fx) { fx = fx || opts.fx; opts.before = []; opts.after = []; opts.cssBefore = $.extend({}, opts.original.cssBefore); opts.cssAfter = $.extend({}, opts.original.cssAfter); opts.animIn = $.extend({}, opts.original.animIn); opts.animOut = $.extend({}, opts.original.animOut); opts.fxFn = null; $.each(opts.original.before, function () { opts.before.push(this); }); $.each(opts.original.after, function () { opts.after.push(this); }); var init = $.fn.cycle.transitions[fx]; if ($.isFunction(init)) { init(opts.$cont, $(opts.elements), opts); } }; function go(els, opts, manual, fwd) { if (manual && opts.busy && opts.manualTrump) { debug("manualTrump in go(), stopping active transition"); $(els).stop(true, true); opts.busy = false; } if (opts.busy) { debug("transition active, ignoring new tx request"); return; } var p = opts.$cont[0], curr = els[opts.currSlide], next = els[opts.nextSlide]; if (p.cycleStop != opts.stopCount || p.cycleTimeout === 0 && !manual) { return; } if (!manual && !p.cyclePause && !opts.bounce && ((opts.autostop && (--opts.countdown <= 0)) || (opts.nowrap && !opts.random && opts.nextSlide < opts.currSlide))) { if (opts.end) { opts.end(opts); } return; } var changed = false; if ((manual || !p.cyclePause) && (opts.nextSlide != opts.currSlide)) { changed = true; var fx = opts.fx; curr.cycleH = curr.cycleH || $(curr).height(); curr.cycleW = curr.cycleW || $(curr).width(); next.cycleH = next.cycleH || $(next).height(); next.cycleW = next.cycleW || $(next).width(); if (opts.multiFx) { if (opts.lastFx == undefined || ++opts.lastFx >= opts.fxs.length) { opts.lastFx = 0; } fx = opts.fxs[opts.lastFx]; opts.currFx = fx; } if (opts.oneTimeFx) { fx = opts.oneTimeFx; opts.oneTimeFx = null; } $.fn.cycle.resetState(opts, fx); if (opts.before.length) { $.each(opts.before, function (i, o) { if (p.cycleStop != opts.stopCount) { return; } o.apply(next, [curr, next, opts, fwd]); }); } var after = function () { $.each(opts.after, function (i, o) { if (p.cycleStop != opts.stopCount) { return; } o.apply(next, [curr, next, opts, fwd]); }); }; debug("tx firing; currSlide: " + opts.currSlide + "; nextSlide: " + opts.nextSlide); opts.busy = 1; if (opts.fxFn) { opts.fxFn(curr, next, opts, after, fwd, manual && opts.fastOnEvent); } else { if ($.isFunction($.fn.cycle[opts.fx])) { $.fn.cycle[opts.fx](curr, next, opts, after, fwd, manual && opts.fastOnEvent); } else { $.fn.cycle.custom(curr, next, opts, after, fwd, manual && opts.fastOnEvent); } } } if (changed || opts.nextSlide == opts.currSlide) { opts.lastSlide = opts.currSlide; if (opts.random) { opts.currSlide = opts.nextSlide; if (++opts.randomIndex == els.length) { opts.randomIndex = 0; } opts.nextSlide = opts.randomMap[opts.randomIndex]; if (opts.nextSlide == opts.currSlide) { opts.nextSlide = (opts.currSlide == opts.slideCount - 1) ? 0 : opts.currSlide + 1; } } else { if (opts.backwards) { var roll = (opts.nextSlide - 1) < 0; if (roll && opts.bounce) { opts.backwards = !opts.backwards; opts.nextSlide = 1; opts.currSlide = 0; } else { opts.nextSlide = roll ? (els.length - 1) : opts.nextSlide - 1; opts.currSlide = roll ? 0 : opts.nextSlide + 1; } } else { var roll = (opts.nextSlide + 1) == els.length; if (roll && opts.bounce) { opts.backwards = !opts.backwards; opts.nextSlide = els.length - 2; opts.currSlide = els.length - 1; } else { opts.nextSlide = roll ? 0 : opts.nextSlide + 1; opts.currSlide = roll ? els.length - 1 : opts.nextSlide - 1; } } } } if (changed && opts.pager) { opts.updateActivePagerLink(opts.pager, opts.currSlide, opts.activePagerClass); } var ms = 0; if (opts.timeout && !opts.continuous) { ms = getTimeout(els[opts.currSlide], els[opts.nextSlide], opts, fwd); } else { if (opts.continuous && p.cyclePause) { ms = 10; } } if (ms > 0) { p.cycleTimeout = setTimeout(function () { go(els, opts, 0, !opts.backwards); }, ms); } } $.fn.cycle.updateActivePagerLink = function (pager, currSlide, clsName) { $(pager).each(function () { $(this).children().removeClass(clsName).eq(currSlide).addClass(clsName); }); }; function getTimeout(curr, next, opts, fwd) { if (opts.timeoutFn) { var t = opts.timeoutFn.call(curr, curr, next, opts, fwd); while ((t - opts.speed) < 250) { t += opts.speed; } debug("calculated timeout: " + t + "; speed: " + opts.speed); if (t !== false) { return t; } } return opts.timeout; } $.fn.cycle.next = function (opts) { advance(opts, 1); }; $.fn.cycle.prev = function (opts) { advance(opts, 0); }; function advance(opts, moveForward) { var val = moveForward ? 1 : -1; var els = opts.elements; var p = opts.$cont[0], timeout = p.cycleTimeout; if (timeout) { clearTimeout(timeout); p.cycleTimeout = 0; } if (opts.random && val < 0) { opts.randomIndex--; if (--opts.randomIndex == -2) { opts.randomIndex = els.length - 2; } else { if (opts.randomIndex == -1) { opts.randomIndex = els.length - 1; } } opts.nextSlide = opts.randomMap[opts.randomIndex]; } else { if (opts.random) { opts.nextSlide = opts.randomMap[opts.randomIndex]; } else { opts.nextSlide = opts.currSlide + val; if (opts.nextSlide < 0) { if (opts.nowrap) { return false; } opts.nextSlide = els.length - 1; } else { if (opts.nextSlide >= els.length) { if (opts.nowrap) { return false; } opts.nextSlide = 0; } } } } var cb = opts.onPrevNextEvent || opts.prevNextClick; if ($.isFunction(cb)) { cb(val > 0, opts.nextSlide, els[opts.nextSlide]); } go(els, opts, 1, moveForward); return false; } function buildPager(els, opts) { var $p = $(opts.pager); $.each(els, function (i, o) { $.fn.cycle.createPagerAnchor(i, o, $p, els, opts); }); opts.updateActivePagerLink(opts.pager, opts.startingSlide, opts.activePagerClass); } $.fn.cycle.createPagerAnchor = function (i, el, $p, els, opts) { var a; if ($.isFunction(opts.pagerAnchorBuilder)) { a = opts.pagerAnchorBuilder(i, el); debug("pagerAnchorBuilder(" + i + ", el) returned: " + a); } else { a = '<a href="#">' + (i + 1) + "</a>"; } if (!a) { return; } var $a = $(a); if ($a.parents("body").length === 0) { var arr = []; if ($p.length > 1) { $p.each(function () { var $clone = $a.clone(true); $(this).append($clone); arr.push($clone[0]); }); $a = $(arr); } else { $a.appendTo($p); } } opts.pagerAnchors = opts.pagerAnchors || []; opts.pagerAnchors.push($a); $a.bind(opts.pagerEvent, function (e) { e.preventDefault(); opts.nextSlide = i; var p = opts.$cont[0], timeout = p.cycleTimeout; if (timeout) { clearTimeout(timeout); p.cycleTimeout = 0; } var cb = opts.onPagerEvent || opts.pagerClick; if ($.isFunction(cb)) { cb(opts.nextSlide, els[opts.nextSlide]); } go(els, opts, 1, opts.currSlide < i); }); if (!/^click/.test(opts.pagerEvent) && !opts.allowPagerClickBubble) { $a.bind("click.cycle", function () { return false; }); } if (opts.pauseOnPagerHover) { $a.hover(function () { opts.$cont[0].cyclePause++; }, function () { opts.$cont[0].cyclePause--; }); } }; $.fn.cycle.hopsFromLast = function (opts, fwd) { var hops, l = opts.lastSlide, c = opts.currSlide; if (fwd) { hops = c > l ? c - l : opts.slideCount - l; } else { hops = c < l ? l - c : l + opts.slideCount - c; } return hops; }; function clearTypeFix($slides) { debug("applying clearType background-color hack"); function hex(s) { s = parseInt(s).toString(16); return s.length < 2 ? "0" + s : s; } function getBg(e) { for (; e && e.nodeName.toLowerCase() != "html"; e = e.parentNode) { var v = $.css(e, "background-color"); if (v.indexOf("rgb") >= 0) { var rgb = v.match(/\d+/g); return "#" + hex(rgb[0]) + hex(rgb[1]) + hex(rgb[2]); } if (v && v != "transparent") { return v; } } return "#ffffff"; } $slides.each(function () { $(this).css("background-color", getBg(this)); }); } $.fn.cycle.commonReset = function (curr, next, opts, w, h, rev) { $(opts.elements).not(curr).hide(); opts.cssBefore.opacity = 1; opts.cssBefore.display = "block"; if (opts.slideResize && w !== false && next.cycleW > 0) { opts.cssBefore.width = next.cycleW; } if (opts.slideResize && h !== false && next.cycleH > 0) { opts.cssBefore.height = next.cycleH; } opts.cssAfter = opts.cssAfter || {}; opts.cssAfter.display = "none"; $(curr).css("zIndex", opts.slideCount + (rev === true ? 1 : 0)); $(next).css("zIndex", opts.slideCount + (rev === true ? 0 : 1)); }; $.fn.cycle.custom = function (curr, next, opts, cb, fwd, speedOverride) { var $l = $(curr), $n = $(next); var speedIn = opts.speedIn, speedOut = opts.speedOut, easeIn = opts.easeIn, easeOut = opts.easeOut; $n.css(opts.cssBefore); if (speedOverride) { if (typeof speedOverride == "number") { speedIn = speedOut = speedOverride; } else { speedIn = speedOut = 1; } easeIn = easeOut = null; } var fn = function () { $n.animate(opts.animIn, speedIn, easeIn, cb); }; $l.animate(opts.animOut, speedOut, easeOut, function () { if (opts.cssAfter) { $l.css(opts.cssAfter); } if (!opts.sync) { fn(); } }); if (opts.sync) { fn(); } }; $.fn.cycle.transitions = { fade: function ($cont, $slides, opts) { $slides.not(":eq(" + opts.currSlide + ")").css("opacity", 0); opts.before.push(function (curr, next, opts) { $.fn.cycle.commonReset(curr, next, opts); opts.cssBefore.opacity = 0; }); opts.animIn = { opacity: 1 }; opts.animOut = { opacity: 0 }; opts.cssBefore = { top: 0, left: 0 }; } }; $.fn.cycle.ver = function () { return ver; }; $.fn.cycle.defaults = { fx: "fade", timeout: 4000, timeoutFn: null, continuous: 0, speed: 1000, speedIn: null, speedOut: null, next: null, prev: null, onPrevNextEvent: null, prevNextEvent: "click.cycle", pager: null, onPagerEvent: null, pagerEvent: "click.cycle", allowPagerClickBubble: false, pagerAnchorBuilder: null, before: null, after: null, end: null, easing: null, easeIn: null, easeOut: null, shuffle: null, animIn: null, animOut: null, cssBefore: null, cssAfter: null, fxFn: null, height: "auto", startingSlide: 0, sync: 1, random: 0, fit: 0, containerResize: 1, slideResize: 1, pause: 0, pauseOnPagerHover: 0, autostop: 0, autostopCount: 0, delay: 0, slideExpr: null, cleartype: !$.support.opacity, cleartypeNoBg: false, nowrap: 0, fastOnEvent: 0, randomizeEffects: 1, rev: 0, manualTrump: true, requeueOnImageNotLoaded: true, requeueTimeout: 250, activePagerClass: "activeSlide", updateActivePagerLink: null, backwards: false }; })(jQuery);
/*
 * jQuery Cycle Plugin Transition Definitions
 * This script is a plugin for the jQuery Cycle Plugin
 * Examples and documentation at: http://malsup.com/jquery/cycle/
 * Copyright (c) 2007-2010 M. Alsup
 * Version:	 2.73
 * Dual licensed under the MIT and GPL licenses:
 * http://www.opensource.org/licenses/mit-license.php
 * http://www.gnu.org/licenses/gpl.html
 */
(function ($) { $.fn.cycle.transitions.none = function ($cont, $slides, opts) { opts.fxFn = function (curr, next, opts, after) { $(next).show(); $(curr).hide(); after(); }; }; $.fn.cycle.transitions.fadeout = function ($cont, $slides, opts) { $slides.not(":eq(" + opts.currSlide + ")").css({ display: "block", opacity: 1 }); opts.before.push(function (curr, next, opts, w, h, rev) { $(curr).css("zIndex", opts.slideCount + (!rev === true ? 1 : 0)); $(next).css("zIndex", opts.slideCount + (!rev === true ? 0 : 1)); }); opts.animIn = { opacity: 1 }; opts.animOut = { opacity: 0 }; opts.cssBefore = { opacity: 1, display: "block" }; opts.cssAfter = { zIndex: 0 }; }; $.fn.cycle.transitions.scrollUp = function ($cont, $slides, opts) { $cont.css("overflow", "hidden"); opts.before.push($.fn.cycle.commonReset); var h = $cont.height(); opts.cssBefore = { top: h, left: 0 }; opts.cssFirst = { top: 0 }; opts.animIn = { top: 0 }; opts.animOut = { top: -h }; }; $.fn.cycle.transitions.scrollDown = function ($cont, $slides, opts) { $cont.css("overflow", "hidden"); opts.before.push($.fn.cycle.commonReset); var h = $cont.height(); opts.cssFirst = { top: 0 }; opts.cssBefore = { top: -h, left: 0 }; opts.animIn = { top: 0 }; opts.animOut = { top: h }; }; $.fn.cycle.transitions.scrollLeft = function ($cont, $slides, opts) { $cont.css("overflow", "hidden"); opts.before.push($.fn.cycle.commonReset); var w = $cont.width(); opts.cssFirst = { left: 0 }; opts.cssBefore = { left: w, top: 0 }; opts.animIn = { left: 0 }; opts.animOut = { left: 0 - w }; }; $.fn.cycle.transitions.scrollRight = function ($cont, $slides, opts) { $cont.css("overflow", "hidden"); opts.before.push($.fn.cycle.commonReset); var w = $cont.width(); opts.cssFirst = { left: 0 }; opts.cssBefore = { left: -w, top: 0 }; opts.animIn = { left: 0 }; opts.animOut = { left: w }; }; $.fn.cycle.transitions.scrollHorz = function ($cont, $slides, opts) { $cont.css("overflow", "hidden").width(); opts.before.push(function (curr, next, opts, fwd) { if (opts.rev) { fwd = !fwd; } $.fn.cycle.commonReset(curr, next, opts); opts.cssBefore.left = fwd ? (next.cycleW - 1) : (1 - next.cycleW); opts.animOut.left = fwd ? -curr.cycleW : curr.cycleW; }); opts.cssFirst = { left: 0 }; opts.cssBefore = { top: 0 }; opts.animIn = { left: 0 }; opts.animOut = { top: 0 }; }; $.fn.cycle.transitions.scrollVert = function ($cont, $slides, opts) { $cont.css("overflow", "hidden"); opts.before.push(function (curr, next, opts, fwd) { if (opts.rev) { fwd = !fwd; } $.fn.cycle.commonReset(curr, next, opts); opts.cssBefore.top = fwd ? (1 - next.cycleH) : (next.cycleH - 1); opts.animOut.top = fwd ? curr.cycleH : -curr.cycleH; }); opts.cssFirst = { top: 0 }; opts.cssBefore = { left: 0 }; opts.animIn = { top: 0 }; opts.animOut = { left: 0 }; }; $.fn.cycle.transitions.slideX = function ($cont, $slides, opts) { opts.before.push(function (curr, next, opts) { $(opts.elements).not(curr).hide(); $.fn.cycle.commonReset(curr, next, opts, false, true); opts.animIn.width = next.cycleW; }); opts.cssBefore = { left: 0, top: 0, width: 0 }; opts.animIn = { width: "show" }; opts.animOut = { width: 0 }; }; $.fn.cycle.transitions.slideY = function ($cont, $slides, opts) { opts.before.push(function (curr, next, opts) { $(opts.elements).not(curr).hide(); $.fn.cycle.commonReset(curr, next, opts, true, false); opts.animIn.height = next.cycleH; }); opts.cssBefore = { left: 0, top: 0, height: 0 }; opts.animIn = { height: "show" }; opts.animOut = { height: 0 }; }; $.fn.cycle.transitions.shuffle = function ($cont, $slides, opts) { var i, w = $cont.css("overflow", "visible").width(); $slides.css({ left: 0, top: 0 }); opts.before.push(function (curr, next, opts) { $.fn.cycle.commonReset(curr, next, opts, true, true, true); }); if (!opts.speedAdjusted) { opts.speed = opts.speed / 2; opts.speedAdjusted = true; } opts.random = 0; opts.shuffle = opts.shuffle || { left: -w, top: 15 }; opts.els = []; for (i = 0; i < $slides.length; i++) { opts.els.push($slides[i]); } for (i = 0; i < opts.currSlide; i++) { opts.els.push(opts.els.shift()); } opts.fxFn = function (curr, next, opts, cb, fwd) { if (opts.rev) { fwd = !fwd; } var $el = fwd ? $(curr) : $(next); $(next).css(opts.cssBefore); var count = opts.slideCount; $el.animate(opts.shuffle, opts.speedIn, opts.easeIn, function () { var hops = $.fn.cycle.hopsFromLast(opts, fwd); for (var k = 0; k < hops; k++) { fwd ? opts.els.push(opts.els.shift()) : opts.els.unshift(opts.els.pop()); } if (fwd) { for (var i = 0, len = opts.els.length; i < len; i++) { $(opts.els[i]).css("z-index", len - i + count); } } else { var z = $(curr).css("z-index"); $el.css("z-index", parseInt(z) + 1 + count); } $el.animate({ left: 0, top: 0 }, opts.speedOut, opts.easeOut, function () { $(fwd ? this : curr).hide(); if (cb) { cb(); } }); }); }; opts.cssBefore = { display: "block", opacity: 1, top: 0, left: 0 }; }; $.fn.cycle.transitions.turnUp = function ($cont, $slides, opts) { opts.before.push(function (curr, next, opts) { $.fn.cycle.commonReset(curr, next, opts, true, false); opts.cssBefore.top = next.cycleH; opts.animIn.height = next.cycleH; opts.animOut.width = next.cycleW; }); opts.cssFirst = { top: 0 }; opts.cssBefore = { left: 0, height: 0 }; opts.animIn = { top: 0 }; opts.animOut = { height: 0 }; }; $.fn.cycle.transitions.turnDown = function ($cont, $slides, opts) { opts.before.push(function (curr, next, opts) { $.fn.cycle.commonReset(curr, next, opts, true, false); opts.animIn.height = next.cycleH; opts.animOut.top = curr.cycleH; }); opts.cssFirst = { top: 0 }; opts.cssBefore = { left: 0, top: 0, height: 0 }; opts.animOut = { height: 0 }; }; $.fn.cycle.transitions.turnLeft = function ($cont, $slides, opts) { opts.before.push(function (curr, next, opts) { $.fn.cycle.commonReset(curr, next, opts, false, true); opts.cssBefore.left = next.cycleW; opts.animIn.width = next.cycleW; }); opts.cssBefore = { top: 0, width: 0 }; opts.animIn = { left: 0 }; opts.animOut = { width: 0 }; }; $.fn.cycle.transitions.turnRight = function ($cont, $slides, opts) { opts.before.push(function (curr, next, opts) { $.fn.cycle.commonReset(curr, next, opts, false, true); opts.animIn.width = next.cycleW; opts.animOut.left = curr.cycleW; }); opts.cssBefore = { top: 0, left: 0, width: 0 }; opts.animIn = { left: 0 }; opts.animOut = { width: 0 }; }; $.fn.cycle.transitions.zoom = function ($cont, $slides, opts) { opts.before.push(function (curr, next, opts) { $.fn.cycle.commonReset(curr, next, opts, false, false, true); opts.cssBefore.top = next.cycleH / 2; opts.cssBefore.left = next.cycleW / 2; opts.animIn = { top: 0, left: 0, width: next.cycleW, height: next.cycleH }; opts.animOut = { width: 0, height: 0, top: curr.cycleH / 2, left: curr.cycleW / 2 }; }); opts.cssFirst = { top: 0, left: 0 }; opts.cssBefore = { width: 0, height: 0 }; }; $.fn.cycle.transitions.fadeZoom = function ($cont, $slides, opts) { opts.before.push(function (curr, next, opts) { $.fn.cycle.commonReset(curr, next, opts, false, false); opts.cssBefore.left = next.cycleW / 2; opts.cssBefore.top = next.cycleH / 2; opts.animIn = { top: 0, left: 0, width: next.cycleW, height: next.cycleH }; }); opts.cssBefore = { width: 0, height: 0 }; opts.animOut = { opacity: 0 }; }; $.fn.cycle.transitions.blindX = function ($cont, $slides, opts) { var w = $cont.css("overflow", "hidden").width(); opts.before.push(function (curr, next, opts) { $.fn.cycle.commonReset(curr, next, opts); opts.animIn.width = next.cycleW; opts.animOut.left = curr.cycleW; }); opts.cssBefore = { left: w, top: 0 }; opts.animIn = { left: 0 }; opts.animOut = { left: w }; }; $.fn.cycle.transitions.blindY = function ($cont, $slides, opts) { var h = $cont.css("overflow", "hidden").height(); opts.before.push(function (curr, next, opts) { $.fn.cycle.commonReset(curr, next, opts); opts.animIn.height = next.cycleH; opts.animOut.top = curr.cycleH; }); opts.cssBefore = { top: h, left: 0 }; opts.animIn = { top: 0 }; opts.animOut = { top: h }; }; $.fn.cycle.transitions.blindZ = function ($cont, $slides, opts) { var h = $cont.css("overflow", "hidden").height(); var w = $cont.width(); opts.before.push(function (curr, next, opts) { $.fn.cycle.commonReset(curr, next, opts); opts.animIn.height = next.cycleH; opts.animOut.top = curr.cycleH; }); opts.cssBefore = { top: h, left: w }; opts.animIn = { top: 0, left: 0 }; opts.animOut = { top: h, left: w }; }; $.fn.cycle.transitions.growX = function ($cont, $slides, opts) { opts.before.push(function (curr, next, opts) { $.fn.cycle.commonReset(curr, next, opts, false, true); opts.cssBefore.left = this.cycleW / 2; opts.animIn = { left: 0, width: this.cycleW }; opts.animOut = { left: 0 }; }); opts.cssBefore = { width: 0, top: 0 }; }; $.fn.cycle.transitions.growY = function ($cont, $slides, opts) { opts.before.push(function (curr, next, opts) { $.fn.cycle.commonReset(curr, next, opts, true, false); opts.cssBefore.top = this.cycleH / 2; opts.animIn = { top: 0, height: this.cycleH }; opts.animOut = { top: 0 }; }); opts.cssBefore = { height: 0, left: 0 }; }; $.fn.cycle.transitions.curtainX = function ($cont, $slides, opts) { opts.before.push(function (curr, next, opts) { $.fn.cycle.commonReset(curr, next, opts, false, true, true); opts.cssBefore.left = next.cycleW / 2; opts.animIn = { left: 0, width: this.cycleW }; opts.animOut = { left: curr.cycleW / 2, width: 0 }; }); opts.cssBefore = { top: 0, width: 0 }; }; $.fn.cycle.transitions.curtainY = function ($cont, $slides, opts) { opts.before.push(function (curr, next, opts) { $.fn.cycle.commonReset(curr, next, opts, true, false, true); opts.cssBefore.top = next.cycleH / 2; opts.animIn = { top: 0, height: next.cycleH }; opts.animOut = { top: curr.cycleH / 2, height: 0 }; }); opts.cssBefore = { left: 0, height: 0 }; }; $.fn.cycle.transitions.cover = function ($cont, $slides, opts) { var d = opts.direction || "left"; var w = $cont.css("overflow", "hidden").width(); var h = $cont.height(); opts.before.push(function (curr, next, opts) { $.fn.cycle.commonReset(curr, next, opts); if (d == "right") { opts.cssBefore.left = -w; } else { if (d == "up") { opts.cssBefore.top = h; } else { if (d == "down") { opts.cssBefore.top = -h; } else { opts.cssBefore.left = w; } } } }); opts.animIn = { left: 0, top: 0 }; opts.animOut = { opacity: 1 }; opts.cssBefore = { top: 0, left: 0 }; }; $.fn.cycle.transitions.uncover = function ($cont, $slides, opts) { var d = opts.direction || "left"; var w = $cont.css("overflow", "hidden").width(); var h = $cont.height(); opts.before.push(function (curr, next, opts) { $.fn.cycle.commonReset(curr, next, opts, true, true, true); if (d == "right") { opts.animOut.left = w; } else { if (d == "up") { opts.animOut.top = -h; } else { if (d == "down") { opts.animOut.top = h; } else { opts.animOut.left = -w; } } } }); opts.animIn = { left: 0, top: 0 }; opts.animOut = { opacity: 1 }; opts.cssBefore = { top: 0, left: 0 }; }; $.fn.cycle.transitions.toss = function ($cont, $slides, opts) { var w = $cont.css("overflow", "visible").width(); var h = $cont.height(); opts.before.push(function (curr, next, opts) { $.fn.cycle.commonReset(curr, next, opts, true, true, true); if (!opts.animOut.left && !opts.animOut.top) { opts.animOut = { left: w * 2, top: -h / 2, opacity: 0 }; } else { opts.animOut.opacity = 0; } }); opts.cssBefore = { left: 0, top: 0 }; opts.animIn = { left: 0 }; }; $.fn.cycle.transitions.wipe = function ($cont, $slides, opts) { var w = $cont.css("overflow", "hidden").width(); var h = $cont.height(); opts.cssBefore = opts.cssBefore || {}; var clip; if (opts.clip) { if (/l2r/.test(opts.clip)) { clip = "rect(0px 0px " + h + "px 0px)"; } else { if (/r2l/.test(opts.clip)) { clip = "rect(0px " + w + "px " + h + "px " + w + "px)"; } else { if (/t2b/.test(opts.clip)) { clip = "rect(0px " + w + "px 0px 0px)"; } else { if (/b2t/.test(opts.clip)) { clip = "rect(" + h + "px " + w + "px " + h + "px 0px)"; } else { if (/zoom/.test(opts.clip)) { var top = parseInt(h / 2); var left = parseInt(w / 2); clip = "rect(" + top + "px " + left + "px " + top + "px " + left + "px)"; } } } } } } opts.cssBefore.clip = opts.cssBefore.clip || clip || "rect(0px 0px 0px 0px)"; var d = opts.cssBefore.clip.match(/(\d+)/g); var t = parseInt(d[0]), r = parseInt(d[1]), b = parseInt(d[2]), l = parseInt(d[3]); opts.before.push(function (curr, next, opts) { if (curr == next) { return; } var $curr = $(curr), $next = $(next); $.fn.cycle.commonReset(curr, next, opts, true, true, false); opts.cssAfter.display = "block"; var step = 1, count = parseInt((opts.speedIn / 13)) - 1; (function f() { var tt = t ? t - parseInt(step * (t / count)) : 0; var ll = l ? l - parseInt(step * (l / count)) : 0; var bb = b < h ? b + parseInt(step * ((h - b) / count || 1)) : h; var rr = r < w ? r + parseInt(step * ((w - r) / count || 1)) : w; $next.css({ clip: "rect(" + tt + "px " + rr + "px " + bb + "px " + ll + "px)" }); (step++ <= count) ? setTimeout(f, 13) : $curr.css("display", "none"); })(); }); opts.cssBefore = { display: "block", opacity: 1, top: 0, left: 0 }; opts.animIn = { left: 0 }; opts.animOut = { left: 0 }; }; })(jQuery);