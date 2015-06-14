// Instantiate the Bloodhound suggestion engine
var snippets = new Bloodhound({
    datumTokenizer: Bloodhound.tokenizers.obj.whitespace("text"),
    queryTokenizer: Bloodhound.tokenizers.whitespace,
    remote: {
        url: '/search?q=%QUERY',
        wildcard: '%QUERY',
        rateLimitWait: 0,
    }
});

snippets.initialize();

// Instantiate the Typeahead UI
$('#remote .typeahead').typeahead({
    hint: false, 
    highlight: false
}, {
    display: function(obj) { return obj.doc.content; },
    source: snippets.ttAdapter(),
    templates: {
        suggestion: function (obj) { 
            var begin_idx = 0;
            var parts = "<div>";
            obj.highlights.forEach(function (indices) {
                var begin = indices[0];
                var end = indices[1];
                parts += obj.doc.content.slice(begin_idx, begin);
                parts += "<span class=highlight>";
                parts += obj.doc.content.slice(begin, end);
                parts += "</span>";
                begin_idx = end;
            });
            parts += obj.doc.content.slice(begin_idx);
            parts += "</div>";
            return parts;
        }
    }
});
