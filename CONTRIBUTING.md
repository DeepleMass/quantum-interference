# Contributing to quantum-interference

Thanks for taking an interest. This repository has a specific didactic purpose, and that purpose shapes how contributions work. Please read this short guide before opening an issue or a PR.

## Two kinds of contribution

**Corrections to the project itself are welcome here.** If a specification is wrong or ambiguous, a test does not actually cover its acceptance criterion, a docstring is misleading, or a Reading pointer leads to the wrong N&C page, open an issue or a PR. Use the issue templates; they encode what we need to know.

**Implementation progress belongs in your fork, not here.** The placeholders are the learning content. Merging implementations into this repository would remove exactly what makes the project work. Please keep your milestones on the learn branch of your fork and show them off in Discussions (Show and tell).

## The learning workflow (for fork)

1. Fork the repository at the [v0.1.0](https://github.com/DeepleMass/quantum-interference/tree/v0.1.0) tag. Your fork's main stays there as an untouched reference.
2. Run `git switch learn`. All work happens on learn.
3. Pick one milestone. One SPEC section is your working prompt; do not feed the whole document.
4. Before implementing: read the failing tests for that milestone, explain why each one fails, and predict the failure. The first milestone exercise is reviewing the test suite against the specs and fixing coverage gaps.
5. Implement until every CI stage is green.
6. Land the milestone as a PR into your own learn branch, one milestone per PR, and write the journal entry.

## Corrections: what we look for

- **Specificity.** Name the file and section. Quote the passage in question.
- **Sources.** If you correct physics or math, cite the source. N&C citations use section and print page of the 10th anniversary edition; papers are cited as in the README references section.
- **Tests before claims.** If you believe a test is wrong, show what it fails to check and propose the corrected assertion. If you add a test, keep it in the style of the existing suite.
- **One concern per PR.** Small, reviewable, and complete on its own.

## Conventions

- Repository language is English: code, comments, docs, commits.
- Conventional Commits (`docs:`, `fix:`, `test:`, ...).
- Documentation changes go through the learn branch, like everything else.
- Rust: edition 2021, `cargo fmt` and `cargo clippy -D warnings` clean.
- Python: 3.10+, fully type-hinted, strict ruff.
- CI must be green before merge. No exceptions.

## Where to ask

Conceptual questions about the material belong in Discussions (Q&A). Feedback on the didactic design belongs in Discussions (General). Bug reports and doc corrections belong in issues. If in doubt, open a Discussion first — it is easier to move a conversation to an issue than the other way around.

## License

By contributing, you agree that your contributions will be licensed under the [MIT license](LICENSE).
