'use strict';

(function (config) {
  var Console = window.console;
  document.addEventListener('DOMContentLoaded', function () {
    var id = 'presentationNotes';
    var notes = document.getElementById(id);
    if (!notes) {
      Console.error('Expandables script loaded but no expandable content found.');
    }
    var showMore = document.createElement('BUTTON');
    showMore.textContent = 'Show More';
    showMore.classList.add('expand-to-full');
    var h2s = notes.querySelectorAll('h2');
    function puneInDiv(vector, index) {
      var wrapper = document.createElement('DIV');
      wrapper.classList.add('notesWrapper');
      var _iteratorNormalCompletion = true;
      var _didIteratorError = false;
      var _iteratorError = undefined;

      try {
        for (var _iterator = vector[Symbol.iterator](), _step; !(_iteratorNormalCompletion = (_step = _iterator.next()).done); _iteratorNormalCompletion = true) {
          var nod = _step.value;

          wrapper.append(nod);
        }
      } catch (err) {
        _didIteratorError = true;
        _iteratorError = err;
      } finally {
        try {
          if (!_iteratorNormalCompletion && _iterator.return) {
            _iterator.return();
          }
        } finally {
          if (_didIteratorError) {
            throw _iteratorError;
          }
        }
      }

      notes.insertBefore(wrapper, h2s[index]);
    }
    if (config.wrap) {
      var child = notes.firstChild;

      var arr = [];
      var i = 0;

      while (child) {
        if (child.nodeName === 'H2') {
          if (arr.length) {
            puneInDiv(arr, i);
          }
          arr = [];
          i++;
        } else {
          arr.push(child);
          if (child === notes.lastChild) {
            puneInDiv(arr, i);
          }
        }
        child = child.nextSibling;
      }
    }
    notes.append(showMore);

    document.addEventListener('click', function (e) {
      var target = e.target;
      if (target.nodeName === 'H2' && target.parentNode.id === id) {
        target.classList.toggle('expanded');
        var testheader = document.querySelector('.expanded');
        if (!testheader) {
          notes.classList.add('full');
        }
      }
      if (target.classList.contains('expand-to-full')) {
        notes.classList.toggle('full');
        _gaq.push(['_trackEvent', 'ShowNotes-Presentations', "ShowMoreClick-Presentations", "ShowMore"]);
      }
    });
  });
})({
  wrap: true });
