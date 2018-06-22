function setCopyRightYear() {
    var currentTime = new Date()
    var year = currentTime.getFullYear()

    var elm = document.getElementById('copyright_year');
    if (elm != null) {
        elm.innerHTML = year;
    }
}

/* EMPTY BLOCKS REMOVAL FUNCTION     */
/* 2013-10-22 Fixed by Vincent Paroz */
/* 1.2.20016 kazu has moved from itu-functions.js */
function removeEmptyBlocks() {
    $('div[id$="_label"]').remove();
    $('div[id$="_RichHtmlField"]').filter(function () {
        return ($.trim($(this).text()) === '' || $(this).text().replace(/^[\s\xA0]+/, '').replace(/[\s\xA0]+$/, '') === '');
    }).remove();

    $('._invisibleIfEmpty').filter(function () {
        return ($.trim($(this).text()) === '' || $(this).text().replace(/^[\s\xA0]+/, '').replace(/[\s\xA0]+$/, '') === '');
    }).remove();
};

$(document).ready(function () {
    var text = "Maintenance Alert!  Due to maintenance, you may experience some intermittent issues on www.itu.int";
    $('#status_preview_body').html(text);

    // remove the empty blocks after page load:
    /* 1.2.20016 kazu has moved from itu-functions.js */
    removeEmptyBlocks();

});

