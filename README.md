# youtrack-with-gitlab-integration

Integration for youtrack with gitlab

### Integration steps

1. Gitlab:
    - Create web hook on **Merge request events** with **secret token** and chosen **url**
    - rr
2. Youtrack:
    - create ne workflow with content from the [file](workflow/index.js)

### Dev running steps

- cargo-watch

### What is done

- Youtrack sdk
    - [X] Issue reading
    - [X] Issue status changing
    - [ ] Comment adding with link to the gitlab
    - [ ] Endpoint on status changing
- Gitlab:
    - [ ] Checking of merge requests creation
    - [ ] Checking merging of tasks
    - [ ] Endpoint for webhooks events (pipeline finishing, merge, comments)
- Common:
    - [ ] On Merging should check. If pipline not finished:
        - [ ] set label "Pipeline in progress"
        - [ ] remove label when pipeline will be done
    - [ ] If was added comment to the merge request, then task should be labeled with "review comments"