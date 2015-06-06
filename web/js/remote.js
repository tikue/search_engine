// Instantiate the Bloodhound suggestion engine
var snippets = new Bloodhound({
    datumTokenizer: Bloodhound.tokenizers.obj.whitespace("text"),
    queryTokenizer: Bloodhound.tokenizers.whitespace,
    remote: {
        url: 'http://localhost:3000/search?q=%QUERY',
        wildcard: '%QUERY',
        rateLimitWait: 0,
    },
    transform: function(snippets) {
        return $.map(snippets, function(snippet) { 
            return { 
                id: snippet.id,
                content: snippet.content,
            }; 
        });
    }
});

snippets.initialize();

// Instantiate the Typeahead UI
$('#remote .typeahead').typeahead({
    hint: false, 
    highlight: true,
    classNames: {
        highlight: 'highlight'
    }
}, {
    display: 'content',
    source: snippets.ttAdapter(),
    templates: {
        suggestion: function (obj) { return "<div>" + obj.html + "</div>"; }
    }
});
