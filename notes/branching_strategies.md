https://www.abtasty.com/blog/git-branching-strategies/

One thing I noticed working on a chromium fork and a large repo with a group of developers -
It is quite difficult to work without a standardized flow, and local tooling does not match the remote -

That is, that git is not intuitive for a given flow.

With time you become a "master" - however, there are still probably mistakes

An ideal situation would be management of each branching strategy, specifying for a repo, 
and then having all automation tools match that.

I personally like the github flow, having worked mostly in it - however there is an additional flow, the jetbrains flow,
which is not talked about here.

In the jetbrains flow, you work locally with a number of branches, like in normal git, however, when you come to update
your branch, it automatically stashes your local changes, notifies you of conflicts, and requires that you resolve them.
Then it applies changes seamlessly, allowing you to change branches without a lot of issues.

Essentially this is automation of the git commands, but done in a clever way.

to cover:
-
* git subtree
* git submodules

Other issues I came across while working in general with git were the subtree/submodule design.

Git subtree was a decent solution but a tad clunky - you are forced to work locally with a main repo, which magically
has a managed subdirectory which is not really your current branch. This is confusing - but works well for short times.
The jetbrains tooling knows how to work with it well enough.

Git submodules, on the other hand, are very, very clunky. I managed a repo with several subrepos, each having its own head.
While theoretically you just need to update the head, it is impossible to checkout a branch for all subrepos and work
in tandem, not to mention the normal way of working now with tags - which would be ideal for a subrepo so long as it is
not modified - but the second you do, you need to basically move all the repos together very often. Sometimes, you need
to move one subrepo to a specific branch for a bug fix, but often, you're developing everything together.

I personally also like to work with many directories locally, grouping them together. While my IDE does this, I don't
really want to use an IDE. I'm perfectly fine using the simplest text editor for some tasks, or a lighter IDE.

You shouldn't have to download a custom tool just to work normally on a repository, or a set of repositories.

My major criticism of git, unrelated to branching strategies, however, comes from working on a custom chrome. Chromium
took the largest space on my disk, and of that, .git took the lion's share! Upon investigation and watching Linus' talk
from many years ago to Google about git, I found that chromium simply has reached the limits of git. Every file is
stored every single time as a blob for every tiny change. This becomes incredibly massive over time. Yes, it is fast,
yes, in general git is good enough. In my experience however, when developers become used to something, it starts to
take on the form of a kind of religion. We developers are not entirely rational beings.

As Linus himself said, git is a very simple solution, really very simple - it does it's job very well, provides
distributed development, and allows for syncing.

But there's a lot of wasted space. All with the now probably less valid excuse of "your disk will fail you".

We have databases in prod that do atomic diff operations with logs, we have entire filesystems that do log based creation - 
so why not our repos, if your workflow matches that?

For github-style flows as mentioned above, the development is really very simple:
1. Open a new branch
2. Periodically merge from main/master, solving conflicts
3. Eventually merge to main/master

This is not so dissimilar to trunk-style development.

Honestly, the distributed flow makes quite a mess. It makes sense for everyone to have a play-space, and I'm all for that.
I'm for everyone having the ability to define what is main/master by them - but at the end of the day, the code needs to
come to consensus. These are the "tags".

I see the main difference between github-style and gitflow-style being the subbranches that can be opened - tiered branching
is a very powerful concept.

Also politically, the gitflow-style creates problems. Who is in charge? Everyone. That may be fine for open source repos,
but for organizations working to make production code, it's difficult. You don't need a strict gatekeeper, but you need at
least something to prevent bad code from coming in.

Developers seem to like having code style enforced in the whole repo, bad code usages being flagged, etc. There are a lot
of tools that quickly become bloated and difficult to use, especially in the typescript development environment. I wouldn't
wish typescript on anyone.

Practically nowadays, almost everyone works in github-style flows as they are very easy to manage. Each person has their own
page, fork, of the repo, and then things can be merged back. If you only have one fork, then you manage branches basically
as pull request summaries of what changed, and then merge that, usually as a squash, back into the main repo.

Easy to review, easy to fix, easy to do.

I mean yes there are times you need to cherry-pick a file or set of files from a particular branch, or revert a specific file back
to a specific commit. That's cool, but making a mess of the history? Horrible to look at, drives you crazy to reason about it.

You might as well be looking at IDA or ghidra trying to reverse the flow of some binary. Why make such a mess? KISS.

Real simplicity is not just in the file system, but in delivering a product with all of the complexities and components
that come with it - as well as having a knowledge based hierarchy to prevent issues - or at least having the checks in place,
like Google or most companies do, that require at least two reviewers to look at something before it reaches prod. That's not
a terrible compromise, given that the two reviewers actually understand the code.

Informally most repos have core maintainers which essentially are these gatekeepers, and they are essential for maintaining
the quality of the code. That, or testing everything, but even that can lie. How many times has some random chinese dude
checked in a CVE into a core dependency to a stack? You need trust, like Linus said. Trust is earned.

That said, anyway the flexibility of git should be maintained for fixing things, while maintaining a normal, reasonable,
flow of the repo and allowing local and remote branching, as well as forking.