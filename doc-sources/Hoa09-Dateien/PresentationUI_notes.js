/*globals $, setTimeout, i18nMinimize, i18nFullScreen*/
$(function () {
    'use strict';
// // the presentation page has the following panes
// - root pane; its direct children are the top and bottom panes
    var $rootPane = $('#pr_horizontal'),
// - top pane
// - bottom pane
        $bottomPane = $('#bottom'),
//      both the top and bottom pane have the width of the root pane. 
// - left pane
        $leftPane = $('#top .leftSide'),
// - right pane
        $rightPane = $('#top > .rightSide'),
//      The left and right panes are inside either the top or bottom panes as described below.
//      When both the left and the right panes are inside the top pane, they have to share the width. The video takes 37%, the slides 61%
//      
// the presentation page has the following components
// - video container
        $playerContainer = $('#playerContainer'),
// - slides container
        $slidesContainer = $('#slideContainer'),
// - download bar
        $downloadBar = $('div.download_presentation'),
// - view buttons bar
        $viewButtonsBar = $('div.presentation_type'),
// - flash player (only need to mind it for the chinese pages
        $ccVideoPlayer,
// - summary text is placed in the following container when in horizontal view
        $leftBottomTextCont = $('#top .leftSide #summary_height_fix_box .viewport .overview'),
// - summary text is placed in the following container when in vertical view
        $rightTopTextCont = $('#top > .rightSide .right_text_fix .viewport .overview'),
// - bio and conference description are placed here when in horizontal view
        $rightBottomTextCont = $('#top > .rightSide #text_height_fix_box .viewport .overview'),
// - author bio; comes as a sequence of two elements.
        $authorBio = $('#biolabel, #biotext'),
// - conference description
        $confDesc = $('#conference'),
// - presentation notes
        $presentationNotes = $('#presentationNotes'),
// - presentation summary
        $summary = $('#summary'),
// - vcr native top
        $vcrNativeTop = $('#vcrOptional1').clone(),
// - vcr native bottom
        $vcrNativeBottom = $('#vcrOptional2').clone(),


// the presentation page has the following modes
// - horizontal view (H)
//      in this view the player is in the left pane, the slides in the right pane. Both left and right panes are in the top pane
// - vertical view (V)
//      in this view both the player and the slides are in the left pane. The left pane is in the top pane, the right pane is in the bottom pane.
//      the H and V views are called non-full views
// - fullscreen horizontal view (FH)
// - fullscreen vertical view (FV)
// The default view is non-full horizontal
        inFullScreen = false,
        inHorizontal = true,
// - demo is a special view that is meant to display the video in whole width, even if the slides need to be hidden.
//      the purpose of this mode is to allow the user to follow what's happening on the presenter's screen. A demo is a video screencast.
        inDemo = false,
// - the transition from presentation to demo and from demo to presentations are done in an animated transition, configured here:
        transitionTime = 2000, // milliseconds, default is for InfoQ player
        animationInProgress = false, // don't allow switching between vertical, horizontal, full and non-full view during an animation
// when switching from the demo mode to presentation mode the size of the video and slides components needs to be known
        comeBackVideoHeight,
        comeBackVideoWidth,
        comeBackSlideHeight,
        comeBackSlideWidth,
// Transitions between views are possible by using the following buttons:
        $toHorizontalBtn = $('#horizontal'),
        $toVerticalBtn = $('#vertical'),
        $toAndFromFullBtn = $('#full'),
        $exitFullViewBar = $('#exitFullScreenMode'),
// The following transitions are possible:
// H > V
        switchToVertical = function () {
            $rootPane.removeClass('horizontal').addClass('vertical');
            inHorizontal = false;
            updateButtonStates();
            placeBannerSummaryBioAndConfDesc();
            adjustSlidesVideoAndBars();
        },
// V > H
        switchToHorizontal = function () {
            inHorizontal = true;
            $rootPane.removeClass('vertical').addClass('horizontal');
            updateButtonStates();
            placeBannerSummaryBioAndConfDesc();
            adjustSlidesVideoAndBars();
        },
// FH > FV
// FV > FH
// H > FH
// V > FH
        nonFullToFull = function () {
            inFullScreen = true;
            if (!inHorizontal) {
                switchToHorizontal();
            }
            setTimeout(function () {
                $exitFullViewBar.addClass('semiopacity');
            }, 5000);
            $('body').addClass('fullSiteWidth');
            $('#topInfo,#header,#header,#hide1,#hide2,#footer,.bottomContent').hide();
//            $slidesContainer.removeAttr('style');
            if (!inDemo) {
                $rightPane.css('width', '61.4%');
                $leftPane.css('width', '37.1%');
            } else {
                $rightPane.css('width', '0');
                $leftPane.css('width', '100%');
                // transform to pixels
                $leftPane.css('width', $leftPane.width());
            }
            // reassign with new box size
            $('#right_text_fix').tinyscrollbar();
            //$('.text_height_fix_box').tinyscrollbar();
            updateComeBackDimensions();
            adjustSlidesVideoAndBars();
            if (inDemo) {
                $('html, body').scrollTop($viewButtonsBar.offset().top);
            }
            if (inHorizontal) {
                $('#summary_height_fix_box').css('width', '97%');
                $('#summary_height_fix_box .viewport').css('width', '100%');
                $('#summary_height_fix_box .viewport').css('position', 'static');
                $('#summary_height_fix_box .viewport .overview').css('position', 'static');
                $('.scrollbar').hide();
            } else {
                $('#right_text_fix').css('width', '97%');
                $('#right_text_fix .viewport').css('width', '100%');
                $('#right_text_fix .viewport').css('position', 'static');
                $('#right_text_fix .viewport .overview').css('position', 'static');
            }
        },
// FH > H
// FV > H
        fullToNonFull = function () {
            var newVideoWidth,
                newVideoHeight;
            inFullScreen = false;
            if (!inHorizontal) {
                switchToHorizontal();
            }
            $exitFullViewBar.removeClass('semiopacity');
            $('body').removeClass('fullSiteWidth');
            $('#topInfo,#header,#hide1,#hide2,#footer,.bottomContent').show();
            if (!inDemo) {
                // reset panes
                $rightPane.css('width', '595px');
                $leftPane.css('width', '360px');
                // reset slides
                $slidesContainer.removeAttr('style');
                // reset video
                $playerContainer.removeAttr('style');
                // reset download bar
                $downloadBar.removeAttr('style');
                // reset view buttons bar
                $viewButtonsBar.removeAttr('style');
            } else {
                $leftPane.css('width', '100%');
                newVideoWidth = $leftPane.width();
                newVideoHeight = getVideoHeightByWidth(newVideoWidth);
                if (newVideoHeight > $(window).height()) {
                    newVideoHeight = $(window).height() - 40; // 40 is the height of the button bar, which stays desplayed
                    newVideoWidth = getVideoWidthByHeight(newVideoHeight);
                }
                $playerContainer.css('width', newVideoWidth).css('height', newVideoHeight);
                if ($ccVideoPlayer) {
                    $ccVideoPlayer.attr('width', newVideoWidth).attr('height', newVideoHeight); // for the chinese player only
                }
                $downloadBar.width($playerContainer.width() - 12);
                $viewButtonsBar.width($playerContainer.width() - 20);

            }
            updateComeBackDimensions();
            adjustSlidesVideoAndBars();
            if (inDemo) {
                $('html, body').scrollTop($viewButtonsBar.offset().top);
            }

            if(inHorizontal) {
                $('.summary_height_fix_box').tinyscrollbar();
                $('#summary_height_fix_box').css('width', '360px');
                $('#summary_height_fix_box .viewport').css('position', 'relative');
                $('#summary_height_fix_box .viewport').css('width', '340px');
                $('#summary_height_fix_box .viewport .overview').css('position', 'absolute');
                $('.scrollbar').show();
            }
        },
// note that there are no possible transitions from some non-full view to FV directly or from some full view to V directly
// Nominal dimensions (what you get when in horizontal non-full view
        nominalVideoWidth = 360,
        nominalVideoHeight = 203, // by default set for 16:9 (widescreen) videos
// the video player contains a bar containing the controls. The dimensions of that bar is different for different players
        extraVideoHeight = 25, // by default set for the InfoQ player
        nominalSlidesWidth = 585,
        nominalSlidesHeight = 446,
// - Special treatment for special players
        isCCVideoPlayer = $('div.video #player div[id^=cc_video_]').length > 0,
        isHTML5Player = $('video#video').length > 0;

// START PROCESSING
    // for the CC Video player
    $(window).on('player__ready', function() {
        if (isCCVideoPlayer) {
            $ccVideoPlayer = $('div.video #player div[id^=cc_video_] object[id^=cc_] embed');
            adjustSlidesVideoAndBars();
        }
    });

    // for 4:3 videos the nominal width needs to be recompunted
    if (!isWideScreen) {
        nominalVideoHeight = 270;
    }

    // the cc video player has a control bar of 12 px, the html5 players conveniently display the bar on top of the content so no extra height is necessary
    if (isCCVideoPlayer) {
        extraVideoHeight = 12;
    } else if (isHTML5Player) {
        extraVideoHeight = 0;
    }

    // by default, the video container is set for widescreen videos and the InfoQ player. In case that's not the case, stuff needs to be redimensioned
    $playerContainer.height(nominalVideoHeight + extraVideoHeight);

    // enable tipsy for the horizontal/vertical/full screen buttons
    $('a[rel=tipsy]').tipsy({fade: true, gravity: 's'});
    // enable the scrollbars for the summary and the bio/conference texts
    if (!inFullScreen) {
        $('.summary_height_fix_box').tinyscrollbar();
    }
//    $('.text_height_fix_box').tinyscrollbar();

    // enable window resizing
    $(window).resize(function () {
        adjustSlidesVideoAndBars();
    });

    // activate button actions
    $toHorizontalBtn.click(function() {
        if (!animationInProgress) {
            switchToHorizontal();
        }
    });

    $toVerticalBtn.click(function() {
        if (!animationInProgress) {
            switchToVertical();
        }
    });
    $exitFullViewBar.click(function() {
        if (!animationInProgress) {
            fullToNonFull();
        }
    });
    $toAndFromFullBtn.click(function() {
        if (!animationInProgress) {
            inFullScreen ? fullToNonFull() : nonFullToFull();
        }
    });

    function updateComeBackDimensions() {
        if (!inFullScreen) {
            comeBackVideoWidth = nominalVideoWidth;
            comeBackVideoHeight = nominalVideoHeight + extraVideoHeight;
            comeBackSlideWidth = nominalSlidesWidth;
            comeBackSlideHeight = nominalSlidesHeight;
        } else {

            comeBackVideoWidth = Math.floor(0.371 * $rootPane.width()); // the left pane (the one containing the video player) takes 37.1% of the parent's width
            comeBackVideoHeight = getVideoHeightByWidth(comeBackVideoWidth);

            comeBackSlideWidth = Math.floor(0.614 * $rootPane.width()); // the right pane (the one containing the slides) takes 61.4% of the parent's width
            comeBackSlideHeight = Math.floor(comeBackSlideWidth * nominalSlidesHeight / nominalSlidesWidth);
        }
    }

    function updateButtonStates() {
        if (inHorizontal) {
            $toHorizontalBtn.addClass('current');
            $toVerticalBtn.removeClass('current');
        } else {
            $toVerticalBtn.addClass('current');
            $toHorizontalBtn.removeClass('current');
        }

        if (inFullScreen) {
            $toAndFromFullBtn
                .addClass('current')
                .addClass('minimize')
                .attr('original-title', i18nMinimize);
        } else {
            $toAndFromFullBtn
                .removeClass('current')
                .removeClass('minimize')
                .attr('original-title', i18nFullScreen)
                .bind('hover', function () { return false; });
        }
    }

    function placeBannerSummaryBioAndConfDesc() {
        if (inHorizontal) {
            $('#id_300x250_banner_top').remove(); // the banner will be loaded again

            $leftBottomTextCont.append($summary).append($authorBio).append($confDesc);
            $leftPane.append('<div id="id_300x250_banner_top" data-place="TOP"><script type="text/javascript">googletag.cmd.push(function() { googletag.display("id_300x250_banner_top"); });</script></div>');

            $('#vcrOptional1').remove();
            $leftPane.append('<div id="vcrOptional1"/>');
            $('#vcrOptional1').replaceWith($vcrNativeTop.clone());

            // append in 'text_height_fix_box' - box with fix height
            $rightPane.append($slidesContainer).append($presentationNotes);
            $presentationNotes.css('width','auto');
        } else {
            // move the summary and conference texts in the right
            $rightTopTextCont.append($summary).append($authorBio).append($confDesc);
            $('.right_text_fix').tinyscrollbar();

            // first slides
            $bottomPane.append($slidesContainer);

            $('#vcrOptional2').replaceWith($vcrNativeTop.clone().attr("id","vcrOptional2"));
            $('#vcrOptional1').css("display", "none");

            // then banner
            if (!inFullScreen) {
                $('#id_300x250_banner_top').remove(); // the banner will be reloaded again
                $('#vcrOptional1').remove();
                $bottomPane.append('<div id="id_300x250_banner_top" data-place="TOP"><script type="text/javascript">googletag.cmd.push(function() { googletag.display("id_300x250_banner_top"); });</script></div>');
                $bottomPane.append('<div id="vcrOptional1"/>');
                $('#vcrOptional1').replaceWith($vcrNativeTop.clone());
            }

            // last biolabel and biotext
            $bottomPane.append($presentationNotes);

            if (inFullScreen) {
                $authorBio.addClass('in_full_screen');
                $presentationNotes.css('width','auto');
            } else {
                $authorBio.removeClass('in_full_screen');
                $presentationNotes.css('width','650px');
            }
        }
        if (!inFullScreen) {
            $('.summary_height_fix_box').tinyscrollbar();
            $('#right_text_fix').css('width', '595px');
            $('#right_text_fix .viewport').css('height', '265px');
            $('#right_text_fix .viewport').css('position', 'relative');
            $('#right_text_fix .viewport').css('width', '575px');
            $('#right_text_fix .viewport .overview').css('position', 'absolute');
            $('.scrollbar').show();
        } else {
            $('#right_text_fix').css('width', '97%');
            $('#right_text_fix .viewport').css('width', '100%');
            $('#right_text_fix .viewport').css('height', '100%');
            $('#right_text_fix .viewport').css('position', 'static');
            $('#right_text_fix .viewport .overview').css('position', 'static');
        }
//        $('.text_height_fix_box').tinyscrollbar();
    }



    var deleteVideoPosterAttributeForIOSLowerThan4 = function () {
        // ios lower then 4 does not play video if both poster and source attributes are used.
        var ua = navigator.userAgent,
            uaindex,
            userOS,
            userOSver;
        // determine OS
        if (ua.match(/iPad/i) || ua.match(/iPhone/i)) {
            userOS = 'iOS';
            uaindex = ua.indexOf('OS ');
            if (uaindex !== -1) {
                userOSver = ua.substr(uaindex + 3, 3).replace('_', '.');
                if (userOSver < 4) {
                    $('#video').removeAttr("poster");
                }
            }
        }
    };


    function adjustSlidesVideoAndBars() {
        var newVideoHeight,
            newVideoWidth,
            newSlideHeight,
        // the slides container width depends on the containing element's width
            $parent = $rightPane; // default is the horizontal view

        var contentType = determineSlideContent();
        if(contentType !== undefined && contentType == "swf"){
            if (!inDemo || !inHorizontal) { // don't bother if it's in horizontal and demo mode
                if (!inHorizontal) { // in vertical view the containing element is the bottom pane
                    $parent = $bottomPane;
                }
                newSlideHeight = Math.floor($parent.width() * nominalSlidesHeight / nominalSlidesWidth);
                $slidesContainer.css('height', newSlideHeight);
            }
        }
        // video 
        newVideoWidth = $leftPane.width();
        newVideoHeight = getVideoHeightByWidth(newVideoWidth);
        // make sure the entire area of the video fits inside the window
        if (newVideoHeight > $(window).height()) {
            newVideoHeight = $(window).height() - 40; // 40 is the height of the button bar, which stays desplayed
            newVideoWidth = getVideoWidthByHeight(newVideoHeight);
        }
        $playerContainer.css('width', newVideoWidth).css('height', newVideoHeight);
        if ($ccVideoPlayer) {
            $ccVideoPlayer.attr('width', newVideoWidth).attr('height', newVideoHeight); // for the chinese player only
        }

        // dowload bar
        $downloadBar.width($playerContainer.width() - 12);

        // view buttons bar
        $viewButtonsBar.width($playerContainer.width() - 20);
    }

//    differentiates between two slide types: img or swf. in case the slides are images then we don't need to 
//    apply height to the parent, else we need to, otherwise the swfobject container doesn't have a size. 
//    TODO might be worthwhile to add more logic based on aspect ratios 
    function determineSlideContent(){
        if($('#slideContainer > #slide:has(img)').length > 0){ //img
            return "img";
        } else if($('#slideContainer > #slide').is('object')){ //if swfobject
            return "swf";
        } else {
            return undefined;
        }
    }

    deleteVideoPosterAttributeForIOSLowerThan4();

    $(window).on('player__ready', function() {
        if (isCCVideoPlayer) {
            $ccVideoPlayer = $('div.video #player div[id^=cc_video_] object[id^=cc_] embed');
            adjustSlidesVideoAndBars();
        }
    })
    // only bother if there are demos
    if (demoTimings) {
        var demoSwitches = demoTimings.split(',');
        for (var i = 0; i < demoSwitches.length; i++) {
            demoSwitches[i] = parseInt(demoSwitches[i]) * 1000; // translate to milliseconds
        }

        $(window).on('player__playing', function(evt, extra) {
            var idx = getLastSwitchIdx(extra.position_in_ms);
            if (idx % 2 === 0) {
                if (!inDemo) {
                    inDemo = true;
                    demoOn();
                }
            } else {
                if (inDemo) {
                    inDemo = false;
                    demoOff();
                }
            }
            return false; // for now the only consumer of the event.
        });
    }

    function getLastSwitchIdx(ms) {
        for (var i = 0; i < demoSwitches.length; i++) {
            var offset = 0; // transitioning from presentation to demo has to be started 2s before the demo starts, 
            //as it takes 2s for the animation to complete. At the moment it is completed, the demo should already be max size
            if (i % 2 === 0) {
                offset = transitionTime;
            }
            if (ms + offset <= demoSwitches[i]) {
                return i - 1;
            }
            if (i === demoSwitches.length) {
                return i;
            }
        }
    }

    function demoOn() {
        animationInProgress = true;
        var newWidth = $rootPane.width(),
            newHeight = getVideoHeightByWidth(newWidth);

        // make sure the entire area of the video fits inside the window
        if (newHeight > $(window).height()) {
            newHeight = $(window).height() - 40; // 40 is the height of the button bar, which stays desplayed
            newWidth = getVideoWidthByHeight(newHeight);
        }
        if ($viewButtonsBar.offset().top > $('body').scrollTop() && newHeight + $viewButtonsBar.offset().top - $('body').scrollTop() > $(window).height()) {
            $('html, body').animate({
                scrollTop: $viewButtonsBar.offset().top
            }, transitionTime);
        }
        $rightPane.animate({width :'0px', opacity : '0'}, transitionTime - 200, function() {$(this).hide()});
        $leftPane.animate({width : '100%'}, transitionTime);
        $playerContainer.animate({width : newWidth, height : newHeight}, transitionTime, function() {
            animationInProgress = false;
            if (!$ccVideoPlayer) {
                $('#player').attr('width', '100%').attr('height', '100%');
            }
        });
        $downloadBar.animate({width : newWidth - 12}, transitionTime);
        $viewButtonsBar.animate({width : newWidth - 20}, transitionTime);
        // resize the flash object without animation
        $('#player').attr('width', newWidth).attr('height', newHeight);
        updateComeBackDimensions();
    }

    function demoOff() {
        animationInProgress = true;
        // resize the flash object without animation
        $('#player').attr('width', comeBackVideoWidth).attr('height', comeBackVideoHeight);
        $viewButtonsBar.animate({width: comeBackVideoWidth - 20}, transitionTime);
        $downloadBar.animate({width: comeBackVideoWidth - 12}, transitionTime);
        $playerContainer.animate({width: comeBackVideoWidth, height: comeBackVideoHeight}, transitionTime, function() {
            animationInProgress = false;
            if (!$ccVideoPlayer) {
                $('#player').attr('width', '100%').attr('height', '100%');
            }
            $rightTopTextCont.parent().parent().tinyscrollbar();
        });

        $leftPane.animate({width: '37.1%'}, transitionTime);
        var contentType = determineSlideContent();
        if(contentType !== undefined && contentType == 'swf'){
            if (inHorizontal) {
                $slidesContainer.height(comeBackSlideHeight); // not part of the animation, make sure it's done when the animation starts
            }
        }
        $rightPane.animate({width: '61.4%', opacity: '1'}, transitionTime + 100);
        // show the right pane, however after a slight delay, otherwise it is displayed under the video
        var delayId = setInterval(function() {
            $rightPane.show();
            clearInterval(delayId);
        }, 300);
    }

    function getVideoHeightByWidth(newWidth) {
        return extraVideoHeight - 1 + Math.floor(newWidth * nominalVideoHeight / nominalVideoWidth);
    }

    function getVideoWidthByHeight(newHeight) {
        return nominalVideoWidth * (newHeight - extraVideoHeight) / nominalVideoHeight;
    }
});

// for downloading mp3 and pdf
$(function () {
    $('#mp3').click(function () {
        if (!loggedIn) {
            UserActions_Login.showLoginWidget($(this).parent(), 'downloadPresentationMp3');
        } else {
            UserActions_Profile.forceUpdateProfile(PresentationDownloads.loadMp3);
        }
    });
    $('#slides').click(function () {
        if (!loggedIn) {
            UserActions_Login.showLoginWidget($(this).parent(), 'downloadPdf');
        } else {
            UserActions_Profile.forceUpdateProfile(PresentationDownloads.loadPdf);
        }
    });
});

var PresentationDownloads = {
    loadMp3: function() {
        // we might have a force update popup displyed here, close it first
        $.colorbox.close();
        $('#mp3Form').submit();
    },
    loadPdf: function() {
        // we might have a force update popup displyed here, close it first
        $.colorbox.close();
        $('#pdfForm').submit();
    }
};