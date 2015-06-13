// Instantiate the Bloodhound suggestion engine
var snippets = new Bloodhound({
    datumTokenizer: Bloodhound.tokenizers.obj.whitespace("text"),
    queryTokenizer: Bloodhound.tokenizers.whitespace,
    remote: {
        url: '/search?q=%QUERY',
        wildcard: '%QUERY',
        rateLimitWait: 0,
    },
    transform: function(snippets) {
        return $.map(snippets, function(snippet) { 
            return { 
                content: snippet.doc.content,
                html: snippet.highlighted,
            }; 
        });
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
            return "<div>" + obj.highlighted + "</div>"; 
        }
    }
});
