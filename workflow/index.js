// Thanks to Michael Rush for the original version of this rule (https://software-development.dfstudio.com/youtracks-new-javascript-workflows-make-slack-integration-a-breeze-d3275605d565)

var entities = require('@jetbrains/youtrack-scripting-api/entities');
var http = require('@jetbrains/youtrack-scripting-api/http');
var workflow = require('@jetbrains/youtrack-scripting-api/workflow');

exports.rule = entities.Issue.onChange({
    title: workflow.i18n('Call service when status was changed'),
    guard: function(ctx) {
        var issue = ctx.issue;
        return issue.fields.isChanged(ctx.State) && issue.fields.State && !issue.fields.State.isResolved;
    },
    action: function(ctx) {
        var issue = ctx.issue;
        var oldValue = issue.fields.oldValue(ctx.State);

        console.log(issue.fields.State.name);
        console.log(oldValue.name);

        var connection = new http.Connection("https://fecda0bb920bb5b77d1d34e59c5d0229.m.pipedream.net", null, 2000);
        var response = connection.postSync('', null, JSON.stringify(ctx));
        if (!response.isSuccess) {
            console.warn('Failed to post notification to Slack. Details: ' + response.toString());
        }
    },
    requirements: {
        State: {
            type: entities.State.fieldType
        }
    }
});


