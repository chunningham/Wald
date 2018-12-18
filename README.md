# Wald
a holochain-based reddit-like DAG-forum thing

A Wald (german for forest) is a collection of trees:

* Each Tree consists of a root, parentless comment (or subject) with comment children (replies).
* A comment may be a reply to multiple other comments.
* Any comment can have replies.
* New roots can be created.
* Comments can reply to comments from multiple trees, linking them.
* Comments are immutable, apart from deletion.
* As comments are immutable, they can only reply to comments older than themselves, creating a Directed Acyclic Graph (DAGs, so Hot right now).
* Users can vote on comments (once per user per comment, only up or down).
* How the Wald appears depends on view settings which change the representation of the tree via the order of comment children.
