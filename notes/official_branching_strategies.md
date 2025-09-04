gitflow
https://nvie.com/posts/a-successful-git-branching-model/
https://nvie.com/files/Git-branching-model.pdf

github-flow
https://docs.github.com/en/get-started/using-github/github-flow

gitlab-flow
kinda weird and seems like fluff, really
https://about.gitlab.com/blog/gitlab-flow-duo/
https://about.gitlab.com/topics/version-control/what-are-gitlab-flow-best-practices/
https://about.gitlab.com/blog/what-is-gitlab-flow/
https://about.gitlab.com/topics/version-control/what-are-gitlab-flow-best-practices/

Ok so basically we have
---

* workspace management
* issue tracking
* branch tracing (partially related to issue tracking, needs someone else's code to track)
* pr management

Why not just build that in?

```
gat set-style gitflow
gat set-style github-flow
gat set-style gitlab-flow (though honestly this is just dumb and I'm not doing it)
```

Almost all open source now works on github style

Gitflow - as described, limit usage to that, while allowing for
editing tools locally to change basically everything about history from the branch point
(*without* having to force-push, which jetbrains is very good about.)

Existing solutions:
[lazygit](https://github.com/jesseduffield/lazygit)

guis:
[tig](https://github.com/jonas/tig)
[gitui](https://github.com/gitui-org/gitui)
[git-brunch](https://github.com/andys8/git-brunch)
(neither of which actually change the workflow to something normal)

very interesting: [GitUp](https://github.com/git-up/GitUp)

All these are basically guis though except for GitUp, not significant changes to flow

ok, so practically, the flow:

```
gat branch --hotfix                                   // *only* branches from master, which will be from the last tag
gat branch --feature dont-be-so-slow                  // *only* branches from dev
gat branch --release                                  // *only* branches from dev, merges to dev incrementally with bugfixes
                                                      // implicitly there is a code freeze on dev until this happens,
                                                      // and merges to dev will automatically be rejected until the 
                                                      // release is done, or removed
                                                      // and then finally, to master at a tag, and dev
```
Which, would automatically create an upstream to push to if one is specified.

If working in git-flow
![image](https://nvie.com/img/git-model@2x.png)

Horrible is the rebase cycle, which forces you to solve conflicts over and over again, instead of at the end of the cycle.
Just stash, compare at the end.

* develop is shortened to dev obviously
* master is called main because pc and people hate masters I guess, while working at corporations, or just master
* hotfixes
* release branches - go to specific tag only.
```
gat release create 0.1.4-rc.1
gat release dev-push // merge to dev as we go
// only bugfixes
gat release finalize -> creates "tag" in main/master
```

hotfixes
```
gat hotfix create 0.2
gat hotfix finalize
```
How hard is that? No needing to remember to merge to master, and to dev - just one command that follows the workflow.

You still get to cherry-pick, etc - but the workflow is normalized and controlled.

